use futures::future;
use futures::stream::StreamExt;
use r2r::QosProfile;
use r2r::ros2_esp32_interfaces::msg::PinValues;

pub async fn receive_pins(target: String) {
    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "esp32_config_tool", "").unwrap();

    let sub = node.subscribe::<PinValues>(&target, QosProfile::default().best_effort()).unwrap();

    let _ = tokio::task::spawn(async move {
        println!(">>> Start listening to {}...", target);
        sub.for_each(|msg| {
            println!("{{\n\t\"values\": {:?}\n}}", msg.values);
            future::ready(())
        }).await;
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
    }
}
