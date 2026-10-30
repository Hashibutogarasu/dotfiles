pub struct GuideOptions {
    pub output_dir_display: String,
    pub private_key_display: String,
    pub public_key_display: String,
    pub name: String,
    pub host: String,
    pub user: Option<String>,
}

pub fn host_block(options: &GuideOptions) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("Host {}", options.host));
    if let Some(user) = &options.user {
        lines.push(format!("  User {}", user));
    }
    lines.push(format!("  IdentityFile {}", options.private_key_display));
    lines.push("  IdentitiesOnly yes".to_string());
    lines.push("  ProxyCommand cloudflared access ssh --hostname %h".to_string());
    lines.push("  ServerAliveInterval 30".to_string());
    lines.push("  ServerAliveCountMax 3".to_string());
    lines.push("  StrictHostKeyChecking accept-new".to_string());
    lines.join("\n")
}

pub fn render_guide(options: &GuideOptions) -> String {
    let block = host_block(options);
    format!(
        r#"SSH client setup guide (Cloudflare Tunnel / cloudflared)
==========================================================

This guide covers the steps you do yourself after ssh-utils has
generated your keypair. ssh-utils only generates and places the
keys; it does not edit ~/.ssh/config or any other client config.
The config block in step 4 is added by hand.

1. Prerequisites
----------------
Install cloudflared on this machine:
  https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/

Confirm that the tunnel-side hostname you will connect to (for
example a host matching {host}) is published in Cloudflare Zero
Trust as an SSH application / public hostname on a Cloudflare
Tunnel, and that your identity is allowed by its Access policy.

2. Generate keys
----------------
Run:

  ssh-utils generate --output-dir {output_dir}

This creates:

  {private_key}  (permissions 600, directory permissions 700)
  {public_key}   (permissions 644)

{private_key} is your private key. Keep it secret and never copy
it to the server. {public_key} is the public key you register in
the next step. Re-running generate will refuse to overwrite
existing files unless you pass --force.

3. Register the public key on the server
----------------------------------------
Append {name}.pub to ~/.ssh/authorized_keys on the target host.
With ssh-copy-id:

  ssh-copy-id -i {public_key} user@host

Or manually, from this machine:

  cat {public_key} | ssh user@host "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys"
  ssh user@host "chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys"

Use the user and host that apply to the machine you are
registering the key on.

4. Write the client config by hand
----------------------------------
Create or edit ~/.ssh/config yourself and add this block:

{block}

Replace the Host pattern, IdentityFile path, and user with your
own values if they differ from the ones shown here. Make sure
the file permissions are restrictive:

  chmod 700 ~/.ssh
  chmod 600 ~/.ssh/config

5. Connect and verify
---------------------
Connect with:

  ssh <host>

where <host> matches {host}, for example the published tunnel
hostname. For debugging, use verbose output:

  ssh -v <host>

Check the fingerprint of the generated public key with:

  ssh-keygen -lf {public_key}

and compare it with the fingerprint ssh-utils printed when the
key was generated.

6. Troubleshooting
------------------
Permission errors from ssh (key ignored or "permissions are too
open"): fix the private key permissions:

  chmod 700 {output_dir}
  chmod 600 {private_key}

A browser window or `cloudflared access login` prompt when you
connect is expected: Cloudflare Access authenticates you in the
browser before the SSH session starts. Complete that login with
an identity allowed by the Access policy.

"ProxyCommand: cloudflared: No such file or directory" or
"executable file not found": cloudflared is not installed or is
not on your PATH. Reinstall it using the link in step 1 and open
a new shell so PATH is refreshed.
"#,
        host = options.host,
        output_dir = options.output_dir_display,
        private_key = options.private_key_display,
        public_key = options.public_key_display,
        name = options.name,
        block = block,
    )
}
