use std::io::Write;
use curl::easy::Easy;

/// Prints a string in with blank line paddings
macro_rules! lnprint {
    ($($arg:tt)*) => {{
        println!();
        print!($($arg)*);
    }}
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `String`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<String, std::io::Error>` containing the user's input,
/// or `std::io::Error` if an error occurred while reading the input.
pub(crate) fn query_for_string(question: &str, hint: &str) -> Result<String, std::io::Error> {
    // Prompt the user with the question and hint.
    lnprint!("{} [{}]: ", question, hint);
    std::io::stdout().flush()?;
    
    // Read the user's input from the standard input stream.
    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response)?;

    // Return the user's input, with the trailing newline trimmed, as an `Ok`.
    Ok(user_response.trim_end().to_string())
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `String`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<String, std::io::Error>` containing the user's input,
/// or `std::io::Error` if an error occurred while reading the input.
///
/// # Note
/// 
/// This function is dependent on `query_for_string()`.
pub(crate) fn query_for_bool(question: &str) -> Result<bool, std::io::Error> {
    // This is very condensed so here's a more readable summary:
    // - `query_for_string()` is called and passes `question` and `hint`.
    // - The returned string is then checked if it's "yes" or "y".
    // - This yes-ness is wrapped in an `Ok()` and returned
    let answer = query_for_string(question, "y/N")?.trim().to_lowercase();
    Ok(answer == "yes" || answer == "y")
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `usize`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<usize, std::num::ParseIntError>` containing the parsed input as a `usize`,
/// or `std::num::ParseIntError` if the input could not be parsed as a `usize`.
///
/// # Note
/// 
/// This function is dependent on `query_for_string()`.
pub(crate) fn query_for_usize(question: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let user_response = query_for_string(question, "1|2|3...")?;
    Ok(user_response.trim().parse::<usize>()?)
}

/// Makes a generic API call to the given `url`, using the curl crate's Easy API.
/// Returns a `Result` containing the response body as a vector of bytes on success,
/// or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Arguments
///
/// * `url` - A string slice representing the URL to call the API on.
pub(crate) fn call_api_generic(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut api_caller = Easy::new();
    api_caller.url(url)?;
    let mut api_response_bytes = Vec::new();
    {
        let mut transfer = api_caller.transfer();
        transfer.write_function( |received_data| {
            api_response_bytes.extend_from_slice(received_data);
            Ok(received_data.len())
        })?;
        transfer.perform()?;
    }
    Ok(api_response_bytes)
}

/// Queries the user for their current location and returns it as a `String`.
/// Returns a `Result` containing the location string on success,
/// or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Arguments
///
/// This function takes no arguments.
pub(crate) fn query_location_from_user() -> Result<String, Box<dyn std::error::Error>> {
    let location: String = super::query::query_for_string("What's your current location", 
                                                "<optional, addresses, here>, <city>") ?;
    Ok(location)
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
    let api_response_bytes = super::query::call_api_generic(&url)?;
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
        let choice: usize = super::query::query_for_usize("Enter the number of the city you're in")?;
        if choice > 0 && choice <= results.len() {
            Ok(&results[choice - 1])
        } else {
            Err("Invalid choice".into())
        }
    }
}