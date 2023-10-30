use gz::transport::Node;
use gz_msgs::stringmsg::StringMsg;

fn main() {
    let mut node = Node::new().unwrap();
    let rx = node.subscribe_channel::<StringMsg>("/hello", 10).unwrap();

    for msg in rx {
        println!("Received: {}", msg.data);
    }
}
