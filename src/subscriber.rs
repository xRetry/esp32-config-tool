use futures::future;
use futures::stream::StreamExt;
use r2r::QosProfile;
use r2r::ros2_esp32_interfaces::msg::PinValues;

pub async fn receive_pins(target: Option<String>) {
    let target = target.expect(
        "The target topic needs to be provided via commandline using --target <TARGET> or -t <TARGET>"
    );

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "esp32_config_tool", "").unwrap();

    let sub = node.subscribe::<PinValues>(&target, QosProfile::default()).unwrap();

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    sub.for_each(|msg| {
        println!("{:?}", msg.values);
        future::ready(())
    }).await;

    handle.await.unwrap();
}
