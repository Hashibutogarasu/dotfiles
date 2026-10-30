use anyhow::{bail, Context, Result};
use ssh_key::{rand_core::OsRng, Algorithm, HashAlg, LineEnding, PrivateKey};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub struct GeneratedKeys {
    pub private_path: PathBuf,
    pub public_path: PathBuf,
    pub fingerprint: String,
}

#[cfg(unix)]
fn set_permissions(path: &Path, mode: u32) -> Result<()> {
    let permissions = std::fs::Permissions::from_mode(mode);
    std::fs::set_permissions(path, permissions)
        .with_context(|| format!("failed to set permissions on {}", path.display()))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_permissions(_path: &Path, _mode: u32) -> Result<()> {
    Ok(())
}

pub fn generate(
    output_dir: &Path,
    name: &str,
    comment: &str,
    force: bool,
) -> Result<GeneratedKeys> {
    let private_path = output_dir.join(name);
    let public_path = output_dir.join(format!("{}.pub", name));

    if !force && (private_path.exists() || public_path.exists()) {
        bail!(
            "key files already exist at {} or {}; refusing to overwrite without --force",
            private_path.display(),
            public_path.display()
        );
    }

    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("failed to create directory {}", output_dir.display()))?;
    set_permissions(output_dir, 0o700)?;

    let mut private_key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)
        .context("failed to generate Ed25519 keypair")?;
    private_key.set_comment(comment.to_string());

    let private_openssh = private_key
        .to_openssh(LineEnding::LF)
        .context("failed to encode private key in OpenSSH format")?;
    let public_openssh = private_key
        .public_key()
        .to_openssh()
        .context("failed to encode public key in OpenSSH format")?;
    let fingerprint = private_key
        .public_key()
        .fingerprint(HashAlg::Sha256)
        .to_string();

    std::fs::write(&private_path, private_openssh.as_bytes())
        .with_context(|| format!("failed to write private key to {}", private_path.display()))?;
    set_permissions(&private_path, 0o600)?;

    let public_contents = if public_openssh.ends_with('\n') {
        public_openssh
    } else {
        format!("{}\n", public_openssh)
    };
    std::fs::write(&public_path, public_contents)
        .with_context(|| format!("failed to write public key to {}", public_path.display()))?;
    set_permissions(&public_path, 0o644)?;

    Ok(GeneratedKeys {
        private_path,
        public_path,
        fingerprint,
    })
}
