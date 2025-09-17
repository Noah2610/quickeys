#[derive(Debug, clap::Args)]
pub struct ListArgs {
    /// Delimiter string between key and command
    ///
    /// Only used when neither --key-only nor --command-only is set.
    #[arg(short, long, default_value = ": ")]
    pub delimiter: String,

    /// Only list keys without their commands
    #[arg(short, long)]
    pub key_only: bool,

    /// Only list commands without their keys
    #[arg(short = 'C', long)]
    pub command_only: bool,
}
