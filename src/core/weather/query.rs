use crate as journey2;

pub(crate) fn query_current_weather(
    date: &str,
    latitude: &str,
    longitude: &str,
    timezone: &str,
) -> Result<journey2::core::weather::Weather, Box<dyn std::error::Error>> {
    let (current_date_iso, current_hour) = journey2::core::chrono::preprocess_datetime_for_url(date);
    let timezone_url_ready = journey2::core::chrono::preprocess_timezone_for_url(timezone);

    // Calling the API now...
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?\
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
                                &end_date={current_date_iso}"
    );
    let api_response_bytes = journey2::om_api::call_api(&url)?;

    // Deserialising the data received
    let api_response_native: journey2::core::weather::WeatherResult =
        serde_json::from_slice(&api_response_bytes)?;

    Ok(journey2::core::weather::Weather {
        temperature: api_response_native.hourly.temperature_2m[current_hour],
        apparent_temperature: api_response_native.hourly.apparent_temperature[current_hour],
        weather_code: api_response_native.hourly.weathercode[current_hour],
        rain: api_response_native.hourly.rain[current_hour],
        windspeed: api_response_native.hourly.windspeed_120m[current_hour],
        winddirection: api_response_native.hourly.winddirection_120m[current_hour],
        pressure: api_response_native.hourly.pressure_msl[current_hour],
        humidity: api_response_native.hourly.relativehumidity_2m[current_hour],
        visibility: api_response_native.hourly.visibility[current_hour],
        uv_index: api_response_native.daily.uv_index_max[0],
        sunrise: api_response_native.daily.sunrise[0]
            .split("T")
            .last()
            .unwrap()
            .to_string(),
        sunset: api_response_native.daily.sunset[0]
            .split("T")
            .last()
            .unwrap()
            .to_string(),
    })
}