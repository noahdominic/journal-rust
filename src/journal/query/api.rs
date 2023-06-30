/// Retrieves location information for a given full location string by making a request to a geocoding API.
///
/// The function extracts the city from the `full_location` string and performs a geocoding API call to obtain location information.
///
/// # Arguments
///
/// * `full_location` - A string representing the full location, including the city, state, and country.
///
/// # Returns
///
/// Returns a `Result` containing the location information as a vector of `Location` structs if successful, or an error of type `Box<dyn std::error::Error>` if any of the operations encounter an error.
///
/// # Errors
///
/// The function can return an error if any of the following operations fail:
/// * Extracting the city from the `full_location` string due to an invalid format.
/// * Making the API call to the geocoding API encounters an error.
/// * Deserializing the API response into the `GeoResult` type fails.
///
/// # Example
///
/// ```
/// use crate::journal::Location;
///
/// let full_location = "New York, NY, USA";
/// let result = get_location_info(full_location);
///
/// match result {
///     Ok(locations) => {
///         for location in locations {
///             println!("City: {}", location.city);
///             println!("State: {}", location.state);
///             println!("Country: {}", location.country);
///             println!("Latitude: {}", location.latitude);
///             println!("Longitude: {}", location.longitude);
///             println!();
///         }
///     }
///     Err(error) => {
///         eprintln!("Error occurred while retrieving location information: {}", error);
///         // Handle the error accordingly
///     }
/// }
/// ```
pub(crate) fn get_location_info(
    full_location: &str,
) -> Result<Vec<crate::journal::Location>, Box<dyn std::error::Error>> {
    let city = full_location
        .rsplit(',')
        .next()
        .ok_or("Invalid full_location format")?
        .trim()
        .replace(" ", "%20");
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}");
    let api_response_bytes = crate::journal::query::call_api(&url)?;
    let api_response_native: crate::journal::GeoResult =
        serde_json::from_slice(&api_response_bytes)?;
    Ok(api_response_native.results)
}
