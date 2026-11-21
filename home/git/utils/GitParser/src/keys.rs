use serde::{Deserialize, de::DeserializeOwned};
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Deserialize)]
struct Config {
    keys: Vec<Entry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    key_id: String,
    reference: Option<String>,
}

#[derive(Deserialize)]
struct Vault {
    id: String,
}

#[derive(Deserialize)]
struct Item {
    id: String,
    vault: Vault,
}

#[derive(Deserialize)]
struct Field {
    value: String,
}

impl Item {
    fn reference(&self, field: &str) -> String {
        format!("op://{}/{}/{field}", self.vault.id, self.id)
    }
}

fn run_json<T: DeserializeOwned>(command: &mut Command) -> Result<T> {
    let output = command.stderr(Stdio::inherit()).output()?;
    if !output.status.success() {
        return Err(format!("{command:?} exited with {}", output.status).into());
    }
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn op_json<T: DeserializeOwned>(args: &[&str]) -> Result<T> {
    run_json(Command::new("op").args(args).args(["--format", "json"]))
}

fn pipe_secret(reference: &str, mut sink: Command) -> Result<()> {
    let mut op = Command::new("op")
        .args(["read", reference])
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = op.stdout.take().ok_or("failed to capture op output")?;
    let sink_status = sink.stdin(Stdio::from(stdout)).status()?;
    let op_status = op.wait()?;
    if !op_status.success() {
        return Err(format!("op read exited with {op_status}").into());
    }
    if !sink_status.success() {
        return Err(format!("{sink:?} exited with {sink_status}").into());
    }
    Ok(())
}

fn key_part(public_key: &str) -> String {
    public_key.split_whitespace().take(2).collect::<Vec<_>>().join(" ")
}

fn ssh_auth_sock() -> Result<String> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
    Ok(format!("{runtime_dir}/ssh-agent"))
}

fn remove_ssh_key(public_key: &str, auth_sock: &str) {
    let spawned = Command::new("ssh-add")
        .args(["-d", "/dev/stdin"])
        .env("SSH_AUTH_SOCK", auth_sock)
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();
    if let Ok(mut child) = spawned {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = writeln!(stdin, "{public_key}");
        }
        let _ = child.wait();
    }
}

fn import_ssh_key(key_id: &str, auth_sock: &str) -> Result<()> {
    let public_key = key_part(key_id);
    let items: Vec<Item> = op_json(&["item", "list", "--categories", "SSH Key"])?;
    for item in items {
        let field: Field = op_json(&["item", "get", &item.id, "--fields", "public key"])?;
        if key_part(&field.value) == public_key {
            remove_ssh_key(&public_key, auth_sock);
            let mut ssh_add = Command::new("ssh-add");
            ssh_add.arg("-").env("SSH_AUTH_SOCK", auth_sock);
            return pipe_secret(&item.reference("private_key?ssh-format=openssh"), ssh_add);
        }
    }
    Err(format!("no SSH key matches the public key {public_key}").into())
}

fn import_gpg_key(key_id: &str, reference: Option<&str>) -> Result<()> {
    let reference = match reference {
        Some(reference) => reference.to_owned(),
        None => {
            let item: Item = op_json(&["item", "get", key_id])
                .map_err(|e| format!("no item titled {key_id} found: {e}"))?;
            item.reference("private key")
        }
    };
    let _ = Command::new("gpg")
        .args(["--batch", "--yes", "--delete-secret-and-public-keys", key_id])
        .stderr(Stdio::null())
        .status();
    let mut gpg = Command::new("gpg");
    gpg.args(["--batch", "--yes", "--import"]);
    pipe_secret(&reference, gpg)
}

pub fn import(config_path: &Path) -> Result<()> {
    let text = fs::read_to_string(config_path)
        .map_err(|e| format!("failed to read {}: {e}", config_path.display()))?;
    let config: Config = serde_json::from_str(&text)
        .map_err(|e| format!("failed to parse {}: {e}", config_path.display()))?;

    Command::new("gpgconf").args(["--launch", "gpg-agent"]).status()?;
    Command::new("gpg-connect-agent")
        .args(["updatestartuptty", "/bye"])
        .stdout(Stdio::null())
        .status()?;
    let auth_sock = ssh_auth_sock()?;

    for entry in &config.keys {
        let result = if entry.key_id.starts_with("ssh-") {
            import_ssh_key(&entry.key_id, &auth_sock)
        } else {
            import_gpg_key(&entry.key_id, entry.reference.as_deref())
        };
        if let Err(err) = result {
            eprintln!("1Password: failed to import key {}: {err}", entry.key_id);
        }
    }
    Ok(())
}
