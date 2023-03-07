use chrono;
use chrono_tz;
use std::io::Write;
use curl::easy;
use serde::Deserialize;
use serde_json;

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

/// Returns the current date and time as a string formatted as `YYYY-MM-DD HH:MM:SS TZ`.
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
fn get_current_date(tz_as_str: String) -> Option<String> {
    let timezone: chrono_tz::Tz = match tz_as_str.parse() {
        Ok(timezone) => timezone,
        _ => return None,
    };
    let now = chrono::Utc::now().with_timezone(&timezone);
    let date_str = now.format("%Y-%m-%d %H:%M:%S %Z").to_string();
    
    return Some(date_str);
}

fn verify_date() -> Option<bool> {
    let mut input = String::new();
    print!("Is this correct? [y/N/(q)uit]: ");
    std::io::stdout().flush().expect("Error: Flush failed!");

    std::io::stdin().read_line(&mut input).ok()?;

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => Some(true),
        "n" | "no" => Some(false),
        _ => None
    }    
}

fn prompt_for_timezone() -> Option<String> {
    print!("What's the correct timezone? [Enter an IANA Timezone]: ");
    std::io::stdout().flush().expect("Error: Flush failed!");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok()?;
    input.pop();

    return Some(input);
}

fn handle_date(timezone: String) -> Option<String> {
    let mut tz_as_str: String = timezone.to_string();
    loop {
        let current_date = get_current_date(tz_as_str) ?;
        println!("According to your location, it is currently {:?}.", current_date);
        match verify_date() {
            Some(true) => { return Some(current_date) },
            Some(false) => { tz_as_str = prompt_for_timezone() ?},
            None => { return None }
        };
    }
}

fn handle_generic_query(question: String, hint: String) -> Option<String> {
    let mut input = String::new();
    
    print!("{} [{}]: ", question, hint);
    std::io::stdout().flush().expect("Error: Flush failed!");

    std::io::stdin().read_line(&mut input).ok()?;
    input.pop();

    return Some(input);
}

fn handle_location() -> Result<(String, String, f64, f64, String, String), Box<dyn std::error::Error>>  {
    let location: String = handle_generic_query("What's your current location".to_string(), 
                                                "<optional, addresses, here>, <city>".to_string())
                           .ok_or("Error: Something happened while trying to process location. ") ?;

    println!(">>> Location: {}", location);

    // Getting city info via API below...
    let city = location.split(",")
                               .last()
                               .unwrap()
                               .trim()
                               .replace(" ", "%20");   
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}");

    let mut caller = easy::Easy::new();
    caller.url(url.as_str()) ?;
    let mut data = Vec::new();
    {
        let mut transfer = caller.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    let response: GeoResult = serde_json::from_slice(&data) ?;

    if response.results.is_empty() {
        return Err("No results found".into());
    }

    let city_info = response.results;

    println!(">>> You're currently in {}, {} ({}, {}) in {}", 
             city_info[0].name, 
             city_info[0].country,
             city_info[0].latitude,
             city_info[0].longitude,
             city_info[0].timezone);
    Ok((location,
        city_info[0].name.clone(), 
        city_info[0].latitude,
        city_info[0].longitude,
        city_info[0].country.clone(),
        city_info[0].timezone.clone()))
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (location, name, latitude, longitude, country, timezone) = handle_location() ?;
    let current_date = handle_date(timezone).ok_or("Error: Somthing happened while trying to pro date.") ?;

    println!(">>> Date: {}", current_date);


    Ok(())
}