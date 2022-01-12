use clap::{AppSettings, Parser, Subcommand};
use env_logger::{Builder, Target};
use log::{debug, error, info, log_enabled, warn, Level};
use serialport;
use std::env;
use std::{io::Read, time::Duration};
use ureq::serde_json;
use user_config::{read_config, Config};

mod chirpstack;
mod serial;
mod user_config;
mod utils;

extern crate confy;

/// A tool for managing your LoRa devices and ChirpStack API
#[derive(Parser)]
#[clap(name = "laser-utils")]
#[clap(about = "A tool for managing your LoRa devices and ChirpStack API")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
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
        command: serial::at::AtCommands,
    },
    /// Send request to ChirpStack API. The infomation of API will be read from config file.
    /// Please make sure the config file is correctly set.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Api {
        #[clap(subcommand)]
        command: chirpstack::ApiCommands,
    },
    /// Config the device automatically, hopefully.
    /// Please make sure the device is connected to your computer.
    All {
        /// The path of serial port
        #[clap(short, long)]
        path: String,
        /// Baudrate
        #[clap(short, long, default_value_t = 115200)]
        baud: u32,
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
    }, // TODO: Write config file by subcommand
       // /// A convenient way to set config file
       // #[clap(setting(AppSettings::ArgRequiredElseHelp))]
       // Config
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
    let cfg = read_config(app_name.to_string());
    let cfg = match cfg {
        Ok(cfg) => cfg,
        Err(e) => {
            let file = confy::get_configuration_file_path(&app_name, None).unwrap();
            error!(
                "Parse Config Error: Please check your configuration file at {:#?}",
                file
            );
            panic!("{}", e)
        }
    };

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
        } => serial::at::handle_at_commands(path, baud, command),
        Commands::Api { command } => chirpstack::handle_chirpstack_api(&cfg, command),
        Commands::All {
            path,
            baud,
            name,
            description,
            dev_eui,
            app_key,
        } => {
            fn serial_builder(path:&str, baud:u32) -> Box<dyn serialport::SerialPort> {
                let serial = serialport::new(path, baud)
                    .timeout(Duration::new(5, 0))
                    .open()
                    .expect("Failed to open serial port");
                serial
            }
            let device = chirpstack::LoraDevice::new(&cfg, app_key, dev_eui, description, name);
            serial::at_dev_eui(serial_builder(path, *baud), &device.dev_eui);
            // open a new serial port to avoid ownership problem
            serial::at_app_key(serial_builder(path, *baud), &device.app_key);
            info!("The device is configured successfully, maybe");
            chirpstack::handle_post_device(&cfg, &device);
            info!("The info has been updated successfully, maybe");
        }
    }
}
