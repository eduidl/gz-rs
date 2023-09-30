use gz::transport::Node;
use gz_msgs::StringMsg;

fn main() {
    env_logger::init();

    let mut node = Node::new().unwrap();
    node.subscribe::<StringMsg, _>("/hello", |msg| {
        println!("{}", msg.data);
    });

    gz::transport::wait_for_shutdown();
}
