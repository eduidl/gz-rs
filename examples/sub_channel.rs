use gz::{msgs::StringMsg, transport::Node};

fn main() {
    env_logger::init();

    let mut node = Node::new().unwrap();
    let rx = node.subscribe_channel::<StringMsg>("/hello", 10).unwrap();

    for msg in rx {
        println!("Received: {}", msg.data);
    }
}
