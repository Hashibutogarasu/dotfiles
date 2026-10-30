pub fn init_locale() {
    rust_i18n::set_locale(detect_locale());
}

fn detect_locale() -> &'static str {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(value) = std::env::var(key) {
            if !value.trim().is_empty() {
                return normalize(&value);
            }
        }
    }
    "en"
}

fn normalize(value: &str) -> &'static str {
    let base = value.split(['.', '@']).next().unwrap_or("");
    if base.starts_with("ja") {
        "ja"
    } else {
        "en"
    }
}
