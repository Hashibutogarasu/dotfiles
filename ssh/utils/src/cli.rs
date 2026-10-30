use anyhow::{anyhow, Result};
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ssh-utils",
    version,
    about = "Generate an SSH keypair for system access and print the client setup guide",
    after_help = "Run `ssh-utils guide` for the full client setup guide, including the ~/.ssh/config block to add by hand."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Generate the system_key Ed25519 keypair")]
    Generate(GenerateArgs),
    #[command(about = "Print the Cloudflared SSH client setup guide")]
    Guide(GuideArgs),
    #[command(about = "Generate shell completion scripts")]
    Completions(CompletionsArgs),
}

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    #[arg(value_enum)]
    pub shell: Shell,
}

pub fn print_completions(shell: Shell) {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "ssh-utils", &mut std::io::stdout());
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[arg(
        short,
        long,
        value_name = "DIR",
        help = "Directory where the key files are written (default: ~/.ssh/generated)"
    )]
    pub output_dir: Option<String>,
    #[arg(
        long,
        default_value = "system_key",
        help = "Base name of the key files"
    )]
    pub name: String,
    #[arg(
        long,
        help = "Comment stored in the public key (default: <name>@<hostname>)"
    )]
    pub comment: Option<String>,
    #[arg(long, help = "Overwrite existing key files")]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct GuideArgs {
    #[arg(
        short,
        long,
        value_name = "DIR",
        help = "Directory where the key files are written (default: ~/.ssh/generated)"
    )]
    pub output_dir: Option<String>,
    #[arg(
        long,
        default_value = "system_key",
        help = "Base name of the key files"
    )]
    pub name: String,
    #[arg(
        long,
        default_value = "*.example.com",
        help = "Host pattern used in the printed ~/.ssh/config block"
    )]
    pub host: String,
    #[arg(
        long,
        help = "Remote user name shown in the printed ~/.ssh/config block"
    )]
    pub user: Option<String>,
}

pub fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or_else(|| anyhow!("could not determine the home directory"))
}

pub fn expand_path(raw: &str) -> Result<PathBuf> {
    if raw == "~" {
        return home_dir();
    }
    if let Some(rest) = raw.strip_prefix("~/") {
        return Ok(home_dir()?.join(rest));
    }
    Ok(PathBuf::from(raw))
}

pub fn resolve_output_dir(raw: Option<&str>) -> Result<PathBuf> {
    match raw {
        Some(value) => expand_path(value),
        None => Ok(home_dir()?.join(".ssh").join("generated")),
    }
}

pub fn default_comment(name: &str) -> String {
    match std::env::var("HOSTNAME") {
        Ok(hostname) if !hostname.trim().is_empty() => format!("{}@{}", name, hostname.trim()),
        _ => name.to_string(),
    }
}

pub fn display_path(path: &std::path::Path) -> String {
    if let Ok(home) = home_dir() {
        if let Ok(rest) = path.strip_prefix(&home) {
            if rest.as_os_str().is_empty() {
                return "~".to_string();
            }
            return format!("~/{}", rest.display());
        }
    }
    path.display().to_string()
}
