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

use crate::journal::file::FileError;

const MESSAGE_GREETING_CONFIG_INIT: &str = r#"

--Welcome to journal_CLI!--

This command-line interface app is here to help you document your thoughts,
experiences, and ideas effortlessly.  Let's get you started :)

For this part, we'll set your defaults.
"#;

const MESSAGE_LOCATION_EXPLAINER: &str = r#"
We'll only need your usual location.  

We use your default location to automatically detect your default timezome and 
to detect the current weather.  This will also be printed in your entries.  
To ensure the best results, make sure that the last part of your location is 
somewhere that is specific enough for accurate timezone and weather data.

Don't worry---if your city has the same name as a city elsewhere,
like Los Angeles, Los Santos or San Francisco, Cebu,
you would be asked to pick which city you meant.

Example:
- Avenida 9 SO - Carchi, Guiyaquil
- Lor Marzuki, Singapore City
- Café What?, Moshoeshoe Rd, Maseru
"#;

const MESSAGE_EDITORS_EXPLAINER: &str = r#"
This application does not use its own text editors and will separately run 
a text editor of your own choosing, like vim, nano, and emacs.
"#;

pub(crate) fn init_new_config_driver() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", MESSAGE_GREETING_CONFIG_INIT);
    println!("{}", MESSAGE_LOCATION_EXPLAINER);

    // default_location_name and default_location are separate bc
    //      default_location_name IS user input
    //      but default_location IS api information based on last substring of default_location_name
    let (default_location_name, default_location) =
        crate::journal::query::user::ask_for_location()?;

    println!("{}", MESSAGE_EDITORS_EXPLAINER);

    let editor = crate::journal::query::user::ask_for_text_editor_multchoice()?;

    let config_contents = format!(
        "[defaults]\n\
        location_full_name=\"{}\"\n\
        location_latitude=\"{}\"\n\
        location_longitude=\"{}\"\n\
        timezone=\"{}\"\n\
        editor=\"{}\"\n",
        default_location_name,
        default_location.latitude,
        default_location.longitude,
        default_location.timezone,
        editor
    );

    println!(
        "\nHere are the settings we've made for you: \n{}",
        config_contents
    );

    // Ask user for path of config file
    //      Prompt: Where do you want to put config.toml?
    let config_file_path = crate::journal::query::user::ask_for_config_file_path()?;

    // If it doesn't exist, create the directories; return the PathBuf of created/existing path
    let config_file_pathbuf = crate::journal::file::mkdir_p(config_file_path)?;

    // Add filename to that PathBuf
    let config_file_pathbuf = config_file_pathbuf.join("config.toml");

    // Check for file if file already exists
    let is_proceed_with_writing = crate::journal::file::is_proceed_with_writing(&config_file_pathbuf)?;

    if !is_proceed_with_writing {
        // Early return.  No file writing needed
        return Ok(());
    }

    // Write the settings to the path
    let mut file = std::fs::File::create(&config_file_pathbuf)?;
    std::io::Write::write_all(&mut file, config_contents.as_bytes())?;

    // Write the path to config.toml to ~/.journal
    let dotfile_pathbuf = crate::journal::file::get_dotfile_path()?;
    let mut dotfile = std::fs::File::create(&dotfile_pathbuf)?;
    std::io::Write::write_all(
        &mut dotfile,
        config_file_pathbuf
            .parent()
            .ok_or(FileError::HomeDirNotFound)?
            .to_string_lossy()
            .as_bytes(),
    )?;

    Ok(())
}

pub(crate) fn create_new_entry_driver() -> Result<(), Box<dyn std::error::Error>> {
    // Check if the journal is initialised
    if !is_journal_initialised_checker()? {
        return Ok(());
    }
    // This will read the contents of the dotfile, which is the path of the config file, which is set by the user in the init
    let journal_dir = crate::journal::file::read_dotfile()?;

    // Retrieve details from config file
    let (location_full_name, location_latitude, location_longitude, timezone, editor ) =
        crate::journal::file::read_configfile(&journal_dir)?;

    // Create the file here
    let filepath_for_todays_entry = crate::journal::calculators::get_path_to_todays_entry(journal_dir)?;

    let filepath_for_dir =  std::path::Path::new(&filepath_for_todays_entry).parent().expect("Error in extracting parent of today's entry").to_str().expect("Error in converting Path to str");

    std::fs::create_dir_all(filepath_for_dir)?;

    let mut file_for_todays_entry = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&filepath_for_todays_entry)?;

    let current_date = crate::journal::calculators::get_current_date_from_tz_as_str(&timezone)?;

    // Use info from config file to query weather from OpenMeteo API
    let current_weather = crate::journal::query::open_meteo::for_weather_info(
        &(current_date
            .format("%Y-%m-%d %H:%M").to_string()),
        &location_latitude,
        &location_longitude,
        &timezone,
    )?;

    let weather_map = std::collections::HashMap::from([
        (0, "Clear skies"),
        (1, "Mainly clear skies"),
        (2, "Partly cloudy skies"),
        (3, "Overcast skies"),
        (45, "Fog"),
        (48, "Fog"),
        (51, "Light drizzle"),
        (53, "Moderate drizzle"),
        (55, "Heavy drizzle"),
        (56, "Light drizzle, freezing"),
        (57, "Moderate or heavy drizzle, freezing"),
        (61, "Light rain"),
        (63, "Moderate rain"),
        (65, "Heavy rain"),
        (66, "Light rain, freezing"),
        (67, "Moderate or heavy rain, freezing"),
        (71, "Snow fall: Slight intensity"),
        (73, "Snow fall: Moderate intensity"),
        (75, "Snow fall: Heavy intensity"),
        (77, "Snow grains"),
        (80, "Light rain showers"),
        (81, "Moderate rain showers"),
        (82, "Violent rainshowers"),
        (85, "Snow showers: Slight intensity"),
        (86, "Snow showers: Heavy intensity"),
        (95, "Thunderstorm: Slight or moderate"),
        (96, "Thunderstorm with slight hail"),
        (99, "Thunderstorm with heavy hail"),
    ]);

    let preamble_str = format!(
        "DATE: {}\n\
        LOCATION: {}\n\
        \n\
        Temperature: {} C, feels like {} C, {}.\n\
        UV Index: {}  Sunrise: {}   Sunset: {}\n\
        Rain: {} mm\n\
        Winds: {} km/h {}\n\
        Pressure: {} hPa\n\
        Humidity: {}%\n\
        Visibility: {} km\n\
        ",
        current_date.format("%a, %Y %b %d %H:%M:%S %Z (%:z)"),
        location_full_name,
        current_weather.temperature,
        current_weather.apparent_temperature,
        weather_map
            .get(&current_weather.weather_code)
            .unwrap_or(&"Unknown conditions"),
        current_weather.uv_index,
        current_weather.sunrise,
        current_weather.sunset,
        current_weather.rain,
        current_weather.windspeed,
        crate::journal::calculators::get_direction(current_weather.winddirection),
        current_weather.pressure,
        current_weather.humidity,
        current_weather.visibility / 1000.0
    );

    // The line that writes var file_message into the file.
    std::io::Write::write_all(&mut file_for_todays_entry, preamble_str.as_bytes())?;

    // Calls the user's editor command, as deserialised from the config file
    let status = std::process::Command::new(&editor)
        .arg(&filepath_for_todays_entry)
        .status()?;

    // ? This is a dev print, but should we keep this?
    if status.success() {
        println!("File opened in {}", editor);
    } else {
        eprintln!("Failed to open file in {}", editor);
    }

    Ok(())
}

pub(crate) fn open_todays_entry_driver() ->  Result<(), Box<dyn std::error::Error>> {
    // // Check if the journal is initialised
    // // If it doesn't it execs an early return of Ok(())
    // if !is_journal_initialised_driver()? {
    //     return Ok(());
    // }
    //
    // // This will read the contents of the dotfile, which is the path of the config file, which is set by the user in the init
    // let journal_dir = crate::journal::file::get_base_dir()?;
    //
    // // Retrieve details from config file
    // let (_, _, _, timezone, editor ) =
    //     crate::journal::file::get_config_details()?;
    //
    // // Create the file here
    // let filepath_for_todays_entry = crate::journal::calculators::get_path_to_todays_entry(journal_dir, &timezone)?;
    //
    // // Calls the user's editor command, as deserialised from the config file
    // let status = std::process::Command::new(&editor)
    //     .arg(&filepath_for_todays_entry)
    //     .status()?;
    //
    // // ? This is a dev print, but should we keep this?
    // if status.success() {
    //     println!("File opened in {}", editor);
    // } else {
    //     eprintln!("Failed to open file in {}", editor);
    // }

    Ok(())
}


fn is_journal_initialised_checker() -> Result<bool, FileError> {
    // Is it true that the file does NOT exist?
    // i.e. If the file exists, do not run what is inside then return true.
    if !crate::journal::file::is_dotfile_exists()? {
        println!(
            "Oops!  Looks like you haven't initialised your journal yet.  Try running `journal init` first."
        );
        return Ok(false);
    }

    return Ok(true);
}
