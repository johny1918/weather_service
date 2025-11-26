use serde::{Deserialize, Serialize};
use crate::errors::AppError;

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

impl WeatherApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            api_key,
            base_url: "https://api.openweathermap.org/data/2.5".to_string(),
        }
    }

    pub async fn fetch_current_weather(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<WeatherData, AppError> {
        // Build the API URL
        let url = format!(
            "{}/weather?lat={}&lon={}&appid={}&units=metric",
            self.base_url, latitude, longitude, self.api_key
        );

        // Make the HTTP request
        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Validation(format!("Network error: {}", e)))?;

        // Check if request was successful
        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            return Err(AppError::Validation(format!(
                "Weather API error: {} - {}",
                status, error_body
            )));
        }

        // Parse the JSON response
        let api_response: OpenWeatherResponse = response
            .json()
            .await
            .map_err(|e| AppError::Validation(format!("Failed to parse API response: {}", e)))?;

        // Transform to our internal WeatherData struct
        Ok(WeatherData {
            temperature: api_response.main.temp,
            humidity: api_response.main.humidity,
            pressure: api_response.main.pressure,
            wind_speed: api_response.wind.speed,
            weather_condition: api_response
                .weather
                .first()
                .map(|condition| condition.main.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            timestamp: chrono::DateTime::from_timestamp(api_response.dt, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
        })
    }
}