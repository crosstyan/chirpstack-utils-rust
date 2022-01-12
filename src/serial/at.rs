use crate::serial;
use crate::utils;
use clap::{AppSettings, Parser, Subcommand};
use log::{debug, error, info, log_enabled, warn, Level};
use std::{io::Read, time::Duration};

#[derive(Subcommand)]
pub enum AtCommands {
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
pub enum SetCommands {
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

pub fn handle_at_commands(path: &String, baud: &u32, command: &AtCommands) {
    let serial = serialport::new(path, *baud)
        .timeout(Duration::new(15, 0))// give a longer timeout for the command
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
