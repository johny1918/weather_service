use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f64,      // in Celsius
    pub humidity: u8,          // percentage
    pub pressure: u16,         // hPa
    pub wind_speed: f64,       // m/s
    pub weather_condition: String, // e.g., "Clear", "Rain"
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
struct OpenWeatherResponse {
    main: OpenWeatherMain,
    wind: OpenWeatherWind,
    weather: Vec<OpenWeatherCondition>,
    dt: i64, // UNIX timestamp
}

#[derive(Debug, Deserialize)]
struct OpenWeatherMain {
    temp: f64,
    humidity: u8,
    pressure: u16,
}

#[derive(Debug, Deserialize)]
struct OpenWeatherWind {
    speed: f64,
}

#[derive(Debug, Deserialize)]
struct OpenWeatherCondition {
    main: String,
}

#[derive(Clone)]
pub struct WeatherApiClient {
    http_client: reqwest::Client,
    api_key: String,
    base_url: String,
}