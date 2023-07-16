mod service;
mod subscriber;
mod types;

use service::send_config;
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
    /// Change the config on a microcontroller
    Set {
        /// Path to a YAML config file
        file: String,
        /// Topic name of the target ROS2-node (overwrites target in config file)
        #[clap(long, short)]
        target: Option<String>,
    },
    /// Prints the pin values published by the microcontroller to stdout
    Echo {
        /// Topic name of the target ROS2-node
        target: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Set { file, target } => send_config(file, target).await,
        Command::Echo { target } => receive_pins(target).await,
    };

    Ok(())
}
