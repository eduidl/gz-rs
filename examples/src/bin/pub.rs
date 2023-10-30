use std::{thread::sleep, time::Duration};

use gz::{msgs::stringmsg::StringMsg, transport::Node};

fn main() {
    let mut node = Node::new().unwrap();
    let mut publisher = node.advertise("topic").unwrap();

    for i in 0..100 {
        let topic = StringMsg {
            data: format!("Count {}", i),
            ..Default::default()
        };

        println!("Publishing: {}", topic.data);
        assert!(publisher.publish(&topic));
        sleep(Duration::from_millis(100));
    }
}
