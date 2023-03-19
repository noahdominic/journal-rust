use serde::Deserialize;
use std;
use std::io::Write;
use dirs;
use isocountry;

mod query;
mod drivers;
mod calculators;

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
    timezone: String,
    country_code: String,
    admin1: Option<String>,
    admin2: Option<String>,
    admin3: Option<String>,
    admin4: Option<String>
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let admin1 = self.admin1.as_ref().map_or("".to_string(), |x| format!("{}, ", x));
        let admin2 = self.admin2.as_ref().map_or("".to_string(), |x| format!("{}, ", x));
        let admin3 = self.admin3.as_ref().map_or("".to_string(), |x| format!("{}, ", x));
        let admin4 = self.admin4.as_ref().map_or("".to_string(), |x| format!("{}, ", x));

        write!(f, "{}, {}{}{}{}{} ({}, {}) with timezone '{}'", 
            self.name, 
            admin4, admin3, admin2, admin1, isocountry::CountryCode::for_alpha2(&(self.country_code)).unwrap(), 
            self.latitude, self.longitude,
            self.timezone
        )
    }
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
    
    println!("\n\nThis will written in: {}\n{}", file_path.display(), preamble);

    if query::query_for_bool("Does everything look correct?  This will print in the file if yes.")? {
        // This is the file of the journal entry
        let mut file = std::fs::OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open(&file_path)?;    
        
        writeln!(&mut file, "{}", preamble)?;
    } else {
        println!("OK.  File not written.")
    }
    Ok(())
}