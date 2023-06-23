use isocountry;
use serde::Deserialize;
use std;

mod calculators;
mod drivers;
mod file;
mod query;

#[derive(Debug, Deserialize)]
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

pub(crate) struct Weather {
    temperature: f64,
    apparent_temperature: f64,
    weather_code: usize,
    rain: f64,
    windspeed: f64,
    winddirection: f64,
    pressure: f64,
    humidity: f64,
    visibility: f64,
    uv_index: f64,
    sunrise: String,
    sunset: String,
}

#[derive(Debug, Deserialize)]
struct GeoResult {
    results: Vec<Location>,
}

#[derive(Debug, Deserialize)]
struct DailyWeather {
    sunrise: Vec<String>,
    sunset: Vec<String>,
    uv_index_max: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct HourlyWeather {
    temperature_2m: Vec<f64>,
    relativehumidity_2m: Vec<f64>,
    apparent_temperature: Vec<f64>,
    rain: Vec<f64>,
    pressure_msl: Vec<f64>,
    visibility: Vec<f64>,
    windspeed_120m: Vec<f64>,
    winddirection_120m: Vec<f64>,
    weathercode: Vec<usize>,
}

#[derive(Debug, Deserialize)]
struct WeatherResult {
    hourly: HourlyWeather,
    daily: DailyWeather,
}

/// This is the main handler of the journal package.  This is where subfunctions are called.
///
/// # Returns
///
/// A `Result` that contains either
/// an empty tuple, representing a successful run or
/// a `Box<dyn std::error::Error` which is the error that is passed on from the subfunctions
///     if there are any errors inc
pub fn journal_main_driver() -> Result<(), Box<dyn std::error::Error>> {
    let (preamble, file_name) = drivers::journal_init_driver()?;
    let file_path = file::expand_file_path(&file_name)?;
    file::create_file(&file_path)?;
    file::write_preamble(&file_path, &preamble)?;
    Ok(())
}
