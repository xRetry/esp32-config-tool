use r2r::QosProfile;
use r2r::ros2_esp32_interfaces::msg::PinValues;

pub fn send_pins(target: String) {
    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "esp32_config_tool", "").unwrap();

    let publisher = node
        .create_publisher::<PinValues>(&target, QosProfile::default())
        .unwrap();

    let msg = PinValues {
        values: vec![1.; 36],
    };
    print!(">>> Sending values ... ");
    publisher.publish(&msg).unwrap();
    println!("done");
}
