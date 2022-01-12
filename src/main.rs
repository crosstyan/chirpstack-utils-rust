use clap::{AppSettings, Parser, Subcommand};
use log::{debug, error, info, log_enabled, warn, Level};
use serialport;
use ureq::serde_json;
use env_logger::{Builder, Target};
use serde::{Deserialize, Serialize};
use std::env;
use user_config::{read_config, Config};

mod serial;
mod utils;
mod chirpstack;
mod user_config;

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
            serial::at::handle_at_commands(path, baud, command)
        }
        Commands::Api { command } => {
            chirpstack::handle_chirpstack_api(&cfg, app_name, command)
        }
    }
}
