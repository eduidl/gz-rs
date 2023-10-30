use gz::{msgs::stringmsg::StringMsg, transport::Node};

fn main() {
    let mut node = Node::new().unwrap();
    let rx = node.subscribe_channel::<StringMsg>("topic", 10).unwrap();

    for msg in rx {
        println!("Received: {}", msg.data);
    }
}
