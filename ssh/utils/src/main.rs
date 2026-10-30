mod cli;
mod guide;
mod keys;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            output::error(&format!("{:#}", err));
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate(args) => {
            let output_dir = cli::resolve_output_dir(args.output_dir.as_deref())?;
            let comment = args
                .comment
                .clone()
                .unwrap_or_else(|| cli::default_comment(&args.name));
            let spinner = output::spinner("Generating Ed25519 keypair");
            let result = keys::generate(&output_dir, &args.name, &comment, args.force);
            match result {
                Ok(generated) => {
                    spinner.finish_and_clear();
                    output::success("Keypair generated");
                    output::detail("Private key", &generated.private_path.display().to_string());
                    output::detail("Public key", &generated.public_path.display().to_string());
                    output::detail("Fingerprint", &generated.fingerprint);
                    output::info("Next: run `ssh-utils guide` to register the public key and set up your SSH client by hand");
                    Ok(())
                }
                Err(err) => {
                    spinner.finish_and_clear();
                    Err(err)
                }
            }
        }
        Commands::Guide(args) => {
            let output_dir = cli::resolve_output_dir(args.output_dir.as_deref())?;
            let private_path = output_dir.join(&args.name);
            let public_path = output_dir.join(format!("{}.pub", args.name));
            let options = guide::GuideOptions {
                output_dir_display: cli::display_path(&output_dir),
                private_key_display: cli::display_path(&private_path),
                public_key_display: cli::display_path(&public_path),
                name: args.name.clone(),
                host: args.host.clone(),
                user: args.user.clone(),
            };
            println!("{}", guide::render_guide(&options));
            Ok(())
        }
    }
}
