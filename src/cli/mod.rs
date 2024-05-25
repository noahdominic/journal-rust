// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

mod args;
mod helper;
mod interaction;

use crate as journey2;

enum HelperMessage {
    TutorialWelcome,
    TutorialLocation,
    TutorialEditor,
}

impl std::fmt::Display for HelperMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelperMessage::TutorialWelcome => write!(
                f,
                r#"
--Welcome to Journey!--

This command-line interface app is here to help you document your thoughts,
experiences, and ideas effortlessly.  Let's get you started :)
"#
            ),
            HelperMessage::TutorialLocation => write!(
                f,
                r#"
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
            HelperMessage::TutorialEditor => write!(
                f,
                r#"
--Set your editor--

Journey lets you use your preferred text editor, such as vim, nano, or emacs.
"#
            ),
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
            args::JournalCommand::New => handle_new()?,
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
    let editor = interaction::ask::ask_for_editor()?;

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

    if journey2::core::file::is_config_file_exists()? {
        if !interaction::ask::ask_if_to_overwrite_config()? {
            // Was cancelled
            return Ok(());
        }
    }

    journey2::core::file::write_contents_to_config_file(config_contents)?;

    Ok(())
}

fn handle_new() -> Result<(), Box<dyn std::error::Error>> {
    if !helper::is_journal_initialised()? {
        return Ok(()); // Early return if journal not initialised
    }

    let config_data = journey2::core::file::get_config_from_config_file()?;

    println!("{:?}", config_data);

    let (location_full_name, location_latitude, location_longitude, timezone, editor) = (
        config_data.defaults.location_full_name,
        config_data.defaults.location_latitude,
        config_data.defaults.location_longitude,
        config_data.defaults.timezone,
        config_data.defaults.editor,
    );

    let current_date =
        journey2::core::chrono::get_current_date_from_tz_as_str(&timezone)?;

    println!(
        "{:?}",
        journey2::core::chrono::get_current_date_from_tz_as_str(&timezone)
    );

    println!(
        "{:?}",
        journey2::core::weather::query::query_current_weather(
            &current_date.to_string(),
            &location_latitude.to_string(),
            &location_latitude.to_string(),
            &timezone
        )?
    );

    Ok(())
}
