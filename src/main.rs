mod ros2_service;
mod types;

use std::env;
use serde_yaml;
use types::FileContent; 
use ros2_service::send_request;
use r2r::ros2_esp32_interfaces::{srv::SetConfig, msg::{PinConfig, TransportConfig, NodeConfig}};
use anyhow::{Result, anyhow};

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
        change_pins: file_content.change_pins,
        change_node: file_content.change_node,
        change_transport: file_content.change_transport,
        new_pin_config: PinConfig{ pin_modes: pin_modes.to_vec() },
        new_transport_config: TransportConfig{
            use_wifi: file_content.transport.use_wifi,
            wifi_pw: file_content.transport.wifi_password,
            wifi_ssid: file_content.transport.wifi_ssid,
            agent_ip: file_content.transport.agent_ip,
            agent_port: file_content.transport.agent_port,
        },
        new_node_config: NodeConfig{
            node_name: file_content.node.node_name,
            service_name: file_content.node.service_name,
            publisher_name: file_content.node.publisher_name,
            subscriber_name: file_content.node.subcriber_name,
            refresh_rate_ms: file_content.node.refresh_rate_ms,
        }
    })
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Provide config file path as console argument");
    let reader = std::fs::File::open(file_path).expect("Unable to open file");
    let file_content: FileContent = serde_yaml::from_reader(reader).expect("Unable to parse file");

    let request = content_to_request(file_content)?;

    println!("{:?}", request);
    send_request(request).unwrap();

    Ok(())
}
