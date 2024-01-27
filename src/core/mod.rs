pub mod geo;
pub mod str_man;

use serde;

/// Struct for location details
///
/// # Note
///
/// This structure is heavily dependent on Open Meteo's API structure.  They
/// must someday be decoupled to decrease `core`'s dependence on `om_api`.
#[derive(Debug, serde::Deserialize, Clone)]
pub(crate) struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
    timezone: String,
    country_code: String,
    admin1: Option<String>,
    admin2: Option<String>,
    admin3: Option<String>,
    admin4: Option<String>,
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let admin1 = self
            .admin1
            .as_ref()
            .map_or("".to_string(), |x| format!("{}, ", x));
        let admin2 = self
            .admin2
            .as_ref()
            .map_or("".to_string(), |x| format!("{}, ", x));
        let admin3 = self
            .admin3
            .as_ref()
            .map_or("".to_string(), |x| format!("{}, ", x));
        let admin4 = self
            .admin4
            .as_ref()
            .map_or("".to_string(), |x| format!("{}, ", x));

        write!(
            f,
            "{}, {}{}{}{}{} ({}, {}) with timezone '{}'",
            self.name,
            admin4,
            admin3,
            admin2,
            admin1,
            isocountry::CountryCode::for_alpha2(&(self.country_code)).unwrap(),
            self.latitude,
            self.longitude,
            self.timezone
        )
    }
}

/// Struct for Open Meteo's geolocation API
///
/// # Note
///
/// This structure is heavily dependent on Open Meteo's API structure.  They
/// must someday be decoupled to decrease `core`'s dependence on `om_api`.
#[derive(Debug, serde::Deserialize)]
struct GeoResult {
    results: Vec<Location>,
}
