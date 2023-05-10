use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PinSetting {
    pub number: u8,
    pub mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileContent {
    pub change_pins: bool,
    pub change_transport: bool,
    pub change_node: bool,
    pub pins: Vec<PinSetting>,
    pub transport: TransportConfig,
    pub node: NodeConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransportConfig {
    pub use_wifi: bool,
    pub agent_ip: String,
    pub agent_port: String,
    pub wifi_ssid: String,
    pub wifi_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeConfig {
    pub node_name: String,
    pub service_name: String,
    pub publisher_name: String,
    pub subcriber_name: String,
    pub refresh_rate_ms: u32,
}
