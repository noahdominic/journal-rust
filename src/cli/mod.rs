// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

mod args;
mod interaction;

use indoc;

/** Calls the appropriate function for each subcommand (`init`, `new`, `open`)
 */
pub fn handle_main() -> Result<(), Box<dyn std::error::Error>> {
    let args = <args::JournalArgs as clap::Parser>::parse();
    if let Some(command) = args.journal_command {
        match command {
            args::JournalCommand::Init => handle_init()?,
            args::JournalCommand::New => (),
            args::JournalCommand::Open => (),
        }
    }
    Ok(())
}

/** Runs the journal initialisation routine
*/
fn handle_init() -> Result<(), Box<dyn std::error::Error>> {
    println!(indoc::indoc! {r#"
        --Welcome to Journey!--
        
        This command-line interface app is here to help you document your thoughts,
        experiences, and ideas effortlessly.  Let's get you started :)

        Let's initialise your journal.
    "#});

    let _ = interaction::pause()?;

    println!(indoc::indoc! {r#"

    
    --Set your usual location--
    
    Your journal will use your default location to automatically detect your
    default time zone and to detect the current weather.  This will also be printed
    in your entries.  To ensure the best results, make sure that the last part of
    your location is somewhere that is specific enough for accurate time zone and
    weather data.
    
    Examples:                               
    Avenida 9 SO - Carchi, Guayaquil........Will list locations named 'Guayaquil';
                                                user will need to choose.
    Lor Marzuki, Singapore City.............Only one location named 'Singapore 
                                                City', which is automatically 
                                                chosen.
    Al Quds Open University, Gaza...........Will list locations named 'Gaza';
                                                user will need to choose.
    "#});

    // ask for location
    let (default_location_string, default_location) = interaction::ask::ask_user_for_location()?;

    Ok(())
}
