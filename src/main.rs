use chrono;
use chrono_tz;
use std::io::Write;
use std::collections::HashMap;
use curl::easy;
use serde::Deserialize;
use serde_json;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use dirs::home_dir;

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
    timezone: String,
    country: String
}

#[derive(Debug, Deserialize)]
struct GeoResult {
    results: Vec<Location>
}

#[derive(Debug, Deserialize)]
struct DailyWeather {
    sunrise: Vec<String>,
    sunset: Vec<String>,
    uv_index_max: Vec<f64>
}

#[derive(Debug, Deserialize)]
struct HourlyWeather {
    temperature_2m: Vec<f64>,
    relativehumidity_2m: Vec<f64>,
    apparent_temperature: Vec<f64>,
    rain: Vec<f64>,
    pressure_msl: Vec<f64>,
    visibility: Vec<f64>,
    windspeed_120m: Vec<f64>,
    winddirection_120m: Vec<f64>,
    weathercode: Vec<usize>
}

#[derive(Debug, Deserialize)]
struct WeatherResult {
    hourly: HourlyWeather,
    daily: DailyWeather
}

fn get_direction(degrees: f64) -> String {
    let directions = ["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"];
    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    return String::from(directions[index]);
}


/// Takes an IANA timezone and returns the current date and time as a string formatted as `YYYY-MM-DD HH:MM:SS TZ`.
///
/// # Arguments
///
/// * `tz_as_str` - A string representing the timezone to use for formatting the date.
///   This should be in the format `Area/Location`, where `Area` is the name of a time zone
///   area (e.g., "America", "Europe", "Asia"), and `Location` is the name of a specific
///   location within that area (e.g., "New_York", "London", "Tokyo").
///   This str format is based on the `chrono_tz` crate which is further based on IANA TZs
///
/// # Returns
///
/// A string representing the current date and time in the specified timezone, formatted as
/// `YYYY-MM-DD HH:MM:SS TZ`.
///
fn get_current_date(tz_as_str: String) -> Option<chrono::DateTime<chrono_tz::Tz>> {
    let timezone: chrono_tz::Tz = match tz_as_str.parse() {
        Ok(timezone) => timezone,
        _ => return None,
    };

    let now = chrono::Utc::now().with_timezone(&timezone);
    
    return Some(now);
}

/// This asks the user if the automatic date is correct
fn verify_date() -> Option<bool> {
    let mut input = String::new();

    print!("\nIs this correct? [y/N/(q)uit]: ");
    std::io::stdout().flush().expect("Error: Flush failed!");

    std::io::stdin().read_line(&mut input).ok()?;

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => Some(true),
        "n" | "no" => Some(false),
        _ => None
    }    
}

/// Asks the user to enter a correct timezone and returns their input as an `Option<String>`.
///
/// # Returns
///
/// An `Option<String>` containing the user's input, or `None` if an error occurred while
/// reading the input.
/// 
fn ask_for_correct_timezone() -> Option<String> {
    // Prompt the user to enter a timezone.
    print!("What's the correct timezone? [Enter an IANA Timezone]: ");
    std::io::stdout().flush().expect("Error: Flush failed!");

    // Read the user's input from the standard input stream.
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok()?;

    // Remove the trailing newline character from the input.
    input.pop();

    // Return the user's input as an `Option<String>`.
    Some(input)
}


/// Handles the figuring-out of the date, such as verifying and correcting.
/// 
/// # Arguments
/// 
/// * `timezone`: A `String` object representing a IANA timezone to be used to determine the current time
/// 
/// # Returns
/// 
/// And `Option<String>` containing the current date, or `None` if an error occured during verification.
/// 
fn determine_date(timezone: &str) -> Option<chrono::DateTime<chrono_tz::Tz>> {
    let mut tz_as_str: String = timezone.to_string();
    loop {
        let current_date = get_current_date(tz_as_str) ?;
        println!("According to your given location, it is currently {:?}.", current_date.format("%Y-%m-%d %H:%M:%S %Z (%:z)").to_string());
        match verify_date() {
            Some(true) => { return Some(current_date) },
            Some(false) => { tz_as_str = ask_for_correct_timezone() ?},
            None => { return None }
        };
    }
}

/// Determines a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream.
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Option<String>` containing the user's input, or `None` if an error occurred while
/// reading the input.
///
fn determine_generic_query(question: &str, hint: &str) -> Option<String> {
    let mut input = String::new();

    // Prompt the user with the question and hint.
    print!("{} [{}]: ", question, hint);
    std::io::stdout().flush().expect("Error: Flush failed!");

    // Read the user's input from the standard input stream.
    std::io::stdin().read_line(&mut input).ok()?;

    // Remove the trailing newline character from the input.
    input.pop();

    // Return the user's input as an `Option<String>`.
    Some(input)
}

/// Prompts the user to input their current location and queries a geocoding API 
/// to retrieve location information about the city.
///
/// Returns a `Result` object containing a tuple with the following elements:
/// - `location`: A string representing the user's original input location
/// - `city_name`: A string representing the name of the city where the user is located
/// - `latitude`: A floating-point number representing the latitude of the city
/// - `longitude`: A floating-point number representing the longitude of the city
/// - `country`: A string representing the name of the country where the city is located
/// - `timezone`: A string representing the timezone of the city
///
/// # Errors
///
/// Returns an error in the form of a string slice if an error occurs while
/// processing the user's input location or if the geocoding API does not return any results.
/// 
fn determine_location_info() -> Result<(String, f64, f64, String), Box<dyn std::error::Error>>  {
    // Uses determine_generic_query to ask user for location
    let full_location: String = determine_generic_query("What's your current location", 
                                                   "<optional, addresses, here>, <city>")
                           .ok_or("Error: Something happened while trying to process location. ") ?;

    // Getting city info via API below...
    let mut api_caller = easy::Easy::new();
    let city = full_location.split(",")
                                    .last()
                                    .unwrap()
                                    .trim() // Removes trailing spaces
                                    .replace(" ", "%20");   // Makes string URL-ready
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}");
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
    let api_response_native: GeoResult = serde_json::from_slice(&api_response_bytes) ?;

    if api_response_native.results.is_empty() { // This is necessary because conversion _may_ be successful and the response is empty.
        return Err("No results found".into());
    }

    // This is what we need!
    let city_info = &api_response_native.results[0];

    println!("\nYou are currently in {}, {} ({}, {}) in {}.", 
             city_info.name, 
             city_info.country,
             city_info.latitude,
             city_info.longitude,
             city_info.timezone);

    Ok((full_location,
        city_info.latitude,
        city_info.longitude,
        city_info.timezone.clone()))
}

fn determine_weather(date: &str, latitude: &str, longitude: &str, timezone: &str) -> Result<(f64, f64, usize, f64, f64, f64, f64, f64, f64, f64, String, String), Box<dyn std::error::Error>> {
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

    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&hourly=temperature_2m,relativehumidity_2m,apparent_temperature,rain,pressure_msl,visibility,windspeed_120m,winddirection_120m,weathercode&daily=sunrise,sunset,uv_index_max&timezone={timezone_url_ready}&start_date={current_date_iso}&end_date={current_date_iso}");
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
    let api_response_native: WeatherResult = serde_json::from_slice(&api_response_bytes) ?;

    let temperature = api_response_native.hourly.temperature_2m[current_hour];
    let apparent_temperature = api_response_native.hourly.apparent_temperature[current_hour];
    let weather_code = api_response_native.hourly.weathercode[current_hour];
    let rain = api_response_native.hourly.rain[current_hour];
    let windspeed = api_response_native.hourly.windspeed_120m[current_hour];
    let winddirection = api_response_native.hourly.winddirection_120m[current_hour];
    let pressure = api_response_native.hourly.pressure_msl[current_hour];
    let humidity = api_response_native.hourly.relativehumidity_2m[current_hour];
    let visibility = api_response_native.hourly.visibility[current_hour];
    let uv_index = api_response_native.daily.uv_index_max[0];
    let sunrise = api_response_native.daily.sunrise[0].split("T").last().unwrap().to_string();
    let sunset = api_response_native.daily.sunset[0].split("T").last().unwrap().to_string();
    
    Ok((temperature, apparent_temperature, weather_code, rain, windspeed, winddirection, pressure, humidity, visibility, uv_index, sunrise, sunset))
}


fn journal_init_driver() -> Result<(String, String), Box<dyn std::error::Error>> {
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

    let (location, latitude, longitude, timezone) = determine_location_info() ?;
    let current_date = determine_date(timezone.as_str()).ok_or("Error: Somthing happened while trying to pro date.") ?;

    let (temperature, apparent_temperature, weather_code, rain, windspeed, winddirection, pressure, humidity, visibility, uv_index, sunrise, sunset) = determine_weather(current_date.format("%Y-%m-%d %H:%M").to_string().as_str(), latitude.to_string().as_str(), longitude.to_string().as_str(), timezone.as_str()) ?;

    let output_str = format!("DATE: {}\n\
                            LOCATION: {location}\n\
                            \n\
                            Temperature: {temperature} C, feels like {apparent_temperature} C, {}.\n\
                            UV Index: {uv_index}  Sunrise: {sunrise}   Sunset: {sunset}\n\
                            Rain: {rain}mm\n\
                            Winds: {windspeed}km/h {}\n\
                            Pressure: {pressure}hPa\n\
                            Humidity: {humidity}%\n\
                            Visibility: {}km\
                            ", 
                            current_date.format("%a, %Y %b %d %H:%M:%S %Z (%:z)"),
                            weather_map.get(&weather_code).unwrap_or(&"Unknown conditions"),
                            get_direction(winddirection),
                            visibility/1000.0);

    let file_name = format!("~/journal/{}", current_date.format("%Y/%m/%d"));

    //println!("\n\n{}", output_str);

    Ok((output_str, file_name))
}

fn main() -> Result <(), Box<dyn std::error::Error>>{
    let (preamble, file_name) = journal_init_driver() ?;

    let file_path = Path::new(&file_name);

    // Expand the tilde to the home directory
    let file_path = if let Some(home_dir) = home_dir() {
        PathBuf::from(home_dir).join(file_path.strip_prefix("~").unwrap())
    } else {
        file_path.to_owned()
    };

    println!("\n\nThis will print in: {}\n{}", file_path.display(), preamble);

    // Create directories recursively if needed
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    // Open the file in append mode if it already exists, otherwise create it
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .unwrap();

    // Write the string to the file
    writeln!(&file, "{}", preamble).unwrap();
    Ok(())
}