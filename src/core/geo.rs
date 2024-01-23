use crate::core;
use crate::om_api;

/// Wrapper for 3rd party a geo API (currently pointing to Open Meteo)
///
/// # Arguments
///
/// - `query`: A `&str` representing the ra search query for the API.  
///   Processing will be handled by the the 3rd-party API function.
pub(super) fn get_location_info(
    query: &str,
) -> Result<Vec<core::Location>, Box<dyn std::error::Error>> {
    om_api::get_location_info(query)
}
