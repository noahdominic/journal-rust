/**
 * Generates the path to today's entry based on the provided base directory.
 *
 * This function takes a `base_dir` as input and generates the path to today's entry
 * by concatenating the base directory with the current year, month, and day.
 * It returns a `Result` containing the generated path as a `String` on success,
 * or a boxed `dyn std::error::Error` on failure.
 *
 * # Arguments
 *
 * - `base_dir`: A `PathBuf` representing the base directory for the entries.
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
pub(crate) fn get_path_to_todays_entry(base_dir: std::path::PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    // Get the current date
    let current_date = chrono::Local::now().date_naive();

    // Create the path to today's entry
    let today_entry_path = format!(
        "{}/{}",
        base_dir.to_string_lossy(),
        current_date.format("%Y/%m/%d")
    );

    // Return the path as a String
    Ok(today_entry_path)
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