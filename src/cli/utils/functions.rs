/*******************************************************************************
 * Copyright (c) 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved
 * Licensed under the EUPL v1.2
 ******************************************************************************/


use crate as journey2;
use crate::cli::utils;

pub(crate) fn is_journal_initialised_frontend() -> Result<bool, journey2::core::file::FileError> {
    let is_journal_initialised = journey2::core::file::is_journal_initialised()?;

    if !is_journal_initialised {
        println!(
            "Oops!  Looks like you haven't initialised your journal yet.  Try running `journal init` first."
        );
    }

    Ok(is_journal_initialised)
}

pub(crate) fn generate_preamble(
    location_full_name: &str,
    current_date: &chrono::DateTime<chrono_tz::Tz>,
    current_weather: journey2::core::weather::Weather,
) -> String {
    format!(
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
        utils::enums::get_weather_map()
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
    )
}

pub(crate) fn extract_naive_datetime(
    filename: &std::path::Path,
) -> chrono::ParseResult<chrono::NaiveDateTime> {
    // Ths file name we receive would probably have the format:
    // `YYYY//MM//DD.HH-mm.txt`.
    //
    // Of course, as we allow other options for `open`, we MAY
    // have to accept other file names that have different
    // starting relative paths.
    // Create a PathBuf (for example)

    // Convert PathBuf to string with forward slashes
    let path_string = filename
        .with_extension("")
        .to_str()
        .unwrap()
        .replace("\\", "/");

    chrono::NaiveDateTime::parse_from_str(&path_string, "%Y/%m/%d.%H-%M")
}
