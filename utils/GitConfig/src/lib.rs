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
}

#[derive(Deserialize, Default)]
pub struct GitInput {
    #[serde(default, rename = "use1Password")]
    pub use_1password: bool,
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
    pub ssh: String,
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
struct GpgSshSection {
    program: Option<String>,
}

#[derive(Serialize, Default)]
struct GpgSection {
    format: Option<String>,
    ssh: GpgSshSection,
}

#[derive(Serialize, Default)]
struct CoreSection {
    #[serde(rename = "sshCommand")]
    ssh_command: Option<String>,
}

#[derive(Serialize)]
struct MainConfig {
    user: UserSection,
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

fn expand_leading_tilde(value: &str, home: &str) -> String {
    value
        .strip_prefix('~')
        .map_or_else(|| value.to_owned(), |rest| format!("{home}{rest}"))
}

fn ssh_match_block(gitdir: &str, ssh_command: &str) -> String {
    let mut options = Vec::new();
    let mut tokens = ssh_command.split_whitespace().skip(1);
    while let Some(token) = tokens.next() {
        match (token, tokens.next()) {
            ("-i", Some(file)) => options.push(format!("IdentityFile {file}")),
            ("-o", Some(opt)) => options.push(opt.replacen('=', " ", 1)),
            _ => {}
        }
    }
    let dir = gitdir.trim_end_matches('/');
    let mut block = format!(
        "Match host github.com exec \"case \\\"$PWD/\\\" in {dir}/*) exit 0;; *) exit 1;; esac\"\n"
    );
    for option in options {
        block.push_str(&format!("\t{option}\n"));
    }
    block
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
    fn new(input: &GitInput, home: &str, op_ssh_sign: &str, use_op_ssh_sign: bool) -> Self {
        Self {
            user: UserSection::from_input(&input.user, home),
            commit: CommitSection {
                gpgsign: input.signing.commit.or(use_op_ssh_sign.then_some(true)),
            },
            gpg: GpgSection {
                format: non_empty(&input.signing.format)
                    .or_else(|| use_op_ssh_sign.then(|| "ssh".to_owned())),
                ssh: GpgSshSection {
                    program: use_op_ssh_sign.then(|| op_ssh_sign.to_owned()),
                },
            },
        }
    }
}

impl DirectoryConfig {
    fn new(dir: &DirectoryInput, home: &str) -> Self {
        Self {
            user: UserSection::from_input(&dir.user, home),
            core: CoreSection {
                ssh_command: non_empty(&dir.ssh_command).map(|c| c.replace('~', home)),
            },
        }
    }
}

pub fn render(
    input: &GitInput,
    home: &str,
    local_dir: &Path,
    op_ssh_sign: &str,
) -> Result<RenderedConfig, Box<dyn Error>> {
    input.validate()?;
    let use_op_ssh_sign = input.use_1password;

    let mut main = to_toml(&MainConfig::new(input, home, op_ssh_sign, use_op_ssh_sign))?;
    let mut directories = Vec::with_capacity(input.directories.len());
    let mut ssh = String::new();
    for (i, dir) in input.directories.iter().enumerate() {
        let path = directory_conf_path(local_dir, i);
        let gitdir = expand_leading_tilde(&dir.path, home);
        main.push_str(&include_if(&gitdir, &path));
        directories.push((path, to_toml(&DirectoryConfig::new(dir, home))?));
        if let Some(command) = non_empty(&dir.ssh_command) {
            ssh.push_str(&ssh_match_block(&gitdir, &command.replace('~', home)));
        }
    }
    Ok(RenderedConfig { main, directories, ssh })
}
