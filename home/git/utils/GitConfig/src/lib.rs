#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub signing_key: Option<String>,
    #[serde(default)]
    pub use_gh: bool,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SigningInput {
    pub commit: Option<bool>,
    pub format: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryInput {
    pub path: String,
    #[serde(default)]
    pub user: UserInput,
    pub ssh_command: Option<String>,
    #[serde(default)]
    pub use_gh: bool,
}

#[derive(Deserialize, Default)]
pub struct GitInput {
    #[serde(default, rename = "use1Password")]
    pub use_1password: bool,
    #[serde(default, rename = "useGh")]
    pub use_gh: bool,
    #[serde(default)]
    pub user: UserInput,
    #[serde(default)]
    pub signing: SigningInput,
    #[serde(default)]
    pub directories: Vec<DirectoryInput>,
}

pub struct RenderedConfig {
    pub main: String,
    pub directories: Vec<(PathBuf, String)>,
    pub gh_dirs: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct MissingUserError(Vec<&'static str>);

impl fmt::Display for MissingUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "use1Password is enabled but user.{} is missing",
            self.0.join(", user.")
        )
    }
}

impl Error for MissingUserError {}

#[derive(Serialize, Default)]
struct UserSection {
    name: Option<String>,
    email: Option<String>,
    #[serde(rename = "signingKey")]
    signing_key: Option<String>,
}

#[derive(Serialize, Default)]
struct CommitSection {
    gpgsign: Option<bool>,
}

#[derive(Serialize, Default)]
struct GpgSection {
    format: Option<String>,
}

#[derive(Serialize, Default)]
struct CoreSection {
    #[serde(rename = "sshCommand")]
    ssh_command: Option<String>,
}

#[derive(Serialize)]
struct MainConfig {
    user: UserSection,
    core: CoreSection,
    commit: CommitSection,
    gpg: GpgSection,
}

#[derive(Serialize)]
struct DirectoryConfig {
    user: UserSection,
    core: CoreSection,
}

impl GitInput {
    fn validate(&self) -> Result<(), MissingUserError> {
        if !self.use_1password {
            return Ok(());
        }
        let missing: Vec<&'static str> = [
            ("name", &self.user.name),
            ("email", &self.user.email),
            ("signingKey", &self.user.signing_key),
        ]
        .into_iter()
        .filter(|(_, value)| value.as_deref().is_none_or(str::is_empty))
        .map(|(key, _)| key)
        .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(MissingUserError(missing))
        }
    }
}

fn non_empty(value: &Option<String>) -> Option<String> {
    value.clone().filter(|v| !v.is_empty())
}

fn prune_empty(table: &mut toml::Table) {
    for (_, value) in table.iter_mut() {
        if let toml::Value::Table(inner) = value {
            prune_empty(inner);
        }
    }
    table.retain(|_, value| value.as_table().is_none_or(|inner| !inner.is_empty()));
}

fn to_toml<T: Serialize>(config: &T) -> Result<String, Box<dyn Error>> {
    let mut table = toml::Table::try_from(config)?;
    prune_empty(&mut table);
    Ok(toml::to_string(&table)?)
}

// includeIf takes a quoted condition in its header, which TOML cannot express.
fn include_if(gitdir: &str, path: &Path) -> String {
    format!(
        "[includeIf {}]\n\tpath = {}\n",
        quote(&format!("gitdir:{}/", gitdir.trim_end_matches('/'))),
        quote(&path.display().to_string())
    )
}

fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn gh_credential_helper(gh_config_dir: Option<&Path>, reset: bool) -> String {
    let gh = "gh auth git-credential";
    let command = match gh_config_dir {
        Some(dir) => format!(
            "!GH_CONFIG_DIR={} {gh}",
            shell_quote(&dir.display().to_string())
        ),
        None => format!("!{gh}"),
    };
    let mut section = String::from("[credential \"https://github.com\"]\n");
    if reset {
        section.push_str("\thelper =\n");
    }
    section.push_str(&format!("\thelper = {}\n", quote(&command)));
    section
}

fn expand_leading_tilde(value: &str, home: &str) -> String {
    value
        .strip_prefix('~')
        .map_or_else(|| value.to_owned(), |rest| format!("{home}{rest}"))
}

fn is_literal_key(key: &str) -> bool {
    key.starts_with("ssh-") || key.starts_with("key::")
}

fn ssh_command(
    command: Option<String>,
    user: &UserInput,
    home: &str,
    ssh_config: &str,
) -> String {
    let ssh = format!("ssh -F {}", shell_quote(ssh_config));
    match command {
        Some(command) => {
            let command = command.replace('~', home);
            match command.strip_prefix("ssh") {
                Some(args) if args.is_empty() || args.starts_with(' ') => format!("{ssh}{args}"),
                _ => command,
            }
        }
        None => match non_empty(&user.signing_key).filter(|key| !is_literal_key(key)) {
            Some(key) => format!("{ssh} -i {}", shell_quote(&expand_leading_tilde(&key, home))),
            None => ssh,
        },
    }
}

fn directory_conf_path(local_dir: &Path, index: usize) -> PathBuf {
    local_dir.join(format!("{index}.conf"))
}

impl UserSection {
    fn from_input(user: &UserInput, home: &str) -> Self {
        Self {
            name: non_empty(&user.name),
            email: non_empty(&user.email),
            signing_key: non_empty(&user.signing_key).map(|key| expand_leading_tilde(&key, home)),
        }
    }
}

impl MainConfig {
    fn new(input: &GitInput, home: &str, ssh_config: &str) -> Self {
        Self {
            user: UserSection::from_input(&input.user, home),
            core: CoreSection {
                ssh_command: Some(ssh_command(None, &input.user, home, ssh_config)),
            },
            commit: CommitSection {
                gpgsign: input.signing.commit.or(input.use_1password.then_some(true)),
            },
            gpg: GpgSection {
                format: non_empty(&input.signing.format)
                    .or_else(|| input.use_1password.then(|| "ssh".to_owned())),
            },
        }
    }
}

impl DirectoryConfig {
    fn new(dir: &DirectoryInput, home: &str, ssh_config: &str) -> Self {
        Self {
            user: UserSection::from_input(&dir.user, home),
            core: CoreSection {
                ssh_command: Some(ssh_command(
                    non_empty(&dir.ssh_command),
                    &dir.user,
                    home,
                    ssh_config,
                )),
            },
        }
    }
}

pub fn render(
    input: &GitInput,
    home: &str,
    local_dir: &Path,
    ssh_config: &str,
) -> Result<RenderedConfig, Box<dyn Error>> {
    input.validate()?;

    let mut main = to_toml(&MainConfig::new(input, home, ssh_config))?;
    if input.use_gh || input.user.use_gh {
        main.push_str(&gh_credential_helper(None, false));
    }
    let mut directories = Vec::with_capacity(input.directories.len());
    let mut gh_dirs = Vec::new();
    for (i, dir) in input.directories.iter().enumerate() {
        let path = directory_conf_path(local_dir, i);
        let gitdir = expand_leading_tilde(&dir.path, home);
        main.push_str(&include_if(&gitdir, &path));
        let mut content = to_toml(&DirectoryConfig::new(dir, home, ssh_config))?;
        if dir.use_gh || dir.user.use_gh {
            let gh_dir = Path::new(&gitdir).join(".gh");
            content.push_str(&gh_credential_helper(Some(&gh_dir), true));
            gh_dirs.push(gh_dir);
        }
        directories.push((path, content));
    }
    Ok(RenderedConfig { main, directories, gh_dirs })
}
