use crate::config::Config;
use regex::Regex;

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

        println!("Running shell command:\n{}", &command);

        Ok(())
    }

    fn resolve(&self, key: &str) -> Result<String> {
        let re = Regex::new(r"@(\w+)")?;

        let command: &str = self
            .config
            .keybindings
            .get(key)
            .ok_or_else(|| Error::KeyNotFound(key.to_string()))?;

        let rep = |caps: &regex::Captures| -> Result<String> {
            match caps.get(1).map(|m| m.as_str()) {
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
}
