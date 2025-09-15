use crate::config::Config;
use crate::prompt::{Prompt, PromptResult};
use crate::util;
use regex::Regex;
use std::process::Command;

pub use crate::error::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub struct App {
    config: Config,
}

impl App {
    pub fn run(&self) -> Result {
        if self.config.args.verbose {
            println!("{:#?}", self.config.args);
        }

        if let Some(key) = self.config.args.key.as_deref() {
            self.run_key(key)
        } else {
            self.run_repl()
        }
    }

    fn run_repl(&self) -> Result {
        use crossterm::terminal;

        terminal::enable_raw_mode()?;
        let result = self.repl_loop();
        terminal::disable_raw_mode()?;

        match result {
            Ok(Some(key)) => self.run_key(key.as_str()),
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn repl_loop(&self) -> Result<Option<String>> {
        use crossterm::event::{self, Event};

        let poll_timeout = std::time::Duration::from_millis(500);

        let mut prompt = Prompt::new("> ");
        prompt.print_prompt()?;

        loop {
            if event::poll(poll_timeout)? {
                if let Event::Key(event) = event::read()? {
                    match prompt.handle_event(event)? {
                        PromptResult::Value(input) if self.has_key(input) => {
                            let key = input.to_string();
                            prompt.next_line()?;
                            break Ok(Some(key));
                        }
                        PromptResult::Exit => {
                            break Ok(None);
                        }
                        PromptResult::Noop | PromptResult::Value(_) => {}
                    }
                }
            }
        }
    }

    fn run_key(&self, key: &str) -> Result {
        let command = self.resolve(key)?;
        self.exec(&command)
    }

    fn exec(&self, command_s: &str) -> Result {
        use std::fs::{create_dir_all, File};

        println!("Running shell command:\n{}", command_s);

        let mut command = self.create_command(command_s);

        let mut file_options = File::options();
        file_options.write(true).create(true).truncate(true);

        if let Some(path) = self.config.args.stdout.as_ref() {
            if let Some(parent) = path.parent() {
                create_dir_all(parent).map_err(|e| (e, parent))?;
            }

            let file = file_options.open(path).map_err(|e| (e, path))?;
            command.stdout(file);
        } else {
            command.stdout(std::process::Stdio::inherit());
        }

        if let Some(path) = self.config.args.stderr.as_ref() {
            if let Some(parent) = path.parent() {
                create_dir_all(parent).map_err(|err| (err, parent))?;
            }

            let file = file_options.open(path).map_err(|e| (e, path))?;
            command.stderr(file);
        } else {
            command.stderr(std::process::Stdio::inherit());
        }

        let mut child = command.spawn()?;

        if !self.config.args.background {
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
        util::get_shell(self.config.args.shell.clone())
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

        let replacer = |caps: &regex::Captures| -> Result<String> {
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

        util::replace_all(&re, command, replacer)
    }

    fn has_key(&self, key: &str) -> bool {
        self.config.keybindings.contains_key(key)
    }
}

impl From<Config> for App {
    fn from(config: Config) -> Self {
        Self { config }
    }
}
