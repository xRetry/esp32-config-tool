use r2r::ros2_esp32_interfaces::srv::SetConfig;
use anyhow::{Result, anyhow};
use crate::types::FileContent;

fn content_to_request(file_content: &FileContent) -> Result<SetConfig::Request> {
    let mut pin_modes = [0; 36];
    for p in &file_content.pins {
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

pub async fn send_config(file: String, target: Option<String>) {
    let reader = std::fs::File::open(file).expect("Unable to open file");
    let file_content: FileContent = serde_yaml::from_reader(reader).expect("Unable to parse file");

    let request = content_to_request(&file_content).unwrap();

    let target = target.unwrap_or(file_content.target_topic.expect(
        "The target topic needs to be set in the config file or the command line!"
    ));

    println!("{:?}", request);

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "esp32_config_tool", "").unwrap();
    let client = node.create_client::<SetConfig::Service>(&target).unwrap();

    let task = tokio::task::spawn(async move {
        if let Ok(resp) = client.request(&request).unwrap().await {
            println!("{:?}", resp);
        }
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
        if task.is_finished() { break; }
    }

}


