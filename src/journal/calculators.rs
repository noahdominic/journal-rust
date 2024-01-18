// Copyright 2023  Noah Dominic Miranda Silvio
// Licensed under the EUPL v1.2

/**
 * Generates the path to today's entry based on the provided base directory.
 *
 * This function takes a `base_dir` as input and generates the path to today's entry
 * by concatenating the base directory with the current year, month, and day.
 * It returns a `Result` containing the generated path as a `String` on success,
 * or a boxed `dyn std::error::Error` on failure.
 *
 * # Returns
 *
 * - `Result<String, Box<dyn std::error::Error>>`: A `Result` that contains the generated path to today's entry
 *   as a `String` on success, or an error on failure.
 *
 * # Example
 *
 * ```no_run
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     let base_directory = PathBuf::from("/path/to/entries");
 *     let today_entry_path: String = get_path_to_todays_entry(base_directory)?;
 *     println!("Today's entry path: {}", today_entry_path);
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn get_path_to_todays_entry() -> Result<String, Box<dyn std::error::Error>> {
    let base_dir = crate::journal::file::get_base_dir()?;

    let (_, _, _, timezone, _) = crate::journal::file::get_config_details()?;

    // Get the current date
    let current_date = get_current_date_from_tz_as_str(&timezone)?;

    // Create the path to today's entry
    let today_entry_path = format!(
        "{}/{}",
        base_dir.to_string_lossy(),
        current_date.format("%Y/%m/%d.%H-%M.txt")
    );

    // Return the path as a String
    Ok(today_entry_path)
}

pub(crate) fn get_all_path_to_todays_entry() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let base_dir = crate::journal::file::get_base_dir()?;

    let (_, _, _, timezone, _) = crate::journal::file::get_config_details()?;

    // Get the current date
    let current_date = get_current_date_from_tz_as_str(&timezone)?;

    // Create the path to today's entry
    let this_months_dir = format!(
        "{}/{}",
        base_dir.to_string_lossy(),
        current_date.format("%Y/%m")
    );

    let day = format!("{}", current_date.format("%d"));

    let entries: Vec<String> = std::fs::read_dir(this_months_dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let file_name_osstring = e.file_name();
                let file_name = file_name_osstring.to_string_lossy();
                (file_name.starts_with(&day) && file_name.ends_with(".txt"))
                    .then_some(file_name.to_string())
            })
        })
        .collect();

    dbg!(&entries);

    // Return the path as a String
    Ok(entries)
}

/// Returns the cardinal direction as a string based on the given degrees.
///
/// # Arguments
///
/// * `degrees` - The degrees to convert to a cardinal direction. Should be between 0 and 360.
///
/// # Example
///
/// ```
/// let direction = get_direction(45.0);
/// assert_eq!(direction, "NE");
/// ```
pub(crate) fn get_direction(degrees: f64) -> String {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    return String::from(directions[index]);
}

/// Returns the current date and time in the specified timezone.
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
/// A `Result` containing a
/// `chrono::DateTime<chrono_tz::Tz>` of the current date and time in the specified timezone
/// or a `chrono_tz::ParseError` when `tz_as_str` is an invalid IANA timezone name.
///
/// # Examples
///
/// ```
/// use chrono::DateTime;
/// use chrono_tz::Tz;
///
/// let tz_str = "America/New_York".to_string();
/// let current_date: DateTime<Tz> = get_current_date(tz_str).unwrap();
/// println!("{}", current_date);
/// ```
pub(crate) fn get_current_date_from_tz_as_str(
    tz_as_str: &str,
) -> Result<chrono::DateTime<chrono_tz::Tz>, chrono_tz::ParseError> {
    let timezone: chrono_tz::Tz = tz_as_str.parse()?;
    return Ok(chrono::Utc::now().with_timezone(&timezone));
}

/**
 * Splits a combined date and time string and prepares a URL-ready timezone string.
 *
 * This function takes a `date` and a `timezone` as input, and splits the combined date and time string
 * to extract the ISO-formatted date and the hour. It also prepares the `timezone` string to be URL-ready
 * by replacing slashes with "%2F". The function returns a tuple containing the extracted date, hour,
 * and the URL-ready timezone string.
 *
 * # Arguments
 *
 * - `date`: A string representing the combined date and time.
 * - `timezone`: A string representing the timezone.
 *
 * # Returns
 *
 * - `(String, usize, String)`: A tuple containing the extracted date as a string, the extracted hour as a usize,
 *   and the URL-ready timezone string.
 *
 * # Example
 *
 * ```no_run
 * fn main() {
 *     let date = "2023-08-21 13:45:00";
 *     let timezone = "America/New_York";
 *     let (extracted_date, extracted_hour, url_ready_timezone) = split_date_time(date, timezone);
 *     println!("Extracted Date: {}", extracted_date);
 *     println!("Extracted Hour: {}", extracted_hour);
 *     println!("URL-Ready Timezone: {}", url_ready_timezone);
 * }
 * ```
 */
pub(crate) fn preprocess_datetime_for_url(date: &str, timezone: &str) -> (String, usize, String) {
    // Split the combined date and time string
    let mut date_iter = date.split_whitespace();
    let current_date_iso = date_iter.next().unwrap().trim().to_string();
    let current_hour = date_iter
        .next()
        .unwrap()
        .split(":")
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    // Prepare the timezone string for URLs
    let timezone_url_ready = timezone.replace("/", "%2F");

    // Return the extracted date, hour, and URL-ready timezone
    (current_date_iso, current_hour, timezone_url_ready)
}
