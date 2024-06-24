/*******************************************************************************
 * Copyright (c) 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved
 * Licensed under the EUPL v1.2
 ******************************************************************************/


pub(crate) mod query;

#[derive(Debug)]
pub(crate) struct Weather {
    pub(crate) temperature: f64,
    pub(crate) apparent_temperature: f64,
    pub(crate) weather_code: usize,
    pub(crate) rain: f64,
    pub(crate) windspeed: f64,
    pub(crate) winddirection: f64,
    pub(crate) pressure: f64,
    pub(crate) humidity: f64,
    pub(crate) visibility: f64,
    pub(crate) uv_index: f64,
    pub(crate) sunrise: String,
    pub(crate) sunset: String,
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