/*******************************************************************************
 * Copyright (c) 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved
 * Licensed under the EUPL v1.2
 ******************************************************************************/

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

pub fn preprocess_datetime_for_url(date: &str) -> (String, usize) {
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

    // Return the extracted date, hour, and URL-ready timezone
    (current_date_iso, current_hour)
}

pub fn preprocess_timezone_for_url(timezone: &str) -> String {
    // TBH this feels so silly, but this makes things more semantic, i.e. easy to read
    timezone.replace("/", "%2F")
}

