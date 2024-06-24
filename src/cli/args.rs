/*******************************************************************************
 * Copyright (c) 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved
 * Licensed under the EUPL v1.2
 ******************************************************************************/


use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
#[clap(about)]
pub struct JournalArgs {
    #[clap(subcommand)]
    pub journal_command: Option<JournalCommand>,
}

#[derive(Debug, Subcommand)]
pub enum JournalCommand {
    /// Initialise your journal
    Init,

    /// Creates a new journal entry for today
    New,

    /// Open today's entry
    Open(OpenArgs),
    // /// Prints a file to stdout. Default to today's entry.
    // Show,
}

#[derive(Debug, Args)]
pub struct OpenArgs {
    /// The date of the entry you want to open in 'YYYY-MM-DD' format
    #[arg(short, long)]
    pub date: Option<String>,
}