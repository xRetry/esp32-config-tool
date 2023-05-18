mod ros2_service;
mod types;

use serde_yaml;
use types::FileContent; 
use ros2_service::send_config;
use r2r::ros2_esp32_interfaces::srv::SetConfig;
use anyhow::{Result, anyhow};
use clap::{Parser, command};

/// A tool for interacting with an ESP32 microcontroller, which is running the ROS2-ESP32
/// interface.
#[derive(Parser, Debug)]
#[command(name = "ESP32 Config Tool")]
#[command(version = "1.0")]
struct Args {
    /// Path to a YAML config file
    file: String,
}

fn content_to_request(file_content: FileContent) -> Result<SetConfig::Request> {
    let mut pin_modes = [0; 36];
    for p in file_content.pins {
        pin_modes[p.number as usize] = match p.mode.as_str() {
            "disabled" => 0,
            "digital_input" => 1,
            "digital_output" => 2,
            "analog_input" => 3,
            "analog_output" => 4,
            m => return Err(anyhow!("Invalid pin mode {}", m)),
        };
    }

    Ok(SetConfig::Request{
        read_only: file_content.read_only,
        pin_modes: pin_modes.to_vec(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let reader = std::fs::File::open(args.file).expect("Unable to open file");
    let file_content: FileContent = serde_yaml::from_reader(reader).expect("Unable to parse file");

    let request = content_to_request(file_content)?;

    println!("{:?}", request);
    send_config(request).await.unwrap();

    Ok(())
}
