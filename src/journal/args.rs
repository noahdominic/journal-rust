/*
 * Copyright 2023 Noah Dominic Miranda Silvio
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the European Union Public License version 1.2,
 * as published by the European Commission.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * European Union Public Licence for more details.
 *
 * You should have received a copy of the European Union Public Licence
 * along with this program. If not, see <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
 */

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
