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

    if !utils::functions::is_journal_initialised()? {
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

    let current_date = journey2::core::chrono::get_current_date_from_tz_as_str(&timezone)?;

    let current_weather = journey2::core::weather::query::query_current_weather(
        &current_date.to_string(),
        &location_latitude.to_string(),
        &location_longitude.to_string(),
        &timezone,
    )?;

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
        journey2::core::helper::get_direction(current_weather.winddirection),
        current_weather.pressure,
        current_weather.humidity,
        current_weather.visibility / 1000.0
    );

    println!(
        "{:?}",
        journey2::core::chrono::get_current_date_from_tz_as_str(&timezone)
    );

    println!("{:?}", current_weather);

    println!("{}", preamble_str);

    Ok(())
}
