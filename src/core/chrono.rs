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
pub(crate) fn get_current_date_from_tz_as_str(
    tz_as_str: &str,
) -> Result<chrono::DateTime<chrono_tz::Tz>, chrono_tz::ParseError> {
    let timezone: chrono_tz::Tz = tz_as_str.parse()?;

    Ok(chrono::Utc::now().with_timezone(&timezone))
}
