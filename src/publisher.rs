use r2r::QosProfile;
use r2r::ros2_esp32_interfaces::msg::PinValues;

pub async fn send_pins(target: String) {
    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "esp32_config_tool", "").unwrap();
    let duration = std::time::Duration::from_millis(2500);

    let mut timer = node.create_wall_timer(duration).unwrap();
    let publisher =
        node.create_publisher::<PinValues>(&target, QosProfile::default()).unwrap();

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    for _ in 1..10 {
        timer.tick().await.unwrap();
        let msg = PinValues {
            values: vec![1.; 36],
        };
        publisher.publish(&msg).unwrap();
    }

    handle.await.unwrap();
}
