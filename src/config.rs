use noise::utils::Color;
use ron::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub size: u32,
    pub size_k: f64,
    pub borders: Option<u32>,
    pub seed: Option<u32>,
    pub water_color: Color,
    pub no_color: Color,
    pub low_color: Color,
    pub mid_color: Color,
    pub high_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: 1280,
            size_k: 0.1,
            borders: None,
            seed: None,
            water_color: [0, 0, 255, 255],
            no_color: [0, 0, 0, 255],
            low_color: [255, 255, 255, 255],
            mid_color: [100, 100, 100, 255],
            high_color: [50, 50, 50, 255],
        }
    }
}

pub fn get_config() -> Config {
    if let Ok(config_src) = std::fs::read_to_string("config.ron") {
        from_str(&config_src).unwrap_or_default()
    } else {
        Config::default()
    }
}
