use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PinSetting {
    pub number: u8,
    pub mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileContent {
    pub target_topic: Option<String>,
    pub read_only: bool,
    pub pins: Vec<PinSetting>,
}
