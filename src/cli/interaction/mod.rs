// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

mod primitive;

use crossterm;

pub(in crate::cli) fn pause() -> std::io::Result<()> {
    println!("Press any key to continue...");

    crossterm::terminal::enable_raw_mode()?;

    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(_) = crossterm::event::read()? {
                break;
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
