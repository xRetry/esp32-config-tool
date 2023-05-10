use r2r::ros2_esp32_interfaces::srv::SetConfig;
use futures::executor;

pub fn send_request(request: SetConfig::Request) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "testnode", "")?;
    let client = node.create_client::<SetConfig::Service>("/esp32_set_config")?;

    //let service_available = node.is_available(&client)?;

    let resp = executor::block_on(client.request(&request)?);
    println!("{:?}", resp);
    Ok(())
}


