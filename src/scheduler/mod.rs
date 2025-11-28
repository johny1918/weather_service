use crate::database::AppState;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;
use crate::database::city_db::{get_all_cities, get_city, update_city};
use crate::errors::AppError;
use crate::models::weather::WeatherApiClient;


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
    // Implementation: get all cities, fetch weather for each, store in DB
    Ok(())
}
