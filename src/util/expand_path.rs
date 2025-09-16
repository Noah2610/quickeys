use std::path::PathBuf;

pub fn expand_path(s: &str) -> PathBuf {
    PathBuf::from(expand_path_str(s))
}

pub fn expand_path_str(s: &str) -> String {
    // Expand '~' to home path
    if s.starts_with('~') {
        if let Some(home) =
            dirs::home_dir().and_then(|p| p.to_str().map(ToString::to_string))
        {
            return s.replacen('~', home.as_str(), 1);
        }
    };

    s.to_string()
}

pub fn expand_path_arg(
    s: &str,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(expand_path(s))
}
