CREATE TABLE weather_data (
                              id SERIAL PRIMARY KEY,
                              city_id INTEGER REFERENCES cities(id) ON DELETE CASCADE,
                              temperature DECIMAL(4,1) NOT NULL,
                              humidity INTEGER NOT NULL,
                              pressure INTEGER NOT NULL,
                              weather_condition VARCHAR(50) NOT NULL,
                              recorded_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                              raw_data JSONB NOT NULL
);

CREATE INDEX idx_weather_data_city ON weather_data(city_id);
CREATE INDEX idx_weather_data_timestamp ON weather_data(recorded_at);
CREATE INDEX idx_weather_data_city_timestamp ON weather_data(city_id, recorded_at);