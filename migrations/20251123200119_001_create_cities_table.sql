CREATE TABLE cities (
                        id SERIAL PRIMARY KEY,
                        name VARCHAR(100) NOT NULL,
                        country_code VARCHAR(2) NOT NULL,
                        latitude DECIMAL(8,5) NOT NULL,
                        longitude DECIMAL(8,5) NOT NULL,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_cities_location ON cities(latitude, longitude);