use chrono;
use chrono_tz;
use std::io::{Write};

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
    println!("What's the correct timezone? [Enter an IANA Timezone]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok()?;
    input.pop();

    return Some(input);
}

fn handle_date() -> Option<String> {
    let mut tz_as_str: String = "Asia/Manila".to_string();
    loop {
        let current_date = get_current_date(tz_as_str) ?;
        println!("Today is currently {:?}.", current_date);
        match verify_date() {
            Some(true) => { return Some(current_date) },
            Some(false) => { tz_as_str = prompt_for_timezone() ?},
            None => { return None }
        };
    }
}

fn main() {
    let current_date = match handle_date() {
        Some(date) => date,
        None => return, // return early from main if handle_date returns None
    };

    println!("Ends{}", current_date);
}