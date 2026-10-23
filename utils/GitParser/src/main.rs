use clap::Parser;
use GitConfig::GitInput;
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
    op_ssh_sign: String,
}

fn load(json_path: &Path) -> Result<GitInput, Box<dyn std::error::Error>> {
    match fs::read_to_string(json_path) {
        Ok(text) => Ok(serde_json::from_str(&text)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!(
                "warning: {} not found, skipping signing configuration",
                json_path.display()
            );
            Ok(GitInput::default())
        }
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
    op_ssh_sign: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let input = load(json_path)?;
    let rendered = GitConfig::render(&input, &home, local_dir, op_ssh_sign)?;

    if let Some(parent) = main_path.parent() {
        fs::create_dir_all(parent)?;
    }
    clear_local_dir(local_dir)?;

    for (path, content) in &rendered.directories {
        fs::write(path, content)?;
    }
    fs::write(main_path, rendered.main)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    run(&cli.json, &cli.main_conf, &cli.local_dir, &cli.op_ssh_sign)
}
