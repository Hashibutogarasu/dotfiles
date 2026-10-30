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
    t!(
        "guide.body",
        output_dir = options.output_dir_display.as_str(),
        private_key = options.private_key_display.as_str(),
        public_key = options.public_key_display.as_str(),
        host = options.host.as_str(),
        name = options.name.as_str(),
        block = host_block(options)
    )
    .to_string()
}
