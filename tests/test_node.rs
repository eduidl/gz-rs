use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use gz::{
    msgs::{StringMsg, Vector3d},
    transport::Node,
};
use uuid::Uuid;

#[test]
fn test_node_new() {
    assert!(Node::new().is_some());
}

#[test]
fn test_node_new_with_partition() {
    assert!(Node::with_partition("valid_name").is_some());
    assert!(Node::with_partition("/valid_name").is_some());
    assert!(Node::with_partition(":").is_some());
}

#[test]
fn test_node_new_with_invalid_name() {
    assert!(Node::with_partition("/").is_none());
    assert!(Node::with_partition("//invalid").is_none());
    assert!(Node::with_partition("@").is_none());
    assert!(Node::with_partition("~").is_none());
    assert!(Node::with_partition(":=").is_none());
}

#[test]
fn test_node_topic_list() {
    let partition = Uuid::new_v4().to_string();

    let node1 = Node::with_partition(&partition).unwrap();
    let mut node2 = Node::with_partition(&partition).unwrap();

    assert!(node1.topic_list().is_empty());

    let _p1 = node2.advertise::<StringMsg>("hoge").unwrap();
    assert!(node2.subscribe::<StringMsg, _>("/fuga/hoge", |_msg| {}));

    let topics = node1.topic_list();

    assert_eq!(topics.len(), 1);
    assert!(topics.contains(&"/hoge".to_string()));
}

#[test]
fn test_node_advertised_list() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();

    assert!(node.advertised_topics().is_empty());

    let _p1 = node.advertise::<StringMsg>("hoge").unwrap();
    let _p2 = node.advertise::<StringMsg>("/fuga/hoge").unwrap();

    let topics = node.advertised_topics();

    assert_eq!(topics.len(), 2);
    assert!(topics.contains(&"/hoge".to_string()));
    assert!(topics.contains(&"/fuga/hoge".to_string()));
}

#[test]
fn test_node_subscribed_list() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();

    assert!(node.subscribed_topics().is_empty());

    assert!(node.subscribe::<StringMsg, _>("hoge", |_msg| {}));
    assert!(node.subscribe::<StringMsg, _>("/fuga/hoge", |_msg| {}));

    let topics = node.subscribed_topics();

    assert_eq!(topics.len(), 2);
    assert!(topics.contains(&"/hoge".to_string()));
    assert!(topics.contains(&"/fuga/hoge".to_string()));
}

#[test]
fn test_node_pub_sub() {
    let partition = Uuid::new_v4().to_string();

    let mut pub_node = Node::with_partition(&partition).unwrap();
    let mut sub_node = Node::with_partition(&partition).unwrap();

    let counter = Arc::new(AtomicU8::new(0));
    {
        let counter = Arc::clone(&counter);
        sub_node.subscribe::<StringMsg, _>("hoge", move |msg| {
            assert_eq!(msg.data, "Hello, world!");
            counter.fetch_add(1, Ordering::Relaxed);
        });
    }

    let mut publisher = pub_node.advertise::<StringMsg>("hoge").unwrap();

    let msg = StringMsg {
        data: "Hello, world!".to_string(),
        ..Default::default()
    };

    assert!(publisher.publish(&msg));
    assert!(publisher.publish(&msg));

    thread::sleep(Duration::from_millis(5));

    assert_eq!(counter.load(Ordering::Relaxed), 2);
}

#[test]
fn test_node_pub_sub_include_null_characters() {
    let partition = Uuid::new_v4().to_string();

    let mut pub_node = Node::with_partition(&partition).unwrap();
    let mut sub_node = Node::with_partition(&partition).unwrap();

    let counter = Arc::new(AtomicU8::new(0));
    {
        let counter = Arc::clone(&counter);
        sub_node.subscribe::<Vector3d, _>("hoge", move |msg| {
            assert_eq!(msg.x, 1.0);
            assert_eq!(msg.y, 0.0);
            assert_eq!(msg.z, 0.0);
            counter.fetch_add(1, Ordering::Relaxed);
        });
    }

    let mut publisher = pub_node.advertise::<Vector3d>("hoge").unwrap();

    let msg = Vector3d {
        x: 1.0,
        ..Default::default()
    };

    assert!(publisher.publish(&msg));
    assert!(publisher.publish(&msg));

    thread::sleep(Duration::from_millis(5));

    assert_eq!(counter.load(Ordering::Relaxed), 2);
}
