#![allow(non_snake_case)]

use serde::Deserialize;
use std::path::{Path, PathBuf};

pub const OP_SSH_SIGN: &str = "/opt/1Password/op-ssh-sign";

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
pub struct GitJson {
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
}

struct UserSection {
    name: Option<String>,
    email: Option<String>,
    signing_key: Option<String>,
}

struct CommitSection {
    gpgsign: Option<bool>,
}

struct GpgSection {
    format: Option<String>,
}

struct GpgSshSection {
    program: Option<String>,
}

struct CoreSection {
    ssh_command: Option<String>,
}

struct IncludeIfSection {
    gitdir: String,
    path: PathBuf,
}

struct MainConfig {
    user: UserSection,
    commit: CommitSection,
    gpg: GpgSection,
    gpg_ssh: GpgSshSection,
    includes: Vec<IncludeIfSection>,
}

struct DirectoryConfig {
    user: UserSection,
    core: CoreSection,
}

trait Section {
    fn header(&self) -> String;
    fn entries(&self) -> Vec<(&'static str, Option<String>)>;

    fn render(&self) -> String {
        let body: String = self
            .entries()
            .into_iter()
            .filter_map(|(key, value)| value.filter(|v| !v.is_empty()).map(|v| (key, v)))
            .map(|(key, value)| format!("\t{key} = {}\n", quote(&value)))
            .collect();
        if body.is_empty() {
            String::new()
        } else {
            format!("[{}]\n{body}", self.header())
        }
    }
}

impl Section for UserSection {
    fn header(&self) -> String {
        "user".into()
    }
    fn entries(&self) -> Vec<(&'static str, Option<String>)> {
        vec![
            ("name", self.name.clone()),
            ("email", self.email.clone()),
            ("signingKey", self.signing_key.clone()),
        ]
    }
}

impl Section for CommitSection {
    fn header(&self) -> String {
        "commit".into()
    }
    fn entries(&self) -> Vec<(&'static str, Option<String>)> {
        vec![("gpgsign", self.gpgsign.map(|v| v.to_string()))]
    }
}

impl Section for GpgSection {
    fn header(&self) -> String {
        "gpg".into()
    }
    fn entries(&self) -> Vec<(&'static str, Option<String>)> {
        vec![("format", self.format.clone())]
    }
}

impl Section for GpgSshSection {
    fn header(&self) -> String {
        format!("gpg {}", quote("ssh"))
    }
    fn entries(&self) -> Vec<(&'static str, Option<String>)> {
        vec![("program", self.program.clone())]
    }
}

impl Section for CoreSection {
    fn header(&self) -> String {
        "core".into()
    }
    fn entries(&self) -> Vec<(&'static str, Option<String>)> {
        vec![("sshCommand", self.ssh_command.clone())]
    }
}

impl Section for IncludeIfSection {
    fn header(&self) -> String {
        format!("includeIf {}", quote(&self.gitdir))
    }
    fn entries(&self) -> Vec<(&'static str, Option<String>)> {
        vec![("path", Some(self.path.display().to_string()))]
    }
}

impl MainConfig {
    fn render(&self) -> String {
        let head: [&dyn Section; 4] = [&self.user, &self.commit, &self.gpg, &self.gpg_ssh];
        head.iter()
            .map(|s| s.render())
            .chain(self.includes.iter().map(Section::render))
            .collect()
    }
}

impl DirectoryConfig {
    fn render(&self) -> String {
        [self.user.render(), self.core.render()].concat()
    }
}

fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn expand_leading_tilde(value: &str, home: &str) -> String {
    value
        .strip_prefix('~')
        .map_or_else(|| value.to_owned(), |rest| format!("{home}{rest}"))
}

fn directory_conf_path(local_dir: &Path, index: usize) -> PathBuf {
    local_dir.join(format!("{index}.conf"))
}

impl UserSection {
    fn from_input(user: &UserInput, home: &str) -> Self {
        Self {
            name: user.name.clone(),
            email: user.email.clone(),
            signing_key: user
                .signing_key
                .as_deref()
                .map(|key| expand_leading_tilde(key, home)),
        }
    }
}

impl IncludeIfSection {
    fn new(dir: &DirectoryInput, index: usize, home: &str, local_dir: &Path) -> Self {
        let path = expand_leading_tilde(&dir.path, home);
        Self {
            gitdir: format!("gitdir:{}/", path.trim_end_matches('/')),
            path: directory_conf_path(local_dir, index),
        }
    }
}

impl MainConfig {
    fn new(json: &GitJson, home: &str, local_dir: &Path, use_op_ssh_sign: bool) -> Self {
        Self {
            user: UserSection::from_input(&json.user, home),
            commit: CommitSection {
                gpgsign: json.signing.commit.or(use_op_ssh_sign.then_some(true)),
            },
            gpg: GpgSection {
                format: json
                    .signing
                    .format
                    .clone()
                    .or_else(|| use_op_ssh_sign.then(|| "ssh".to_owned())),
            },
            gpg_ssh: GpgSshSection {
                program: use_op_ssh_sign.then(|| OP_SSH_SIGN.to_owned()),
            },
            includes: json
                .directories
                .iter()
                .enumerate()
                .map(|(i, dir)| IncludeIfSection::new(dir, i, home, local_dir))
                .collect(),
        }
    }
}

impl DirectoryConfig {
    fn new(dir: &DirectoryInput, home: &str) -> Self {
        Self {
            user: UserSection::from_input(&dir.user, home),
            core: CoreSection {
                ssh_command: dir.ssh_command.as_deref().map(|c| c.replace('~', home)),
            },
        }
    }
}

pub fn render(
    json: &GitJson,
    home: &str,
    local_dir: &Path,
    op_ssh_sign_found: bool,
) -> RenderedConfig {
    let use_op_ssh_sign = json.use_1password && op_ssh_sign_found;
    RenderedConfig {
        main: MainConfig::new(json, home, local_dir, use_op_ssh_sign).render(),
        directories: json
            .directories
            .iter()
            .enumerate()
            .map(|(i, dir)| {
                (
                    directory_conf_path(local_dir, i),
                    DirectoryConfig::new(dir, home).render(),
                )
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json(use_1password: bool) -> GitJson {
        GitJson {
            use_1password,
            user: UserInput {
                name: Some("A".into()),
                email: Some("a@example.com".into()),
                signing_key: Some("~/.ssh/a.pub".into()),
            },
            ..Default::default()
        }
    }

    #[test]
    fn signing_is_configured_when_enabled_and_found() {
        let main = render(&json(true), "/home/u", Path::new("/l"), true).main;
        assert!(main.contains("signingKey = \"/home/u/.ssh/a.pub\""));
        assert!(main.contains("gpgsign = \"true\""));
        assert!(main.contains("format = \"ssh\""));
        assert!(main.contains("[gpg \"ssh\"]"));
        assert!(main.contains(OP_SSH_SIGN));
    }

    #[test]
    fn signing_is_not_configured_otherwise() {
        for (enabled, found) in [(true, false), (false, true)] {
            let main = render(&json(enabled), "/home/u", Path::new("/l"), found).main;
            assert!(!main.contains("gpgsign"));
            assert!(!main.contains(OP_SSH_SIGN));
        }
    }
}
