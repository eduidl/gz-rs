use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use gz_msgs::{stringmsg::StringMsg, vector3d::Vector3d};
use gz_transport::Node;
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

    assert!(node.subscribe("hoge", |_msg: StringMsg| {}));
    assert!(node.subscribe("/fuga/hoge", |_msg: StringMsg| {}));

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

    let (tx, rx) = crossbeam_channel::unbounded();
    assert!(sub_node.subscribe("hoge", move |msg: StringMsg| {
        let _ = tx.send(msg);
    }));

    let mut publisher = pub_node.advertise("hoge").unwrap();

    let msg = StringMsg {
        data: "Hello, world!".to_string(),
        ..Default::default()
    };

    assert!(publisher.publish(&msg));
    assert!(publisher.publish(&msg));

    for _ in 0..2 {
        assert_eq!(rx.recv_timeout(Duration::from_secs(10)).unwrap(), msg);
    }
}

#[test]
fn test_node_pub_sub_include_null_characters() {
    let partition = Uuid::new_v4().to_string();

    let mut pub_node = Node::with_partition(&partition).unwrap();
    let mut sub_node = Node::with_partition(&partition).unwrap();

    let (tx, rx) = crossbeam_channel::unbounded();
    assert!(sub_node.subscribe("hoge", move |msg: Vector3d| {
        let _ = tx.send(msg);
    }));

    let mut publisher = pub_node.advertise("hoge").unwrap();

    let msg = Vector3d {
        x: 1.0,
        ..Default::default()
    };

    assert!(publisher.publish(&msg));
    assert!(publisher.publish(&msg));

    for _ in 0..2 {
        assert_eq!(rx.recv_timeout(Duration::from_secs(10)).unwrap(), msg);
    }
}

#[test]
fn test_node_subscribe_runs_on_local_publisher_thread() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let (tx, rx) = crossbeam_channel::unbounded();
    let count = AtomicUsize::new(0);
    assert!(node.subscribe("count", move |_: StringMsg| {
        tx.send((
            count.fetch_add(1, Ordering::SeqCst) + 1,
            thread::current().id(),
        ))
        .unwrap();
    }));
    let mut publisher = node.advertise::<StringMsg>("count").unwrap();
    for expected in 1..=3 {
        assert!(publisher.publish(&StringMsg::default()));
        assert_eq!(rx.try_recv().unwrap(), (expected, thread::current().id()));
    }
}

#[test]
fn test_node_subscribe_allows_concurrent_publishers() {
    const PUBLISHERS: usize = 4;
    const MESSAGES_PER_PUBLISHER: usize = 20;
    const TIMEOUT: Duration = Duration::from_secs(10);

    let partition = Uuid::new_v4().to_string();
    let mut node = Node::with_partition(&partition).unwrap();
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let (result_tx, result_rx) = crossbeam_channel::unbounded();
    let count = AtomicUsize::new(0);
    let (entered_tx, entered_rx) = crossbeam_channel::unbounded();
    let (release_tx, release_rx) = crossbeam_channel::bounded(PUBLISHERS);
    assert!(node.subscribe("count", {
        let active = Arc::clone(&active);
        let max_active = Arc::clone(&max_active);
        move |_: StringMsg| {
            let current = active.fetch_add(1, Ordering::SeqCst) + 1;
            max_active.fetch_max(current, Ordering::SeqCst);
            let number = count.fetch_add(1, Ordering::SeqCst) + 1;
            if number <= PUBLISHERS {
                entered_tx.send(()).unwrap();
                release_rx.recv_timeout(TIMEOUT).unwrap();
            }
            active.fetch_sub(1, Ordering::SeqCst);
            result_tx.send(number).unwrap();
        }
    }));

    let (ready_tx, ready_rx) = crossbeam_channel::unbounded();
    let (start_tx, start_rx) = crossbeam_channel::bounded(PUBLISHERS);
    let (done_tx, done_rx) = crossbeam_channel::unbounded();
    let mut workers = Vec::new();
    for _ in 0..PUBLISHERS {
        let partition = partition.clone();
        let ready_tx = ready_tx.clone();
        let start_rx = start_rx.clone();
        let done_tx = done_tx.clone();
        workers.push(thread::spawn(move || {
            // Node and Publisher stay on the thread that created them.
            let mut node = Node::with_partition(&partition).unwrap();
            let mut publisher = node.advertise::<StringMsg>("count").unwrap();
            ready_tx.send(()).unwrap();
            start_rx.recv_timeout(TIMEOUT).unwrap();
            for _ in 0..MESSAGES_PER_PUBLISHER {
                assert!(publisher.publish(&StringMsg::default()));
            }
            drop(publisher);
            drop(node);
            done_tx.send(()).unwrap();
        }));
    }
    drop(ready_tx);
    drop(done_tx);
    drop(start_rx);

    let deadline = Instant::now() + TIMEOUT;
    for _ in 0..PUBLISHERS {
        ready_rx.recv_deadline(deadline).unwrap();
    }
    for _ in 0..PUBLISHERS {
        start_tx.send(()).unwrap();
    }
    // All publishers must be able to enter before any of them is released.
    for _ in 0..PUBLISHERS {
        entered_rx.recv_deadline(deadline).unwrap();
    }
    for _ in 0..PUBLISHERS {
        release_tx.send(()).unwrap();
    }
    let mut received = (0..PUBLISHERS * MESSAGES_PER_PUBLISHER)
        .map(|_| result_rx.recv_deadline(deadline).unwrap())
        .collect::<Vec<_>>();
    received.sort_unstable();
    assert_eq!(
        received,
        (1..=PUBLISHERS * MESSAGES_PER_PUBLISHER).collect::<Vec<_>>()
    );
    // Bound completion waits before joining, including native cleanup.
    for _ in 0..PUBLISHERS {
        done_rx.recv_deadline(deadline).unwrap();
    }
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(active.load(Ordering::SeqCst), 0);
    assert_eq!(max_active.load(Ordering::SeqCst), PUBLISHERS);
    assert!(result_rx.is_empty());
}

const CALLBACK_TIMEOUT: Duration = Duration::from_secs(10);

struct NotifyOnDrop(crossbeam_channel::Sender<()>);

impl Drop for NotifyOnDrop {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

#[test]
fn test_node_callback_can_publish_to_its_own_topic() {
    let partition = Uuid::new_v4().to_string();
    let mut node = Node::with_partition(&partition).unwrap();
    let (tx, rx) = crossbeam_channel::unbounded();
    assert!(node.subscribe("echo", {
        let partition = partition.clone();
        move |msg: StringMsg| {
            if msg.data == "initial" {
                // Create non-Send Node/Publisher on the calling thread.
                let mut node = Node::with_partition(&partition).unwrap();
                let mut publisher = node.advertise::<StringMsg>("echo").unwrap();
                assert!(publisher.publish(&StringMsg {
                    data: "reply".into(),
                    ..Default::default()
                }));
            }
            // The nested callback completes before the initial callback returns.
            let _ = tx.send((msg.data, thread::current().id()));
        }
    }));
    let mut publisher = node.advertise::<StringMsg>("echo").unwrap();
    assert!(publisher.publish(&StringMsg {
        data: "initial".into(),
        ..Default::default()
    }));
    assert_eq!(
        rx.try_recv().unwrap(),
        ("reply".into(), thread::current().id())
    );
    assert_eq!(
        rx.try_recv().unwrap(),
        ("initial".into(), thread::current().id())
    );
}

#[test]
fn test_node_channel_overflow_and_unsubscribe() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let rx = node.subscribe_channel::<StringMsg>("queue", 2).unwrap();
    let mut publisher = node.advertise::<StringMsg>("queue").unwrap();
    for value in ["first", "second", "third"] {
        assert!(publisher.publish(&StringMsg {
            data: value.into(),
            ..Default::default()
        }));
    }
    assert!(node.unsubscribe("queue"));
    assert_eq!(rx.try_recv().unwrap().data, "second");
    assert_eq!(rx.try_recv().unwrap().data, "third");
    assert!(matches!(
        rx.try_recv(),
        Err(async_channel::TryRecvError::Closed)
    ));
}

#[test]
#[should_panic(expected = "capacity cannot be zero")]
fn test_node_channel_rejects_zero_capacity() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let _ = node.subscribe_channel::<StringMsg>("queue", 0);
}

#[test]
fn test_node_channel_dropped_receiver() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let rx = node.subscribe_channel::<StringMsg>("queue", 1).unwrap();
    let mut publisher = node.advertise::<StringMsg>("queue").unwrap();
    drop(rx);
    assert!(publisher.publish(&StringMsg::default()));
}

#[test]
fn test_node_failed_subscription_releases_captures() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let (tx, rx) = crossbeam_channel::bounded(1);
    let guard = NotifyOnDrop(tx);
    assert!(!node.subscribe("/invalid topic", move |_: StringMsg| {
        let _ = &guard;
    }));
    rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
    assert!(node.subscribed_topics().is_empty());
}

#[test]
fn test_node_channel_processing_can_use_local_mutable_state() {
    use std::{cell::RefCell, rc::Rc};
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let rx = node.subscribe_channel::<StringMsg>("local", 2).unwrap();
    let mut publisher = node.advertise::<StringMsg>("local").unwrap();
    assert!(publisher.publish(&StringMsg {
        data: "received".into(),
        ..Default::default()
    }));
    let state = Rc::new(RefCell::new(String::new()));
    *state.borrow_mut() = rx.try_recv().unwrap().data;
    assert_eq!(*state.borrow(), "received");
}
