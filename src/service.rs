use r2r::ros2_esp32_messages::srv::SetConfig;
use anyhow::Result;
use crate::types::FileContent;

fn file_to_request(file_content: &FileContent) -> Result<SetConfig::Request> {
    let mut pin_modes = [0; 40];
    for p in &file_content.pins {
        pin_modes[p.number as usize] = p.mode;
    }

    Ok(SetConfig::Request{
        read_only: file_content.read_only.unwrap_or(false),
        pin_modes: pin_modes.to_vec(),
    })
}

fn send_request(target: String, request: SetConfig::Request) {
    let ctx = r2r::Context::create()
        .unwrap();
    let mut node = r2r::Node::create(ctx, "esp32_config_tool", "")
        .unwrap();
    let client = node.create_client::<SetConfig::Service>(&target)
        .unwrap();

    let task = tokio::task::spawn(async move {
        if let Ok(resp) = client.request(&request)
            .unwrap()
            .await {
                println!("{{\n\t\"pin_modes\":\t{:?},\n\t\"pin_errors\":\t{:?}\n}}", resp.pin_modes, resp.pin_errors);
            }
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
        if task.is_finished() { break; }
    }
}

pub async fn set_config(file: String, target: Option<String>) {
    let reader = std::fs::File::open(file)
        .expect("Unable to open file");
    let file_content: FileContent = serde_yaml::from_reader(reader)
        .expect("Unable to parse file");

    let request = file_to_request(&file_content)
        .unwrap();

    let target = target.unwrap_or(file_content.target_topic.expect(
        "The target topic needs to be set in the config file or the command line!"
    ));

    send_request(target, request);
}

pub async fn get_config(target: String) {
    let request = SetConfig::Request{
        read_only: true,
        pin_modes: vec![0; 40],
    };

    send_request(target, request);
}

