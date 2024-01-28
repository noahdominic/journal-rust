//! Contains functions concerning geolocation


use crate::core;
use crate::om_api;

/// Wrapper for 3rd party a geo API (currently pointing to Open Meteo)
///
/// # Arguments
///
/// - `query`: A `&str` representing the search query for the API.
///   Processing will be handled by a 3rd-party API function.
///
/// # Note
///
/// Currently, this function relies on `core::GeoResult`, which is based
/// on the OpenMeteo API's response.
pub(crate) fn get_location_info(
    query: &str,
) -> Result<Vec<core::Location>, core::JourneyCoreError> {
    let api_response_bytes = om_api::get_location_info(query)?;
    let api_response_native: core::GeoResult = serde_json::from_slice(&api_response_bytes)?;
    Ok(api_response_native.results)
}
