pub(crate) mod query;

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

#[derive(Debug, serde::Deserialize)]
struct DailyWeather {
    sunrise: Vec<String>,
    sunset: Vec<String>,
    uv_index_max: Vec<f64>,
}

#[derive(Debug, serde::Deserialize)]
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

#[derive(Debug, serde::Deserialize)]
struct WeatherResult {
    hourly: HourlyWeather,
    daily: DailyWeather,
}