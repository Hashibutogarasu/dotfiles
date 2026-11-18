mod keys;

use clap::{Parser, Subcommand};
use GitConfig::GitInput;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Git {
        json: PathBuf,
        main_conf: PathBuf,
        local_dir: PathBuf,
        ssh_config: String,
    },
    Keys {
        config: PathBuf,
    },
}

fn load(json_path: &Path) -> Result<GitInput, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(json_path)
        .map_err(|e| format!("failed to read {}: {e}", json_path.display()))?;
    serde_json::from_str(&text)
        .map_err(|e| format!("failed to parse {}: {e}", json_path.display()).into())
}

fn clear_local_dir(local_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(local_dir)?;
    fs::read_dir(local_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "conf" || ext == "pub"))
        .try_for_each(fs::remove_file)
}

fn run(
    json_path: &Path,
    main_path: &Path,
    local_dir: &Path,
    ssh_config: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let input = load(json_path)?;
    let rendered = GitConfig::render(&input, &home, local_dir, ssh_config)?;

    if let Some(parent) = main_path.parent() {
        fs::create_dir_all(parent)?;
    }
    clear_local_dir(local_dir)?;

    for gh_dir in &rendered.gh_dirs {
        fs::create_dir_all(gh_dir)?;
    }
    for (path, content) in rendered.directories.iter().chain(&rendered.key_files) {
        fs::write(path, content)?;
    }
    fs::write(main_path, rendered.main)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Git { json, main_conf, local_dir, ssh_config } => {
            run(&json, &main_conf, &local_dir, &ssh_config)
        }
        Command::Keys { config } => keys::import(&config),
    }
}
