use std::collections::HashMap;

pub(crate) fn journal_init_driver() -> Result<(String, String), Box<dyn std::error::Error>> {
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
    let (location, latitude, longitude, timezone) = get_location_info()?;
    let current_date = super::query::query_desired_datetime_from_user(&timezone)?;
    let current_weather = super::query::get_weather_info_from_api(&(current_date.format("%Y-%m-%d %H:%M").to_string()), 
                                                                    latitude.to_string().as_str(), 
                                                                    longitude.to_string().as_str(), 
                                                                    &timezone)?;
    let output_str = format!("DATE: {}\n\
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
                            location,
                            current_weather.temperature,
                            current_weather.apparent_temperature,
                            weather_map.get(&current_weather.weather_code).unwrap_or(&"Unknown conditions"),
                            current_weather.uv_index,
                            current_weather.sunrise,
                            current_weather.sunset,
                            current_weather.rain,
                            current_weather.windspeed,
                            super::calculators::get_direction(current_weather.winddirection),
                            current_weather.pressure,
                            current_weather.humidity,
                            current_weather.visibility/1000.0);

    let file_name = format!("~/journal/{}", current_date.format("%Y/%m/%d"));

    Ok((output_str, file_name))
}

/// Prompts the user to input their current location and queries a geocoding API 
/// to retrieve location information about the city.
///
/// # Returns
/// 
/// A `Result` object containing a tuple with the following elements:
/// - `full_location`: A string representing the user's original input location
/// - `latitude`: A floating-point number representing the latitude of the city
/// - `longitude`: A floating-point number representing the longitude of the city
/// - `timezone`: A string representing the timezone of the city
///
/// # Errors
///
/// Returns an error in the form of a string slice if an error occurs while
/// processing the user's input location or if the geocoding API does not return any results.
fn get_location_info() -> Result<(String, f64, f64, String), Box<dyn std::error::Error>>  {
    // Uses determine_generic_query to ask user for location
    let full_location: String = super::query::query_location_from_user()?;

    // Getting location info via API below...
    let api_response_native = super::query::get_location_info_from_api(&full_location)?;

    // Let user choose which location they want
    let city_info = super::query::choose_location_from_results(&api_response_native) ?;
    
    println!("\nYou are currently in {}.", 
            city_info);

    Ok((full_location,
        city_info.latitude,
        city_info.longitude,
        city_info.timezone.clone()))
}

