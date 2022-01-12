use crate::serde_json;
use crate::user_config::Config;
use crate::utils::gen_hex::{
    get_rand_app_key, get_rand_dev_eui, get_rand_hex_str, verify_app_key, verify_dev_eui,
};
use clap::{AppSettings, Parser, Subcommand};
use log::{debug, error, info, log_enabled, warn, Level};
use serde::__private::de;
use serde::{Deserialize, Serialize};
use ureq::json;

/// The app will read the config file automatically.
/// Please make sure the config file is correctly set.
#[derive(Subcommand)]
pub enum ApiCommands {
    /// Post a device to ChirpStack API
    Post {
        /// The device name. If not specified, the name will be generated randomly.
        #[clap(short, long, default_value = "")]
        name: String,
        /// The device description
        #[clap(short, long, default_value = "a test device")]
        description: String,
        /// Set the DevEUI (64 bit hex). if not set, the DevEUI will be generated randomly.
        #[clap(long, default_value = "")]
        dev_eui: String,
        /// Set the app key (128 bit hex). if not set, the app key will be generated randomly.
        #[clap(long, default_value = "")]
        app_key: String,
    },
    /// Get a device list from ChirpStack API
    Get {
        /// Max number of devices to get
        #[clap(short, long, default_value_t = 10)]
        limit: u32,
        /// Number of devices to skip
        #[clap(short, long, default_value_t = 0)]
        offset:u32
    },
}

/// LoraDevice Structure representing a LoRa device
/// Used for ChirpStack API post body
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoraDevice {
    // https://serde.rs/field-attrs.html
    #[serde(rename = "devEUI")]
    pub dev_eui: String,
    ///AppKey
    #[serde(rename = "appKey", skip_serializing_if = "String::is_empty")]
    pub app_key: String,
    #[serde(rename = "applicationID")]
    pub application_id: String,
    pub description: String,
    #[serde(rename = "deviceProfileID")]
    pub device_profile_id: String,
    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,
    #[serde(rename = "skipFCntCheck")]
    pub skip_fcnt_check: bool,
    pub name: String,
    #[serde(rename = "referenceAltitude")]
    pub reference_altitude: i32,
}

impl LoraDevice {
    pub fn new(
        cfg: &Config,
        app_key: &str,
        dev_eui: &str,
        description: &str,
        name: &str,
    ) -> LoraDevice {
        LoraDevice {
            dev_eui: if dev_eui.is_empty() || !verify_dev_eui(dev_eui.to_string()) {
                warn!("The DevEUI is invalid. It will be generated randomly.");
                get_rand_dev_eui()
            } else {
                dev_eui.into()
            },
            app_key: if app_key.is_empty() || !verify_app_key(app_key.to_string()) {
                warn!("The app key is invalid. It will be generated randomly.");
                get_rand_app_key().into()
            } else {
                app_key.into()
            },
            description: description.into(),
            application_id: cfg.application_id.clone().into(),
            device_profile_id: cfg.device_profile_id.clone().into(),
            is_disabled: false,
            skip_fcnt_check: false,
            name: if name.is_empty() {
                warn!("The device name is not specified. It will be generated randomly.");
                get_rand_hex_str(24)
            } else {
                name.to_string()
            },
            reference_altitude: 0,
        }
    }
}

pub fn handle_chirpstack_api(cfg: &Config, command: &ApiCommands) {
    match command {
        ApiCommands::Post {
            name,
            description,
            dev_eui,
            app_key,
        } => {
            let device = LoraDevice::new(cfg, app_key, dev_eui, description, name);
            handle_post_device(cfg, &device);
        }
        ApiCommands::Get { limit , offset} => {
            let msg = get_device(&cfg, *limit, *offset).expect("Failed to get device");
            info!("{}", serde_json::to_string_pretty(&msg).unwrap());
        }
    }
}

fn get_device(cfg: &Config, limit: u32, offset: u32) -> Result<serde_json::Value, ureq::Error> {
    let msg: serde_json::Value = ureq::get(&format!("{}/devices", cfg.url))
        .set("Authorization", &format!("Bearer {}", cfg.token))
        .query("applicationID", &cfg.application_id.clone())
        .query("limit", &limit.to_string())
        .query("offset", &offset.to_string())
        .call()?
        .into_json()?;
    Ok(msg)
}

/// Do both post device and set the key of the device
pub fn handle_post_device(cfg: &Config, device: &LoraDevice) {
    info!(
        "Device Info\nDevEUI: {0}\nAppKey: {1}\nName: {2}",
        device.dev_eui, device.app_key, device.name
    );
    let msg = post_device(cfg, &device).unwrap();
    debug!(
        "Response in post device:\n{}",
        serde_json::to_string_pretty(&msg).unwrap()
    );
    let msg = post_appkey(cfg, &device).unwrap();
    debug!(
        "Response in post appkey:\n{}",
        serde_json::to_string_pretty(&msg).unwrap()
    );
}

fn post_device(cfg: &Config, device: &LoraDevice) -> Result<serde_json::Value, ureq::Error> {
    // chirpstack doesn't allow post a device and appkey at the same time
    // so we have to move appKey out of the device struct
    let mut clone_device = device.clone();
    clone_device.app_key = "".into();
    let request = json!({ "device": clone_device });
    debug!("POST:\n{}", serde_json::to_string_pretty(&request).unwrap());
    let msg: serde_json::Value = ureq::post(&format!("{}/devices", cfg.url))
        .set("Authorization", &format!("Bearer {}", cfg.token))
        .query("applicationID", &cfg.application_id.clone())
        .send_json(request)?
        .into_json()?;
    Ok(msg)
}

fn post_appkey(cfg: &Config, device: &LoraDevice) -> Result<serde_json::Value, ureq::Error> {
    let request = json!({
        "deviceKeys":{
            "nwkKey": device.app_key,
            "devEUI": device.dev_eui
    }});
    debug!("POST:\n{}", serde_json::to_string_pretty(&request).unwrap());
    let msg: serde_json::Value = ureq::post(&format!("{0}/devices/{1}/keys", cfg.url, device.dev_eui))
        .set("Authorization", &format!("Bearer {}", cfg.token))
        .query("applicationID", &cfg.application_id.clone())
        .send_json(request)?
        .into_json()?;
    Ok(msg)
}
