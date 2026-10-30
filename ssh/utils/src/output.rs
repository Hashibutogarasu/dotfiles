use anyhow::Result;
use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn page(text: &str) -> Result<()> {
    let mut pager = minus::Pager::new();
    pager.set_text(text)?;
    pager.set_prompt("ssh-utils guide")?;
    minus::page_all(pager)?;
    Ok(())
}

pub fn success(message: &str) {
    let term = Term::stdout();
    let _ = term.write_line(&format!("{} {}", style("✓").green().bold(), message));
}

pub fn info(message: &str) {
    let term = Term::stdout();
    let _ = term.write_line(&format!("{} {}", style("i").cyan().bold(), message));
}

#[allow(dead_code)]
pub fn warn(message: &str) {
    let term = Term::stderr();
    let _ = term.write_line(&format!("{} {}", style("!").yellow().bold(), message));
}

pub fn error(message: &str) {
    let term = Term::stderr();
    let _ = term.write_line(&format!("{} {}", style("✗").red().bold(), message));
}

pub fn detail(label: &str, value: &str) {
    let term = Term::stdout();
    let _ = term.write_line(&format!(
        "  {} {}",
        style(format!("{}:", label)).bold(),
        value
    ));
}

pub fn spinner(message: &str) -> ProgressBar {
    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner()),
    );
    bar.set_message(message.to_string());
    bar.enable_steady_tick(Duration::from_millis(80));
    bar
}
