#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub signing_key: Option<String>,
    #[serde(default)]
    pub ssh_command_args: Vec<String>,
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
    pub user: Option<String>,
    #[serde(default)]
    pub use_gh: bool,
}

#[derive(Deserialize, Default)]
pub struct GitInput {
    #[serde(default, rename = "useGh")]
    pub use_gh: bool,
    #[serde(default, rename = "defaultUser")]
    pub default_user: Option<String>,
    #[serde(default)]
    pub user: BTreeMap<String, UserInput>,
    #[serde(default)]
    pub signing: SigningInput,
    #[serde(default)]
    pub directories: Vec<DirectoryInput>,
}

pub struct RenderedConfig {
    pub main: String,
    pub directories: Vec<(PathBuf, String)>,
    pub gh_dirs: Vec<PathBuf>,
    pub key_files: Vec<(PathBuf, String)>,
}

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
    gpg: GpgSection,
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

fn is_ssh_key(key: &str) -> bool {
    key.starts_with("ssh-")
}

fn key_file_path(local_dir: &Path, name: &str) -> PathBuf {
    local_dir.join(format!("{name}.pub"))
}

fn ssh_command(
    user: Option<(&str, &UserInput)>,
    home: &str,
    ssh_config: &str,
    local_dir: &Path,
) -> String {
    let mut command = format!("ssh -F {}", shell_quote(ssh_config));
    if let Some((name, user)) = user {
        if non_empty(&user.signing_key).is_some_and(|key| is_ssh_key(&key)) {
            let key_file = key_file_path(local_dir, name).display().to_string();
            command.push_str(&format!(
                " -i {} -o IdentitiesOnly=yes",
                shell_quote(&key_file)
            ));
        }
        for arg in &user.ssh_command_args {
            command.push(' ');
            command.push_str(&shell_quote(&expand_leading_tilde(arg, home)));
        }
    }
    command
}

fn directory_conf_path(local_dir: &Path, index: usize) -> PathBuf {
    local_dir.join(format!("{index}.conf"))
}

impl GitInput {
    fn lookup(&self, key: &str) -> Result<(&str, &UserInput), Box<dyn Error>> {
        self.user
            .get_key_value(key)
            .map(|(name, user)| (name.as_str(), user))
            .ok_or_else(|| format!("user \"{key}\" is not defined in \"user\"").into())
    }
}

impl UserSection {
    fn from_input(user: &UserInput) -> Self {
        Self {
            name: non_empty(&user.name),
            email: non_empty(&user.email),
            signing_key: non_empty(&user.signing_key).map(|key| {
                if is_ssh_key(&key) {
                    format!("key::{key}")
                } else {
                    key
                }
            }),
        }
    }
}

impl MainConfig {
    fn new(
        input: &GitInput,
        user: Option<&UserInput>,
        ssh_command: String,
    ) -> Self {
        Self {
            user: user.map_or_else(UserSection::default, UserSection::from_input),
            core: CoreSection {
                ssh_command: Some(ssh_command),
            },
            commit: CommitSection {
                gpgsign: input.signing.commit,
            },
            gpg: GpgSection {
                format: signing_format(user).or_else(|| non_empty(&input.signing.format)),
            },
        }
    }
}

impl DirectoryConfig {
    fn new(user: Option<&UserInput>, ssh_command: String) -> Self {
        Self {
            user: user.map_or_else(UserSection::default, UserSection::from_input),
            core: CoreSection {
                ssh_command: Some(ssh_command),
            },
            gpg: GpgSection {
                format: signing_format(user)
                    .or_else(|| user.and_then(|u| non_empty(&u.signing_key)).map(|_| "openpgp".to_owned())),
            },
        }
    }
}

fn signing_format(user: Option<&UserInput>) -> Option<String> {
    user.and_then(|u| non_empty(&u.signing_key))
        .filter(|key| is_ssh_key(key))
        .map(|_| "ssh".to_owned())
}

pub fn render(
    input: &GitInput,
    home: &str,
    local_dir: &Path,
    ssh_config: &str,
) -> Result<RenderedConfig, Box<dyn Error>> {
    if let Some(name) = input.user.keys().find(|name| name.chars().any(char::is_whitespace)) {
        return Err(format!("user name \"{name}\" must not contain whitespace").into());
    }
    let default_user = input
        .default_user
        .as_deref()
        .map(|name| input.lookup(name))
        .transpose()?;
    let main_ssh = ssh_command(default_user, home, ssh_config, local_dir);
    let mut main = to_toml(&MainConfig::new(
        input,
        default_user.map(|(_, user)| user),
        main_ssh,
    ))?;
    if input.use_gh || default_user.is_some_and(|(_, user)| user.use_gh) {
        main.push_str(&gh_credential_helper(None, false));
    }
    let mut directories = Vec::with_capacity(input.directories.len());
    let mut gh_dirs = Vec::new();
    for (i, dir) in input.directories.iter().enumerate() {
        let path = directory_conf_path(local_dir, i);
        let gitdir = expand_leading_tilde(&dir.path, home);
        main.push_str(&include_if(&gitdir, &path));
        let user = dir
            .user
            .as_deref()
            .map(|name| input.lookup(name))
            .transpose()?;
        let dir_ssh = ssh_command(user, home, ssh_config, local_dir);
        let mut content = to_toml(&DirectoryConfig::new(user.map(|(_, user)| user), dir_ssh))?;
        if dir.use_gh || user.is_some_and(|(_, user)| user.use_gh) {
            let gh_dir = Path::new(&gitdir).join(".gh");
            content.push_str(&gh_credential_helper(Some(&gh_dir), true));
            gh_dirs.push(gh_dir);
        }
        directories.push((path, content));
    }
    let key_files = input
        .user
        .iter()
        .filter_map(|(name, user)| {
            non_empty(&user.signing_key)
                .filter(|key| is_ssh_key(key))
                .map(|key| (key_file_path(local_dir, name), format!("{key}\n")))
        })
        .collect();
    Ok(RenderedConfig { main, directories, gh_dirs, key_files })
}
