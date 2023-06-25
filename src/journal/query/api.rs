pub(crate) fn for_location_info(
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
