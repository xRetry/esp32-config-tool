mod service;
mod subscriber;
mod types;

use service::{set_config, get_config};
use subscriber::receive_pins;
use anyhow::Result;
use clap::{Parser, command, Subcommand};

/// A tool for interacting with an ESP32 microcontroller, which is running the ROS2-ESP32
/// interface.
#[derive(Parser, Debug)]
#[command(name = "ESP32 Config Tool")]
#[command(version = "1.0")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Change the pin config of the microcontroller
    Set {
        /// Path to a YAML config file
        file: String,
        /// The service topic name of the targeted ROS2-ESP32 Interface (overwrites target in
        /// config file)
        #[clap(long, short)]
        target: Option<String>,
    },
    /// Get the currently active pin config of the microcontroller
    Get {
        /// The service topic name of the targeted ROS2-ESP32 Interface
        target: String,
    },
    /// Print the pin values published by the microcontroller to the console
    Echo {
        /// The publisher topic name of the targeted ROS2-ESP32 Interface
        target: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Set { file, target } => set_config(file, target).await,
        Command::Get { target } => get_config(target).await,
        Command::Echo { target } => receive_pins(target).await,
    };

    Ok(())
}
