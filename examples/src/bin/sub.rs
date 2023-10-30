use gz::{msgs::stringmsg::StringMsg, transport::Node};

fn main() {
    let mut node = Node::new().unwrap();
    assert!(node.subscribe("topic", |msg: StringMsg| {
        println!("Subscribed: {}", msg.data);
    }));

    gz::transport::wait_for_shutdown();
}
