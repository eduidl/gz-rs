//! Direct callbacks run on Gazebo threads and may be concurrent or reentrant.
//! See `sub_channel` for sequential processing on your own thread.

use gz::{msgs::stringmsg::StringMsg, transport::Node};

fn main() {
    let mut node = Node::new().unwrap();
    assert!(node.subscribe("topic", |msg: StringMsg| {
        println!("Subscribed: {}", msg.data);
    }));

    gz::transport::wait_for_shutdown();
}
