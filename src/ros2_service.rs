use r2r::ros2_esp32_interfaces::srv::SetConfig;

pub async fn send_config(request: SetConfig::Request) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "esp32_config_tool", "")?;
    let client = node.create_client::<SetConfig::Service>("/esp32_set_config")?;

    let task = tokio::task::spawn(async move {
        if let Ok(resp) = client.request(&request).unwrap().await {
            println!("{:?}", resp);
        }
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
        if task.is_finished() { break; }
    }

    return Ok(());
}


