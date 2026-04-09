use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    name: String,
    main: MainStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainStats {
    temp: f32,
    humidity: f32,
}
