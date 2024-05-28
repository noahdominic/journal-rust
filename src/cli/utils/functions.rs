use crate as journey2;
use crate::cli::utils;

pub(crate) fn is_journal_initialised() -> Result<bool, journey2::core::file::FileError> {
    let is_journal_initialised = journey2::core::file::is_config_file_exists()?;

    if !is_journal_initialised {
        println!(
            "Oops!  Looks like you haven't initialised your journal yet.  Try running `journal init` first."
        );
    }

    Ok(is_journal_initialised)
}

pub(crate) fn generate_preamble() -> Result<String, Box<dyn std::error::Error>> {
    let weather_map = utils::enums::get_weather_map();

    let config_data = journey2::core::file::get_config_from_config_file()?;

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

    Ok(format!(
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
    ))
}
