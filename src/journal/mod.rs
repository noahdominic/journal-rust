use serde::Deserialize;
use std;
use std::io::Write;
use dirs;

mod drivers;
mod calculators;

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
    timezone: String,
    country_code: String
}

struct Weather {
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
    sunset: String
}

#[derive(Debug, Deserialize)]
struct GeoResult {
    results: Vec<Location>
}

#[derive(Debug, Deserialize)]
struct DailyWeather {
    sunrise: Vec<String>,
    sunset: Vec<String>,
    uv_index_max: Vec<f64>
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
    weathercode: Vec<usize>
}

#[derive(Debug, Deserialize)]
struct WeatherResult {
    hourly: HourlyWeather,
    daily: DailyWeather
}

/// This is the main handler of the journal package.  This is where subfunctions are called.
/// 
/// # Returns
/// 
/// A `Result` that contains either
/// an empty tuple, representing a successful run or 
/// a `Box<dyn std::error::Error` which is the error that is passed on from the subfunctions
///     if there are any errors inc
pub fn main_driver() -> Result<(), Box<dyn std::error::Error>> {
    let (preamble, file_name) = drivers::journal_init_driver()?;

    // Expand the tilde to the home directory
    let file_path = std::path::Path::new(&file_name);
    let file_path = dirs::home_dir().map_or_else(
        || file_path.to_owned(),
        |home_dir| home_dir.join(file_path.strip_prefix("~").unwrap()),
    );
    
    // Create directories recursively if needed
    std::fs::create_dir_all(file_path.parent().unwrap()) ?;
    
    println!("\n\nThis will print in: {}\n{}", file_path.display(), preamble);
    
    // This is the file of the journal entry
    let mut _file = std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(&file_path)?;    
    
    writeln!(&mut file, "{}", preamble)?;
    Ok(())
}









