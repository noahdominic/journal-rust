// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

mod args;
mod interaction;

enum HelperMessage {
    TutorialWelcome,
    TutorialLocation,
    TutorialEditor,
}

impl std::fmt::Display for HelperMessage{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelperMessage::TutorialWelcome => write!(f, r#"
--Welcome to Journey!--

This command-line interface app is here to help you document your thoughts,
experiences, and ideas effortlessly.  Let's get you started :)

Let's initialise your journal.
"#
            ),
            HelperMessage::TutorialLocation => write!(f, r#"
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
"#
            ),
            HelperMessage::TutorialEditor => write!(f, r#"
Journey does not use its own text editor and will separately run
a text editor of your own choosing, like vim, nano, and emacs.
"#
            )
        }
    }
}

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
    println!("{}", HelperMessage::TutorialWelcome);
    let _ = interaction::pause()?;

    // ask for location
    println!("\n\n{}", HelperMessage::TutorialLocation);
    let (default_location_string, default_location) = interaction::ask::ask_user_for_location()?;

    // ask for location
    println!("\n\n{}", HelperMessage::TutorialEditor);
    Ok(())
}


