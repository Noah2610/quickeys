use crate::config::Config;
use regex::Regex;
use std::process::Command;

pub use crate::error::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub struct App {
    config: Config,
}

impl App {
    pub fn run(&self) -> Result {
        Ok(())
    }

    pub fn run_key(&self, key: &str) -> Result {
        let command = self.resolve(key)?;

        self.exec(&command)
    }

    fn exec(&self, command_s: &str) -> Result {
        use std::fs::{create_dir_all, File};

        println!("Running shell command:\n{}", command_s);

        let mut command = self.create_command(command_s);

        let mut file_options = File::options();
        file_options.write(true).create(true).truncate(true);

        if let Some(path) = self.config.config.stdout.as_ref() {
            if let Some(parent) = path.parent() {
                create_dir_all(parent).map_err(|e| (e, parent))?;
            }

            let file = file_options.open(path).map_err(|e| (e, path))?;
            command.stdout(file);
        } else {
            command.stdout(std::process::Stdio::inherit());
        }

        if let Some(path) = self.config.config.stderr.as_ref() {
            if let Some(parent) = path.parent() {
                create_dir_all(parent).map_err(|err| (err, parent))?;
            }

            let file = file_options.open(path).map_err(|e| (e, path))?;
            command.stderr(file);
        } else {
            command.stderr(std::process::Stdio::inherit());
        }

        let mut child = command.spawn()?;

        if !self.config.config.background {
            child.wait()?;
        }

        Ok(())
    }

    fn create_command(&self, command_s: &str) -> Command {
        let (shell, arg) = self.get_shell();
        let mut command = Command::new(shell);
        command.arg(arg).arg(command_s);
        command
    }

    fn get_shell(&self) -> (String, &str) {
        use util::matches_filenames;

        match self
            .config
            .config
            .shell
            .as_ref()
            .map(ToString::to_string)
            .or_else(|| std::env::var("SHELL").ok())
        {
            Some(sh)
                if matches_filenames(sh.as_ref(), ["sh", "bash", "zsh"]) =>
            {
                (sh, "-c")
            }
            Some(cmd)
                if matches_filenames(cmd.as_ref(), ["cmd", "cmd.exe"]) =>
            {
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

    fn resolve(&self, key: &str) -> Result<String> {
        // TODO: Can't have literal '@' in commands without being expanded.
        //       Should implement @ escaping (ex. '\@' or '@@').
        let re = Regex::new(r"@(?:\{\s*)?(?<ident>\w+)(?:\s*\})?")?;

        let command: &str = self
            .config
            .keybindings
            .get(key)
            .ok_or_else(|| Error::KeyNotFound(key.to_string()))?;

        let rep = |caps: &regex::Captures| -> Result<String> {
            match caps.name("ident").map(|m| m.as_str()) {
                Some(constant) => self
                    .config
                    .constants
                    .get(constant)
                    .map(ToString::to_string)
                    .ok_or_else(|| Error::ConstantNotFound {
                        constant: constant.into(),
                        command: command.into(),
                    }),
                None => Err(Error::Unreachable(format!(
                    "Expected capture group to exist\n{:#?}",
                    caps
                ))),
            }
        };

        util::replace_all(&re, command, rep)
    }
}

impl From<Config> for App {
    fn from(config: Config) -> Self {
        Self { config }
    }
}

mod util {
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

    pub fn matches_filenames<'a, I: IntoIterator<Item = &'a str>>(
        target: &'a str,
        names_iter: I,
    ) -> bool {
        let target = Path::new(target)
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or(target);
        names_iter.into_iter().any(|item| item == target)
    }
}
