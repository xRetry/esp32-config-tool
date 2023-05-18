mod service;
mod subscriber;
mod publisher;
mod types;

use service::send_config;
use subscriber::receive_pins;
use publisher::send_pins;
use anyhow::Result;
use clap::{Parser, command, Subcommand};

/// A tool for interacting with an ESP32 microcontroller, which is running the ROS2-ESP32
/// interface.
#[derive(Parser, Debug)]
#[command(name = "ESP32 Config Tool")]
#[command(version = "1.0")]
struct Args {
    /// Topic name of the target ROS2-node (overwrites target in config file)
    #[clap(long, short)]
    target: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Change the config on a microcontroller
    Set {
        /// Path to a YAML config file
        file: String,
    },
    /// Prints the pin values published by the microcontroller to stdout
    Read,
    /// Send values to the microcontroller
    Write, // TODO: Implement
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Set { file } => send_config(file, args.target).await,
        Command::Read => receive_pins(args.target).await,
        Command::Write => send_pins(args.target).await
    };

    Ok(())
}
