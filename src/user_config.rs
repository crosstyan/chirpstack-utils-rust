use serde::{Deserialize, Serialize};
use log::{debug, error, info, log_enabled, warn, Level};
extern crate confy;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// The url of ChirpStack API
    pub url: String,
    /// The token of ChirpStack API
    pub token: String,
    /// The application id of ChirpStack API
    pub application_id: String,
    pub device_profile_id: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            url: "http://localhost:8080".into(),
            // a dummy JWT token required by ChirpStack API
            token: "".into(),
            application_id: "2".into(),
            device_profile_id: "70298761-1bf9-4a6c-bda1-69a0eb04aaaf".into(),
        }
    }
}

pub fn read_config(app_name: String) -> Result<Config, confy::ConfyError> {
    let cfg: Config = confy::load(&app_name, None)?;
    let file = confy::get_configuration_file_path(&app_name, None)?;
    info!("The configuration file path is: {:#?}", file);
    debug!("The configuration is:\n{:#?}", cfg);
    Ok(cfg)
}