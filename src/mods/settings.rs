use config::{Config, ConfigError, File};
use std::path::Path;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub unprocessed_folder: String,
    pub converted_folder: String,
    pub to_revert_folder: String,
    pub reverted_folder: String,
    pub tmp_folder: String,
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {

        let s = Config::builder()
            .add_source(File::from(Path::new("config/app.yml")))
            .build()?;
        s.try_deserialize()
    }
}