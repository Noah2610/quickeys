use crate::args::{Action, ListArgs, RunArgs};
use crate::config::Config;
use crate::prompt::{Prompt, PromptResult};
use crate::util::{self, Merge};
use regex::Regex;
use std::process::Command;

pub use crate::error::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub struct App {
    config: Config,
    verbose: u8,
}

impl App {
    pub fn with_verbose(self, verbose: u8) -> Self {
        Self { verbose, ..self }
    }

    pub fn run(&self, action: Action) -> Result {
        match self.verbose {
            0 => (),
            1 => {
                eprintln!("{:#?}\nAction {:#?}", self.config.run_args, &action)
            },
            2.. => eprintln!("{:#?}\nAction {:#?}", self.config, &action),
        }

        match action {
            Action::Prompt => self.run_repl(),
            Action::Run { key } => self.run_key(&key),
            Action::List { args } => self.list(args),
        }
    }

    fn list(
        &self,
        ListArgs {
            delimiter,
            key_only,
            command_only,
        }: ListArgs,
    ) -> Result {
        use std::collections::BTreeSet;

        for key in BTreeSet::from_iter(self.config.keybindings.keys()) {
            if key_only {
                println!("{}", key);
            } else if command_only {
                match self.resolve(key)? {
                    (None, s) => println!("{}", s),
                    (Some(args), s) => println!("{} ({:?})", s, args),
                }
            } else {
                match self.resolve(key)? {
                    (None, s) => println!("{}{}{}", key, delimiter, s),
                    (Some(args), s) => {
                        println!(
                            "{}{}{}{}{:?}",
                            key, delimiter, s, delimiter, args
                        )
                    },
                }
            }
        }

        Ok(())
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
                        },
                        PromptResult::Exit => {
                            break Ok(None);
                        },
                        PromptResult::Noop | PromptResult::Value(_) => {},
                    }
                }
            }
        }
    }

    fn run_key(&self, key: &str) -> Result {
        if self.verbose > 2 {
            eprintln!(r#"key: "{}""#, key);
        }

        let command = self.resolve(key)?;
        self.exec(command)
    }

    fn exec(
        &self,
        (run_args_opt, command_s): (Option<RunArgs>, String),
    ) -> Result {
        use std::fs::{create_dir_all, File};

        let config = &self.config;
        let run_args = match run_args_opt {
            Some(args) => config.run_args.clone().merge(args),
            None => config.run_args.clone(),
        };

        let mut command = self.create_command(&command_s);

        let mut file_options = File::options();
        file_options.write(true).create(true).truncate(true);

        if let Some(path) = run_args.stdout.as_ref() {
            if self.verbose > 2 {
                eprintln!(r#"stdout: {:?}"#, path);
            }

            if let Some(parent) = path.parent() {
                if self.verbose > 3 {
                    eprintln!(
                        r#"creating parent directories for stdout: {:?}"#,
                        parent
                    );
                }

                create_dir_all(parent).map_err(|e| (e, parent))?;
            }

            let file = file_options.open(path).map_err(|e| (e, path))?;
            command.stdout(file);
        } else {
            command.stdout(std::process::Stdio::inherit());
        }

        if let Some(path) = run_args.stderr.as_ref() {
            if self.verbose > 2 {
                eprintln!(r#"stderr: {:?}"#, path);
            }

            if let Some(parent) = path.parent() {
                if self.verbose > 3 {
                    eprintln!(
                        r#"creating parent directories for stderr: {:?}"#,
                        parent
                    );
                }

                create_dir_all(parent).map_err(|err| (err, parent))?;
            }

            let file = file_options.open(path).map_err(|e| (e, path))?;
            command.stderr(file);
        } else {
            command.stderr(std::process::Stdio::inherit());
        }

        if run_args.background {
            match fork::daemon(true, true).map_err(Error::ForkError)? {
                fork::Fork::Parent(child_pid) => {
                    if self.verbose > 1 {
                        eprintln!("forked process, child pid: {}", child_pid);
                    }
                    if self.verbose > 0 {
                        eprintln!(r#"running in fork: {}"#, command_s);
                    }

                    fork::waitpid(child_pid).map_err(Error::ForkError)
                },
                fork::Fork::Child => {
                    let status = command.spawn()?.wait()?;
                    if status.success() {
                        std::process::exit(0);
                    } else {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                },
            }
        } else {
            if self.verbose > 0 {
                eprintln!(r#"running: {}"#, command_s);
            }

            let status = command.spawn()?.wait()?;
            if status.success() {
                Ok(())
            } else {
                Err(Error::CommandError {
                    command: command_s.to_string(),
                    code: status.code(),
                })
            }
            // .code()
            // .map(|c| u8::try_from(c).unwrap_or(1))
            // .map(ExitCode::from)
        }
    }

    fn create_command(&self, command_s: &str) -> Command {
        let (shell, arg) = self.get_shell();

        if self.verbose > 2 {
            eprintln!(r#"shell: "{} {}""#, shell, arg);
        }

        let mut command = Command::new(shell);
        command.arg(arg).arg(command_s);
        command
    }

    fn get_shell(&self) -> (String, &str) {
        util::get_shell(self.config.run_args.shell.clone())
    }

    fn resolve(&self, key: &str) -> Result<(Option<RunArgs>, String)> {
        // TODO: Can't have literal '@' in commands without being expanded.
        //       Should implement @ escaping (ex. '\@' or '@@').
        let re = Regex::new(r"@(?:\{\s*)?(?<ident>\w+)(?:\s*\})?")?;

        let (command_args, command) = self
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

        let s = util::replace_all(&re, command, replacer)?;

        Ok((command_args.clone(), s))
    }

    fn has_key(&self, key: &str) -> bool {
        self.config.keybindings.contains_key(key)
    }
}

impl From<Config> for App {
    fn from(config: Config) -> Self {
        Self { config, verbose: 0 }
    }
}
