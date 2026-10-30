use rust_i18n::t;

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
    let host = options.host.as_str();
    let output_dir = options.output_dir_display.as_str();
    let private_key = options.private_key_display.as_str();
    let public_key = options.public_key_display.as_str();
    let name = options.name.as_str();
    format!(
        "{title}\n{eq}\n\n{intro}\n\n{step1_heading}\n{dash}\n{step1_install}\n  https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/\n\n{step1_confirm}\n\n{step2_heading}\n{dash}\n{step2_run}\n\n  ssh-utils generate --output-dir {output_dir}\n\n{step2_creates}\n\n  {private_key}  {step2_private_note}\n  {public_key}   {step2_public_note}\n\n{step2_note}\n\n{step3_heading}\n{dash}\n{step3_intro}\n\n  ssh-copy-id -i {public_key} user@host\n\n{step3_or_manually}\n\n  cat {public_key} | ssh user@host \"mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys\"\n  ssh user@host \"chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys\"\n\n{step3_note}\n\n{step4_heading}\n{dash}\n{step4_intro}\n\n{block}\n\n{step4_note}\n\n  chmod 700 ~/.ssh\n  chmod 600 ~/.ssh/config\n\n{step5_heading}\n{dash}\n{step5_connect}\n\n  ssh <host>\n\n{step5_connect_note}\n\n  ssh -v <host>\n\n{step5_fingerprint}\n\n  ssh-keygen -lf {public_key}\n\n{step5_fingerprint_note}\n\n{step6_heading}\n{dash}\n{step6_permission_intro}\n\n  chmod 700 {output_dir}\n  chmod 600 {private_key}\n\n{step6_access}\n\n{step6_proxy}\n",
        title = t!("guide.title"),
        eq = "=".repeat(58),
        dash = "-".repeat(24),
        intro = t!("guide.intro"),
        step1_heading = t!("guide.step1_heading"),
        step1_install = t!("guide.step1_install"),
        step1_confirm = t!("guide.step1_confirm", host = host),
        step2_heading = t!("guide.step2_heading"),
        step2_run = t!("guide.step2_run"),
        step2_creates = t!("guide.step2_creates"),
        step2_private_note = t!("guide.step2_private_note"),
        step2_public_note = t!("guide.step2_public_note"),
        step2_note = t!("guide.step2_note", private_key = private_key, public_key = public_key),
        step3_heading = t!("guide.step3_heading"),
        step3_intro = t!("guide.step3_intro", name = name),
        step3_or_manually = t!("guide.step3_or_manually"),
        step3_note = t!("guide.step3_note"),
        step4_heading = t!("guide.step4_heading"),
        step4_intro = t!("guide.step4_intro"),
        step4_note = t!("guide.step4_note"),
        step5_heading = t!("guide.step5_heading"),
        step5_connect = t!("guide.step5_connect"),
        step5_connect_note = t!("guide.step5_connect_note", host = host),
        step5_fingerprint = t!("guide.step5_fingerprint"),
        step5_fingerprint_note = t!("guide.step5_fingerprint_note"),
        step6_heading = t!("guide.step6_heading"),
        step6_permission_intro = t!("guide.step6_permission_intro"),
        step6_access = t!("guide.step6_access"),
        step6_proxy = t!("guide.step6_proxy"),
        output_dir = output_dir,
        private_key = private_key,
        public_key = public_key,
        block = block,
    )
}
