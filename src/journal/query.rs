pub(crate) mod generic;

/// Queries the user for their current location and returns it as a `String`.
/// Returns a `Result` containing the location string on success,
/// or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Arguments
///
/// This function takes no arguments.
pub(crate) fn query_location_from_user() -> Result<String, Box<dyn std::error::Error>> {
    let location: String = super::query::generic::query_for_string("What's your current location?", 
                                                "<optional, addresses, here>, <city>") ?;
    Ok(location)
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
pub(crate) fn query_desired_datetime_from_user(timezone: &str) -> Result<chrono::DateTime<chrono_tz::Tz>, Box<dyn std::error::Error>> {
    let mut timezone = String::from(timezone);
    loop {
        let current_date = super::calculators::get_current_date_from_tz(&timezone) ?;
        println!("According to your given timezone, it is currently {:?}.", 
            current_date.format("%Y %b %d %H:%M:%S %Z (%:z)").to_string());
        match super::query::generic::query_for_bool("Is this the timezone you want to use?") ? {
            true => { return Ok(current_date) },
            false => { timezone = super::query::generic::query_for_string("What should the timezone be?", "Enter an IANA Timezone ") ?}
        };
    }
}

/// Gets location information from the Open Meteo geocoding API for the given `full_location` string,
/// which should include at least a city/country/island name. The last item in the comma-separated address
/// will be used for the API query.
/// Returns a `Result` containing a vector of `Location` objects on success,
/// or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Arguments
///
/// * `full_location` - A string slice representing the full location to query for.
pub(crate) fn get_location_info_from_api(full_location: &str) -> Result<Vec<super::Location>, Box<dyn std::error::Error>> {
    let city = full_location.split(",")
                                    .last()
                                    .unwrap()
                                    .trim() // Removes trailing spaces
                                    .replace(" ", "%20");   // Makes string URL-ready
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}");
    let api_response_bytes = super::query::generic::call_api(&url)?;
    let api_response_native: super::GeoResult = serde_json::from_slice(&api_response_bytes)?;
    Ok(api_response_native.results)
}

/// Allows the user to choose a location from a vector of `Location` objects.
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
pub(crate) fn choose_location_from_results(results: &[super::Location]) -> Result<&super::Location, Box<dyn std::error::Error>> {
    if results.is_empty() {
        Err("The location with that name found in the database".into())
    } else if results.len() == 1 {
        Ok(&results[0])
    } else {
        println!("Multiple locations found with the name '{}'. Choose one:", results[0].name);
        for (i, result) in results.iter().enumerate() {
            println!("{}. {}", i + 1, result);
        }
        let choice: usize = super::query::generic::query_for_usize("Enter the number of the city you're in")?;
        if choice > 0 && choice <= results.len() {
            Ok(&results[choice - 1])
        } else {
            Err("Invalid choice".into())
        }
    }
}
