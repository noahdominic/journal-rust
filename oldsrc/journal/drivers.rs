use std::collections::HashMap;

pub(crate) fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "--Welcome to journal_CLI!--\n\
        This command-line interface app is here to help you document your thoughts,\n\
        experiences, and ideas effortlessly.  Let's get you started :)"
    );
    println!();
    println!();
    println!(
        "Let's start with your default location.  \
        We use your default location to automatically detect your timezome and the weather.\
        This will also be printed in your entries.  To ensure best results, make sure \
        that the last part of your location is somewhere that is specific enough \
        for accurate timezone and weather data."
    );
    let (location_name, _, _, auto_tz_name) = crate::journal::query::user::for_current_location()?;
    let current_timezone =
        crate::journal::query::user::for_desired_datetime(&auto_tz_name)?.timezone();
    println!("{}, {}", location_name, current_timezone);
    Ok(())
}

pub(crate) fn create_new_entry() -> Result<(String, String), Box<dyn std::error::Error>> {
    // Create a HashMap that maps integers to status strings
    let weather_map = HashMap::from([
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
    let (location_name, latitude, longitude, timezone_name) =
        crate::journal::query::user::for_current_location()?;
    let current_date = crate::journal::query::user::for_desired_datetime(&timezone_name)?;
    let current_weather = crate::journal::query::api::for_weather_info(
        &(current_date.format("%Y-%m-%d %H:%M").to_string()),
        latitude.to_string().as_str(),
        longitude.to_string().as_str(),
        &timezone_name,
    )?;
    let output_str = format!(
        "DATE: {}\n\
                            LOCATION: {}\n\
                            \n\
                            Temperature: {} C, feels like {} C, {}.\n\
                            UV Index: {}  Sunrise: {}   Sunset: {}\n\
                            Rain: {}mm\n\
                            Winds: {}km/h {}\n\
                            Pressure: {}hPa\n\
                            Humidity: {}%\n\
                            Visibility: {}km\
                            ",
        current_date.format("%a, %Y %b %d %H:%M:%S %Z (%:z)"),
        location_name,
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

    let file_name = format!("~/journal/{}", current_date.format("%Y/%m/%d"));

    Ok((output_str, file_name))
}
