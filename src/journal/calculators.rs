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
    let directions = ["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"];
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
pub(crate) fn get_current_date_from_tz(tz_as_str: &str) -> Result<chrono::DateTime<chrono_tz::Tz>, chrono_tz::ParseError> {
    let timezone: chrono_tz::Tz = tz_as_str.parse() ?;    
    return Ok(chrono::Utc::now().with_timezone(&timezone));
}

pub(crate) fn split_date_time(date: &str, timezone: &str) -> (String, usize, String) {
    let mut date_iter = date.split_whitespace();
    let current_date_iso = date_iter.next().unwrap().trim().to_string();
    let current_hour = date_iter.next().unwrap().split(":").next().unwrap().parse::<usize>().unwrap();
    let timezone_url_ready = timezone.replace("/", "%2F");
    (current_date_iso, current_hour, timezone_url_ready)
}
