/*******************************************************************************
 * Copyright (c) 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved
 * Licensed under the EUPL v1.2
 ******************************************************************************/


pub(in crate::cli) mod ask;
pub(in crate::cli) mod q_basic;

use crate::core;


#[derive(Debug)]
pub(in crate::cli) enum InteractionError {
    IoError(std::io::Error),
    ChronoParseError(chrono::ParseError),
    ParseIntError(std::num::ParseIntError),
    JourneyCoreError(core::JourneyCoreError)
}

impl From<std::io::Error> for InteractionError {
    fn from(error: std::io::Error) -> Self {
        InteractionError::IoError(error)
    }
}

impl From<chrono::ParseError> for InteractionError {
    fn from(error: chrono::ParseError) -> Self { InteractionError::ChronoParseError(error) }
}

impl From<std::num::ParseIntError> for InteractionError {
    fn from(error: std::num::ParseIntError) -> Self {
        InteractionError::ParseIntError(error)
    }
}

impl From<core::JourneyCoreError> for InteractionError {
    fn from(error: core::JourneyCoreError) -> Self {
        InteractionError::JourneyCoreError(error)
    }
}

impl std::fmt::Display for InteractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionError::IoError(ref err) => err.fmt(f),
            InteractionError::ChronoParseError(ref err) => err.fmt(f),
            InteractionError::ParseIntError(ref err) => err.fmt(f),
            InteractionError::JourneyCoreError(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for InteractionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InteractionError::ParseIntError(err) => Some(err),
            InteractionError::ChronoParseError(err) => Some(err),
            InteractionError::IoError(err) => Some(err),
            InteractionError::JourneyCoreError(err) => Some(err),
        }
    }
}


pub(in crate::cli) fn pause() -> std::io::Result<()> {
    #[cfg(windows)]
    let res = press_btn_continue::wait("Press any key to continue...");

    #[cfg(not(windows))]
    let res = unix_pause();

    res
}

#[cfg(not(windows))]
fn unix_pause() -> std::io::Result<()> {
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
