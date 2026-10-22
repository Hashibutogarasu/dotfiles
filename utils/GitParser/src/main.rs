use clap::Parser;
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    json: PathBuf,
    main_conf: PathBuf,
    local_dir: PathBuf,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct UserInput {
    name: Option<String>,
    email: Option<String>,
    signing_key: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SigningInput {
    commit: Option<bool>,
    format: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DirectoryInput {
    path: String,
    #[serde(default)]
    user: UserInput,
    ssh_command: Option<String>,
}

#[derive(Deserialize, Default)]
struct GitJson {
    #[serde(default)]
    user: UserInput,
    #[serde(default)]
    signing: SigningInput,
    #[serde(default)]
    directories: Vec<DirectoryInput>,
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
        let head: [&dyn Section; 3] = [&self.user, &self.commit, &self.gpg];
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
            path: local_dir.join(format!("{index}.conf")),
        }
    }
}

impl MainConfig {
    fn new(json: &GitJson, home: &str, local_dir: &Path) -> Self {
        Self {
            user: UserSection::from_input(&json.user, home),
            commit: CommitSection {
                gpgsign: json.signing.commit,
            },
            gpg: GpgSection {
                format: json.signing.format.clone(),
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

fn load(json_path: &Path) -> Result<GitJson, Box<dyn std::error::Error>> {
    match fs::read_to_string(json_path) {
        Ok(text) => Ok(serde_json::from_str(&text)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(GitJson::default()),
        Err(e) => Err(e.into()),
    }
}

fn clear_local_dir(local_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(local_dir)?;
    fs::read_dir(local_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "conf"))
        .try_for_each(fs::remove_file)
}

fn run(
    json_path: &Path,
    main_path: &Path,
    local_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let json = load(json_path)?;

    if let Some(parent) = main_path.parent() {
        fs::create_dir_all(parent)?;
    }
    clear_local_dir(local_dir)?;

    for (i, dir) in json.directories.iter().enumerate() {
        let config = DirectoryConfig::new(dir, &home);
        fs::write(local_dir.join(format!("{i}.conf")), config.render())?;
    }
    fs::write(main_path, MainConfig::new(&json, &home, local_dir).render())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    run(&cli.json, &cli.main_conf, &cli.local_dir)
}
