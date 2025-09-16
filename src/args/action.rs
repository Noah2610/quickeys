use crate::args::RunArgs;
use crate::util::Merge;
use clap::Subcommand;

#[derive(Debug, Default, Subcommand)]
pub enum Action {
    /// Run an interactive prompt
    ///
    /// While in the prompt, when you type a valid key,
    /// the associated command will be run immediately.
    /// This is the default behavior if no command is given.
    #[default]
    Prompt,

    /// Run command for given key
    ///
    /// Runs the passed KEY's command, defined in the config file.
    Run {
        /// Key to run
        #[arg(value_name = "KEY")]
        key: String,
    },

    /// List resolved keys and their commands
    List,
}

impl Merge for Action {
    fn merge(self, other: Self) -> Self {
        other
    }
}
