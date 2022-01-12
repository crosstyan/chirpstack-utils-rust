use clap::{AppSettings, Parser, Subcommand};
use crate::user_config::{Config};
use crate::serde_json;
use log::{debug, error, info, log_enabled, warn, Level};

#[derive(Subcommand)]
pub enum ApiCommands {
    /// Post a device to ChirpStack API
    Post {
        /// The device name
        #[clap(short, long, default_value = "")]
        name: String,
        /// The device description
        #[clap(short, long, default_value = "a test device")]
        description: String,
        #[clap(long, default_value = "")]
        dev_eui: String,
        #[clap(long, default_value = "")]
        app_key: String,
    },
    /// Get a device list from ChirpStack API
    Get,
}
struct LoraDevice {
    devEUI: String,
    appKey: String,
    applicationID: String,
    description: String,
    deviceProfileID: String,
    isDisabled: bool,
    skipFCntCheck: bool,
    name: String,
    referenceAltitude: i32,
}

pub fn handle_chirpstack_api(cfg:&Config, app_name:&str, command:&ApiCommands) {
    if cfg.url.trim().is_empty() {
        let file = confy::get_configuration_file_path(app_name, None).unwrap();
        error!(
            "The url is invalid. Please check the configuration file path at: {:#?}",
            file
        );
        panic!("The url token is invalid.");
    }
    if cfg.token.trim().is_empty() {
        let file = confy::get_configuration_file_path(app_name, None).unwrap();
        error!(
            "The JWT token is invalid. Please check the configuration file path at: {:#?}",
            file
        );
        panic!("The JWT token is invalid.");
    }
    match command {
        ApiCommands::Post {
            name,
            description,
            dev_eui,
            app_key,
        } => {
            // fill the logic here
        }
        ApiCommands::Get => {
            let msg = get_device(&cfg).expect("Failed to get device");
            info!("{}", serde_json::to_string_pretty(&msg).unwrap());
        }
    }
}

fn get_device(cfg: &Config) -> Result<serde_json::Value, ureq::Error> {
    let msg: serde_json::Value = ureq::get(&format!("{}/devices", cfg.url))
        .set("Authorization", &format!("Bearer {}", cfg.token))
        .call()?
        .into_json()?;
    Ok(msg)
}
