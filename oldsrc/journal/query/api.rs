/// Gets location information from the Open Meteo geocoding API for the given `full_location` string,
/// which should include at least a city/country/island name. The last item in the comma-separated address
/// will be used for the API query.
/// Returns a `Result` containing a vector of `Location` objects on success,
/// or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Arguments
///
/// * `full_location` - A string slice representing the full location to query for.
pub(crate) fn for_location_info(full_location: &str) -> Result<Vec<crate::journal::Location>, Box<dyn std::error::Error>> {
    let city = full_location.split(",")
                                    .last()
                                    .unwrap()
                                    .trim() // Removes trailing spaces
                                    .replace(" ", "%20");   // Makes string URL-ready
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}");
    let api_response_bytes = crate::journal::query::call_api(&url)?;
    let api_response_native: crate::journal::GeoResult = serde_json::from_slice(&api_response_bytes)?;
    Ok(api_response_native.results)
}


/// Retrieves the current weather conditions (at a specific date and time) 
/// for a given location. 
/// This makes an API call to the Open Meteo API 
/// to get weather data and 
/// then returns a CurrentWeather object that 
/// contains information about the weather at the specified location.
/// 
/// # Arguments
/// * date: A string representing the date and time for which the weather data is to be retrieved. 
///     The date should be in ISO format, i.e., "YYYY-MM-DD HH:MM:SS". 
///     The hour is not optional and should be in 24-hour format.
///     The minute and seconds are never checked.
/// * latitude: A string representing the latitude of the location 
///     for which the weather data is to be retrieved.
///     This should be in decimal form.
/// * longitude: A string representing the longitude of the location 
///     for which the weather data is to be retrieved.
///     This should be in decimal form.
/// * timezone: A string representing the timezone of the location 
///     for which the weather data is to be retrieved. 
///     The timezone should be in "Area/Location" format, such as "Europe/London".
///     See IANA timezone databases for reference.
pub(crate) fn for_weather_info(date: &str, latitude: &str, longitude: &str, timezone: &str) -> Result<crate::journal::Weather, Box<dyn std::error::Error>> {
    // Getting weather info via API below...
    let (current_date_iso, current_hour, timezone_url_ready) = crate::journal::calculators::split_date_time(date, timezone);
    let url = format!("https://api.open-meteo.com/v1/forecast?\
                                latitude={latitude}\
                                &longitude={longitude}\
                                &hourly=\
                                    temperature_2m,\
                                    relativehumidity_2m,\
                                    apparent_temperature,\
                                    rain,\
                                    pressure_msl,\
                                    visibility,\
                                    windspeed_120m,\
                                    winddirection_120m,\
                                    weathercode\
                                &daily=\
                                    sunrise,\
                                    sunset,\
                                    uv_index_max\
                                &timezone={timezone_url_ready}\
                                &start_date={current_date_iso}\
                                &end_date={current_date_iso}");
    let api_response_bytes = crate::journal::query::call_api(&url)?;
    let api_response_native: crate::journal::WeatherResult = serde_json::from_slice(&api_response_bytes)?;
    Ok(crate::journal::Weather {
        temperature:            api_response_native.hourly.temperature_2m[current_hour],
        apparent_temperature:   api_response_native.hourly.apparent_temperature[current_hour],
        weather_code:           api_response_native.hourly.weathercode[current_hour],
        rain:                   api_response_native.hourly.rain[current_hour],
        windspeed:              api_response_native.hourly.windspeed_120m[current_hour],
        winddirection:          api_response_native.hourly.winddirection_120m[current_hour],
        pressure:               api_response_native.hourly.pressure_msl[current_hour],
        humidity:               api_response_native.hourly.relativehumidity_2m[current_hour],
        visibility:             api_response_native.hourly.visibility[current_hour],
        uv_index:               api_response_native.daily.uv_index_max[0],
        sunrise:                api_response_native.daily.sunrise[0].split("T").last().unwrap().to_string(),
        sunset:                 api_response_native.daily.sunset[0].split("T").last().unwrap().to_string()
    })
}