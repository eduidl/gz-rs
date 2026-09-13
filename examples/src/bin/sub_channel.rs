//! Recommended reception API: keep the latest 10 messages and process locally.

use gz::{msgs::stringmsg::StringMsg, transport::Node};

fn main() {
    let mut node = Node::new().unwrap();
    let rx = node.subscribe_channel::<StringMsg>("topic", 10).unwrap();

    let mut count = 0;
    while let Ok(msg) = rx.recv_blocking() {
        count += 1;
        println!("Received #{count}: {}", msg.data);
    }
}
