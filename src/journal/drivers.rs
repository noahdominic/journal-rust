use std::io::Write;
use curl::easy;
use std::collections::HashMap;
use isocountry;

/// Prints a string in with blank line paddings
macro_rules! lnprint {
    ($($arg:tt)*) => {{
        println!();
        print!($($arg)*);
    }}
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `String`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<String, std::io::Error>` containing the user's input,
/// or `std::io::Error` if an error occurred while reading the input.
/// 
fn query_for_string(question: &str, hint: &str) -> Result<String, std::io::Error> {
    // Prompt the user with the question and hint.
    lnprint!("{} [{}]: ", question, hint);
    std::io::stdout().flush().expect("Error: Flush failed!");
    
    // Read the user's input from the standard input stream.
    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response).expect("Error: Reading failed!");

    // Return the user's input, with the trailing newline trimmed, as an `Ok`.
    Ok(user_response.trim_end().to_string())
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `String`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<String, std::io::Error>` containing the user's input,
/// or `std::io::Error` if an error occurred while reading the input.
///
/// # Note
/// 
/// This function is dependent on `query_for_string()`.
/// 
fn query_for_bool(question: &str, hint: &str) -> Result<bool, std::io::Error> {
    // This is very condensed so here's a more readable summary:
    // - `query_for_string()` is called and passes `question` and `hint`.
    // - The returned string is then checked if it's "yes" or "y".
    // - This yes-ness is wrapped in an `Ok()` and returned
    let answer = query_for_string(question, hint)?.trim().to_lowercase();
    Ok(answer == "yes" || answer == "y")
}

/// Determines the current date and time that a user wants.  By default, it
/// chooses the time at the timezone in the argument
/// 
/// # Arguments
/// 
/// * `timezone`: A `String` object representing a IANA timezone to be used to determine the current time
/// 
/// # Returns
/// 
/// A `Result` that contains either 
/// - a `chrono::DateTime<chrono_tz::Tz>` value representing 
///     the current date and time in the given timezone 
/// - or an `Box<dyn std::error::Error>` if something went wrong while determining the date and time.
/// 
fn choose_desired_datetime(timezone: &str) -> Result<chrono::DateTime<chrono_tz::Tz>, Box<dyn std::error::Error>> {
    let mut tz_as_string: String = timezone.to_string();
    loop {
        let current_date = super::calculators::get_current_date_from_tz(tz_as_string) ?;
        println!("According to your given timezone, it is currently {:?}.", 
            current_date.format("%Y %b %d %H:%M:%S %Z (%:z)").to_string());

        match query_for_bool("Is this the timezone you want to use?", "y/N") ? {
            true => { return Ok(current_date) },
            false => { tz_as_string = query_for_string("What should the timezone be?", "Enter an IANA Timezone ") ?}
        };
    }
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
fn determine_location_info() -> Result<(String, f64, f64, String), Box<dyn std::error::Error>>  {
    // Uses determine_generic_query to ask user for location
    let full_location: String = query_for_string("What's your current location", 
                                                "<optional, addresses, here>, <city>") ?;

    // Getting city info via API below...
    let mut api_caller = easy::Easy::new();
    let city = full_location.split(",")
                                    .last()
                                    .unwrap()
                                    .trim() // Removes trailing spaces
                                    .replace(" ", "%20");   // Makes string URL-ready
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}");
    println!("{}", url);
    // set the URL in api_caller
    api_caller.url(url.as_str()) ?;
    let mut api_response_bytes = Vec::new();
    // NOTE     I'm honestly confused why scoping is necessary here.  
    //          Something to do about Rust's borrowing rules.
    {
        let mut transfer = api_caller.transfer();
        // set the write_function in api_caller
        transfer.write_function( |received_data| {
            api_response_bytes.extend_from_slice(received_data);
            Ok(received_data.len())
        })?;
        // perform the API call
        transfer.perform() ?;
    }

    // Translate what we got from the server to native Rust structs
    let api_response_native: super::GeoResult = serde_json::from_slice(&api_response_bytes)?;

    // TODO     Choosing the 0th item is completely arbitrary.
    //          We should find a way to let the use pick. 
    //          There are multiple cities with the same name.
    
    let city_info = &api_response_native.results[0];

    println!("\nYou are currently in {}, {} ({}, {}) in {}.", 
            city_info.name,
            isocountry::CountryCode::for_alpha2(&(city_info.country_code)) ?,
            city_info.latitude,
            city_info.longitude,
            city_info.timezone);

    Ok((full_location,
        city_info.latitude,
        city_info.longitude,
        city_info.timezone.clone()))
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
fn determine_weather(date: &str, 
                     latitude: &str, 
                     longitude: &str, 
                     timezone: &str) -> Result<super::Weather, Box<dyn std::error::Error>> {

    // Getting weather info via API below...
    let mut api_caller = easy::Easy::new();

    let mut date_iter = date.split(" ");

    let current_date_iso = date_iter
                            .next()
                            .unwrap()
                            .trim();
    let current_hour = date_iter
                            .next()
                            .unwrap()
                            .split(":").next().unwrap().parse::<usize>().unwrap();

    let timezone_url_ready = timezone.replace("/", "%2F");

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
    
    // set the URL in api_caller
    api_caller.url(url.as_str()) ?;
    let mut api_response_bytes = Vec::new();
    // NOTE  I'm honestly confused why scoping is necessary here.  Something to do about Rust's borrowing rules.
    {
        let mut transfer = api_caller.transfer();
        // set the write_function in api_caller
        transfer.write_function(|received_data| {
            api_response_bytes.extend_from_slice(received_data);
            Ok(received_data.len())
        })?;
        // perform the API call
        transfer.perform()?;
    }

    // Translate what we got from the server to native Rust structs
    let api_response_native: super::WeatherResult = serde_json::from_slice(&api_response_bytes) ?;

    Ok(super::Weather {
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
        }
    )
}

pub(crate) fn journal_init_driver() -> Result<(String, String), Box<dyn std::error::Error>> {
    // Create a HashMap that maps integers to status strings
    // COULDDO  There should be a better way to execute this.  A HashMap seems excessive.
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

    let (location, latitude, longitude, timezone) = determine_location_info() ?;
    let current_date = choose_desired_datetime(timezone.as_str())?;
    let current_weather = determine_weather(current_date.format("%Y-%m-%d %H:%M").to_string().as_str(), 
                                                    latitude.to_string().as_str(), 
                                                    longitude.to_string().as_str(), 
                                                    timezone.as_str()) ?;

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

    //println!("\n\n{}", output_str);

    Ok((output_str, file_name))
}