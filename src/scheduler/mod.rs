use crate::database::AppState;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;
use crate::database::city_db::{get_all_cities, get_city, update_city};
use crate::database::weather_db::insert_weather_data;
use crate::errors::AppError;
use crate::models::weather::{WeatherApiClient, WeatherDataInsert};


pub struct WeatherScheduler {
    scheduler: JobScheduler,
}

impl WeatherScheduler {
    pub async fn new(
        state: AppState,
        weather_client: WeatherApiClient,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let scheduler = JobScheduler::new().await?;
        let state = Arc::new(state);
        let weather_client = Arc::new(weather_client);

        // Create and schedule the job
        Self::schedule_weather_job(&scheduler, state, weather_client).await?;

        Ok(Self { scheduler })
    }

    async fn schedule_weather_job(
        scheduler: &JobScheduler,
        state: Arc<AppState>,
        weather_client: Arc<WeatherApiClient>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Run every 30 minutes: "0 1/30 * * * *"
        let job = Job::new("0 1/1 * * * *", move |_uuid, _l| {
            let state = Arc::clone(&state);
            let weather_client = Arc::clone(&weather_client);

            tokio::spawn(async move {
                info!("Starting scheduled weather data collection");
                if let Err(e) = collect_weather_data(&state, &weather_client).await {
                    tracing::error!("Weather collection failed: {}", e);
                }
            });
        })?;

        scheduler.add(job).await?;
        Ok(())
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting weather scheduler");
        self.scheduler.start().await?;
        Ok(())
    }
}

async fn collect_weather_data(
    state: &AppState,
    weather_client: &WeatherApiClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get all cities from database
    let cities = get_all_cities(state).await?;

    // 2. For each city, fetch weather and insert
    for city in cities {
        match weather_client.fetch_current_weather(city.latitude, city.longitude).await {
            Ok(weather) => {
                // 3. Prepare data for insertion
                let weather_insert = WeatherDataInsert {
                    city_id: city.id,
                    temperature: weather.temperature,
                    humidity: weather.humidity,
                    pressure: weather.pressure,
                    weather_condition: weather.weather_condition.clone(),
                    raw_data: serde_json::to_value(&weather)?, // Store full weather struct as JSON
                };

                // 4. Insert into database
                if let Err(e) = insert_weather_data(state, &weather_insert).await {
                    tracing::error!("Failed to insert weather for city {}: {}", city.name, e);
                } else {
                    tracing::info!("Weather data collected for {}", city.name);
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch weather for city {}: {}", city.name, e);
            }
        }
    }

    Ok(())
}
