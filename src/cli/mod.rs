// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

mod args;
mod interaction;
mod utils {
    pub(crate) mod enums;
    pub(crate) mod functions;
}

use crate as journey2;

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
    println!("{}", utils::enums::HelperMessage::TutorialWelcome);
    let _ = interaction::pause()?;

    // ask for location
    println!("\n\n{}", utils::enums::HelperMessage::TutorialLocation);
    let (default_location_string, default_location) = interaction::ask::ask_user_for_location()?;
    let _ = interaction::pause()?;

    // ask for location
    println!("\n\n{}", utils::enums::HelperMessage::TutorialEditor);
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
    if !utils::functions::is_journal_initialised_frontend()? {
        return Ok(()); // Early return if journal not initialised
    }

    let config_data = journey2::core::file::get_config_from_config_file()?.defaults;

    let preamble_str = utils::functions::generate_preamble(
        &config_data.location_full_name,
        config_data.location_latitude,
        config_data.location_longitude,
        &config_data.timezone)?;

    print!("{}", preamble_str);

    Ok(())
}
