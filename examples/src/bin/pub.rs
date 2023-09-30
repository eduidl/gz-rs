use std::{thread::sleep, time::Duration};

use gz::transport::Node;
use gz_msgs::StringMsg;

fn main() {
    env_logger::init();

    let mut node = Node::new().unwrap();
    let mut publisher = node.advertise::<StringMsg>("hello").unwrap();

    for i in 0..100 {
        let topic = StringMsg {
            data: format!("Hello, world! {}", i),
            ..Default::default()
        };

        println!("Publishing: {}", i);
        publisher.publish(&topic);
        sleep(Duration::from_millis(100));
    }
}
