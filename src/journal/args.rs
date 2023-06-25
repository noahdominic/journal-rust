use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
#[clap(about)]
pub struct JournalArgs {
    #[clap(subcommand)]
    pub journal_command: Option<JournalCommand>,
}

#[derive(Debug, Subcommand)]
pub enum JournalCommand {
    /// Creates a new journal entry for today
    New,
    // /// Open a file. Defaults to today's entry.
    // Open,

    // /// Prints a file to stdout. Default to today's entry.
    // Show,
    /// Initialise the your journal
    Init,
}
