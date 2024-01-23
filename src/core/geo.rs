use crate::core;
use crate::om_api;

/// Wrapper for 3rd party a geo API (currently pointing to Open Meteo)
///
/// # Arguments
///
/// - `query`: A `&str` representing the ra search query for the API.  
///   Processing will be handled by the the 3rd-party API function.
///
/// # Note
///
/// Currently, this function relies on `core::GeoResult`, which causes a
/// dependence on Open Meteo's API.  This must be changed.
pub(crate) fn get_location_info(
    query: &str,
) -> Result<Vec<core::Location>, Box<dyn std::error::Error>> {
    let api_response_bytes = om_api::get_location_info(query)?;
    let api_response_native: core::GeoResult = serde_json::from_slice(&api_response_bytes)?;
    Ok(api_response_native.results)
}
