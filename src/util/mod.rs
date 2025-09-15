pub mod expand_path;
pub mod merge;

pub use expand_path::*;
pub use merge::*;

use regex::{Captures, Regex};
use std::ffi::OsStr;
use std::path::Path;

// https://docs.rs/regex/1.11.2/regex/struct.Regex.html#fallibility
pub fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

pub fn get_shell<'a>(shell_conf: Option<String>) -> (String, &'a str) {
    match shell_conf.or_else(|| std::env::var("SHELL").ok()) {
        Some(sh) if matches_filenames(sh.as_ref(), ["sh", "bash", "zsh"]) => {
            (sh, "-c")
        }
        Some(cmd) if matches_filenames(cmd.as_ref(), ["cmd", "cmd.exe"]) => {
            (cmd, "/C")
        }

        // TODO
        Some(other) => (other, ""),

        None => {
            #[cfg(not(target_os = "windows"))]
            return ("sh".into(), "-c");
            #[cfg(target_os = "windows")]
            return ("cmd".into(), "/C");
        }
    }
}

fn matches_filenames<'a, I: IntoIterator<Item = &'a str>>(
    target: &'a str,
    names_iter: I,
) -> bool {
    let target = Path::new(target)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or(target);
    names_iter.into_iter().any(|item| item == target)
}
