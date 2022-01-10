use clap::{AppSettings, Parser, Subcommand};
use serialport::{available_ports, new};
use log::{debug, error, log_enabled, info, Level};
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;

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
    /// pushes things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Send {
        /// The path of serial port
        #[clap(short, long)]
        path: String,
        /// Baudrate
        #[clap(short, long, default_value_t = 115200)]
        baud: u32,
        #[clap(subcommand)]
        command: SendCommands
    },
}

#[derive(Subcommand)]
enum SendCommands {
    /// Send a `at+version`
    Version,
}

fn main() {
    env_logger::init();
    let args = Cli::parse();

    match &args.command {
        Commands::Ls => {
            let ports = available_ports().unwrap();
            for port in ports {
                println!("{}", port.port_name);
            }
        }
        Commands::Send { path, baud, command } => {
            let mut serial = new(path, *baud)
                                                .timeout(Duration::new(5,0))
                                                .open()
                                                .expect("Failed to open serial port");
            match &command {
                // https://stackoverflow.com/questions/63131868/how-to-pass-mut-str-and-change-the-original-mut-str-without-a-return
                // https://stackoverflow.com/questions/68021274/pass-mut-reference-to-a-function-and-get-it-back
                SendCommands::Version => {
                    let w_buf = "at+version\r\n";
                    let mut r_buf = String::new();
                    serial.write(w_buf.as_bytes()).expect("Failed to write");
                    // https://stackoverflow.com/questions/67520106/how-to-use-read-line-function-with-rusts-serialport-crate
                    let mut reader = BufReader::new(serial);
                    reader.read_line(&mut r_buf).expect("Failed to read");
                    println!("{}", r_buf);
                }
            }
        }
    }
    // Continued program logic goes here...
}