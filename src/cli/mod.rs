// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

mod args;
mod interaction;
mod utils {
    pub(crate) mod enums;
    pub(crate) mod functions;
}

use std::io::Read;

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

    let current_date = journey2::core::chrono::get_current_date_from_tz_as_str(&config_data.timezone)?;

    let current_weather = journey2::core::weather::query::get_current_weather_at_location_and_time(
        &current_date.to_string(),
        &config_data.location_latitude.to_string(),
        &config_data.location_longitude.to_string(),
        &config_data.timezone,
    )?;


    let preamble_str = utils::functions::generate_preamble(&config_data.location_full_name, &current_date, current_weather);

    print!("{}", preamble_str);

    println!("{:?}", journey2::core::file::get_temp_file_path()?);

    // Write temporary file to data directory
    let mut temporary_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(journey2::core::file::get_temp_file_path()?)?;

    std::io::Write::write_all(&mut temporary_file, preamble_str.as_bytes())?;

    // Invoke the editor as a subprocess
    let editor_proc = std::process::Command::new(&config_data.editor)
        .arg(&journey2::core::file::get_temp_file_path()?)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .expect(&format!("Failed to start {}", &config_data.editor));

    if !editor_proc.success() {
        println!("Vim was not successful");
        return Ok(());
    }

    // Read the modified content from the temporary file
    let mut modified_content = String::new();
    std::fs::File::open(&journey2::core::file::get_temp_file_path()?)
        .expect("Failed to open temporary file")
        .read_to_string(&mut modified_content)
        .expect("Failed to read temporary file");

    // Check if there were any changes
    if modified_content == preamble_str {
        println!("No changes found.  Will not be writing into a new entry.");
        return Ok(());
    } else {
        println!("Ok ra man");
    }

    Ok(())
}
