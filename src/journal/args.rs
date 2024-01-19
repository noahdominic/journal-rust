// Copyright 2023  Noah Dominic Miranda Silvio
// Licensed under the EUPL v1.2

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
    /// Initialise the your journal
    Init,

    /// Creates a new journal entry for today
    New,

    /// Open today's entry
    Open,
    // /// Prints a file to stdout. Default to today's entry.
    // Show,
}
