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
pub(crate) fn for_current_location(
) -> Result<(String, f64, f64, String), Box<dyn std::error::Error>> {
    // Uses determine_generic_query to ask user for location
    let full_location: String = crate::journal::query::for_string(
        "What's your current location?",
        "<optional, addresses, specifiers>, <location>",
    )?;

    // Getting location info via API below...
    let api_response_native = crate::journal::query::api::for_location_info(&full_location)?;

    // Let user choose which location they want
    let city_info = crate::journal::query::user::for_location_from_list(&api_response_native)?;

    println!("\nYou are currently in {}.", city_info);

    Ok((
        full_location,
        city_info.latitude,
        city_info.longitude,
        city_info.timezone.clone(),
    ))
}

/// Queries the current date and time that a user wants.  By default, it
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
pub(crate) fn for_desired_datetime(
    timezone: &str,
) -> Result<chrono::DateTime<chrono_tz::Tz>, Box<dyn std::error::Error>> {
    let desired_timezone: chrono_tz::Tz = for_desired_timezone(timezone)?;
    let current_date = crate::journal::calculators::get_current_date_from_tz_as_str(
        &desired_timezone.to_string(),
    )?;
    Ok(current_date)
}

pub(crate) fn for_desired_timezone(
    timezone: &str,
) -> Result<chrono_tz::Tz, Box<dyn std::error::Error>> {
    let mut desired_timezone = String::from(timezone);
    loop {
        let current_date =
            crate::journal::calculators::get_current_date_from_tz_as_str(&desired_timezone)?;
        println!(
            "According to the desired timezone, it is currently {:?}.",
            current_date
                .format("%Y %b %d %H:%M:%S %Z (%:z)")
                .to_string()
        );
        match crate::journal::query::for_bool("Is this the timezone you want to use?")? {
            true => return Ok(current_date.timezone()),
            false => {
                desired_timezone = crate::journal::query::for_string(
                    "What should the timezone be?",
                    "Enter an IANA Timezone ",
                )?
            }
        };
    }
}

/// Allows the user to choose a location fcrom a vector of `Location` objects.
/// If the vector is empty, an error is returned.
/// If the vector contains only one location, that location is returned.
/// If the vector contains multiple locations, the user is prompted to choose one by number.
///
/// Returns a `Result` containing a reference to the chosen `Location` object on success,
/// or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Arguments
///
/// * `results` - A slice of `Location` objects representing the search results to choose from.
pub(crate) fn for_location_from_list(
    results: &[crate::journal::Location],
) -> Result<&crate::journal::Location, Box<dyn std::error::Error>> {
    if results.is_empty() {
        Err("The location with that name found in the database".into())
    } else if results.len() == 1 {
        Ok(&results[0])
    } else {
        println!(
            "There are multiple locations found with the name '{}'. Choose one:",
            results[0].name
        );
        for (i, result) in results.iter().enumerate() {
            println!("{}. {}", i + 1, result);
        }
        let choice: usize =
            crate::journal::query::for_usize("Enter the number of your correct location")?;
        if choice > 0 && choice <= results.len() {
            Ok(&results[choice - 1])
        } else {
            Err("Invalid choice".into())
        }
    }
}
