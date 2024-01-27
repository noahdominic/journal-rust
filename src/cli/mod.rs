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
  Avenida 9 SO - Carchi, Guayaquil
  1600 Pennsylvania Avenue NW, Washington, D.C
  Lor Marzuki, Singapore City
  Al Quds Open University, Gaza
  25 Paddington Grn, City of Westminster
"#
            ),
            HelperMessage::TutorialEditor => write!(f, r#"
--Set your editor--

Journey lets you use your preferred text editor, such as vim, nano, or emacs.
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
    let _ = interaction::pause()?;

    // ask for location
    println!("\n\n{}", HelperMessage::TutorialEditor);
    let editor = interaction::ask::ask_for_editor_multichoice()?;

    let config_contents = format!(
        "[defaults]\n\
        location_full_name=\"{}\"\n\
        location_latitude=\"{}\"\n\
        location_longitude=\"{}\"\n\
        timezone=\"{}\"\n\
        editor=\"{}\"\n",
        default_location_string,
        default_location.latitude,
        default_location.longitude,
        default_location.timezone,
        editor
    );

    println!(
        "\nHere are the settings we've made for you: \n{}",
        config_contents
    );
    Ok(())
}


