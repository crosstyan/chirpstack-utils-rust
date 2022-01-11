use clap::{AppSettings, Parser, Subcommand};
use log::{debug, error, info, log_enabled, warn, Level};
use serialport;
use ureq::serde_json;
use std::{io::Read, time::Duration};
mod serial;
mod utils;
use env_logger::{Builder, Target};
use serde::{Deserialize, Serialize};
use std::env;
extern crate confy;

/// A tool for managing your LoRa devices and ChirpStack API
#[derive(Parser)]
#[clap(name = "laser-utils")]
#[clap(about = "A tool for managing your LoRa devices and ChirpStack API")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    /// The url of ChirpStack API
    url: String,
    /// The token of ChirpStack API
    token: String,
    /// The application id of ChirpStack API
    id: String,
    device_profile_id: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            url: "https://localhost:8080".into(),
            // a dummy JWT token required by ChirpStack API
            token: "".into(),
            id: "2".into(),
            device_profile_id: "70298761-1bf9-4a6c-bda1-69a0eb04aaaf".into(),
        }
    }
}

fn read_config(app_name: String) -> Result<Config, confy::ConfyError> {
    let cfg: Config = confy::load(&app_name, None)?;
    let file = confy::get_configuration_file_path(&app_name, None)?;
    info!("The configuration file path is: {:#?}", file);
    debug!("The configuration is:");
    debug!("{:#?}", cfg);
    Ok(cfg)
}

#[derive(Subcommand)]
enum Commands {
    /// List Serial ports
    // No need params
    Ls,
    /// Send at command to serial ports
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    At {
        /// The path of serial port
        #[clap(short, long)]
        path: String,
        /// Baudrate
        #[clap(short, long, default_value_t = 115200)]
        baud: u32,
        #[clap(subcommand)]
        command: AtCommands,
    },
    /// Send request to ChirpStack API
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Api {
        #[clap(subcommand)]
        command: ApiCommands,
    },
}

#[derive(Subcommand)]
enum ApiCommands {
    /// Post a device to ChirpStack API
    Post,
    /// Get a device list from ChirpStack API
    Get,
}

#[derive(Subcommand)]
enum AtCommands {
    /// Send a `at+version` to check the software version of device
    Version,
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    /// Set the device property by at command
    Set {
        #[clap(subcommand)]
        command: SetCommands,
    },
    /// Send a `at+join` to make the device join the local Lora network
    Join,
}

#[derive(Subcommand)]
enum SetCommands {
    /// The DevEUI is a 64-bit globally-unique Extended Unique Identifier (EUI-64) assigned by the manufacturer, or the owner, of the end-device.
    DevEui {
        #[clap(default_value = "")]
        dev_eui: String,
    },
    /// The Application Session Key (AppSKey) is used for encryption and decryption of the payload.
    AppKey {
        #[clap(default_value = "")]
        app_key: String,
    },
}

fn main() {
    // Set the default log level to debug
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    // Output the log to STDOUT
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
    let args = Cli::parse();
    let app_name = "laser-utils";
    let cfg = read_config(app_name.to_string()).expect("Couldn't read config file");

    match &args.command {
        Commands::Ls => {
            let ports = serialport::available_ports().unwrap();
            for port in ports {
                println!("{}", port.port_name);
            }
        }
        Commands::At {
            path,
            baud,
            command,
        } => {
            let serial = serialport::new(path, *baud)
                .timeout(Duration::new(5, 0))
                .open()
                .expect("Failed to open serial port");
            match &command {
                // https://stackoverflow.com/questions/63131868/how-to-pass-mut-str-and-change-the-original-mut-str-without-a-return
                // https://stackoverflow.com/questions/68021274/pass-mut-reference-to-a-function-and-get-it-back
                AtCommands::Version => {
                    serial::at_version(serial);
                }
                AtCommands::Join => {
                    serial::at_join(serial);
                }
                AtCommands::Set { command } => match command {
                    SetCommands::DevEui { dev_eui } => {
                        if utils::gen_hex::verify_dev_eui(dev_eui.clone()) {
                            serial::at_dev_eui(serial, dev_eui);
                        } else {
                            warn!("Invalid DevEUI or DevEUI is not provided. Auto-generated DevEUI will be used.");
                            let dev_eui = utils::gen_hex::get_rand_dev_eui();
                            info!("Please save the DevEUI: {}", dev_eui);
                            serial::at_dev_eui(serial, &dev_eui);
                        }
                    }
                    SetCommands::AppKey { app_key } => {
                        if utils::gen_hex::verify_app_key(app_key.clone()) {
                            serial::at_app_key(serial, app_key);
                        } else {
                            warn!("Invalid AppKey or AppKey is not provided. Auto-generated AppKey will be used.");
                            let app_key = utils::gen_hex::get_rand_app_key();
                            info!("Please save the AppKey: {}", app_key);
                            serial::at_app_key(serial, &app_key);
                        }
                    }
                },
            }
        }
        Commands::Api { command } => {
            if cfg.url.trim().is_empty() {
                let file = confy::get_configuration_file_path(app_name, None).unwrap();
                error!("The url is invalid. Please check the configuration file path at: {:#?}", file);
                panic!("The url token is invalid.");
            }
            if cfg.token.trim().is_empty() {
                let file = confy::get_configuration_file_path(app_name, None).unwrap();
                error!("The JWT token is invalid. Please check the configuration file path at: {:#?}", file);
                panic!("The JWT token is invalid.");
            }
            match command {
                ApiCommands::Post => {
                    // let client = Client::new(&cfg.url, &cfg.token, &cfg.id).unwrap();
                    // let device_profile_id = cfg.device_profile_id.clone();
                    // let dev_eui = utils::gen_hex::get_rand_dev_eui();
                    // let app_key = utils::gen_hex::get_rand_app_key();
                    // let device = Device {
                    //     dev_eui: dev_eui.clone(),
                    //     app_key: app_key.clone(),
                    //     device_profile_id: device_profile_id.clone(),
                    // };
                    // let res = client.post_device(&device);
                    // info!("{:#?}", res);
                }
                ApiCommands::Get => {
                    let msg = get_device(&cfg).expect("Failed to get device");
                    info!("{}", serde_json::to_string_pretty(&msg).unwrap());
                }
            }
        }
    }
}

fn get_device(cfg: &Config) -> Result<serde_json::Value, ureq::Error> {
    let msg:serde_json::Value = ureq::get(&format!("{}/devices", cfg.url))
        .set("Authorization", &format!("Bearer {}", cfg.token))
        .call()?
        .into_json()?;
    Ok(msg)
}