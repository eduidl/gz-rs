use std::{
    cell::Cell,
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
fn test_node_subscribe_send_not_sync_callback() {
    let partition = Uuid::new_v4().to_string();
    let mut node = Node::with_partition(&partition).unwrap();
    let (tx, rx) = crossbeam_channel::unbounded();

    // Cell is Send but not Sync. Reassigning it also requires FnMut.
    let mut count = Cell::new(0);
    assert!(node.subscribe("count", move |_: StringMsg| {
        count = Cell::new(count.get() + 1);
        let _ = tx.send(count.get());
    }));

    let mut publisher = node.advertise::<StringMsg>("count").unwrap();
    for expected in 1..=3 {
        assert!(publisher.publish(&StringMsg::default()));
        assert_eq!(rx.recv_timeout(Duration::from_secs(10)).unwrap(), expected);
    }
}

#[test]
fn test_node_subscribe_serializes_concurrent_publishers() {
    const PUBLISHERS: usize = 4;
    const MESSAGES_PER_PUBLISHER: usize = 20;
    const TIMEOUT: Duration = Duration::from_secs(10);

    let partition = Uuid::new_v4().to_string();
    let mut node = Node::with_partition(&partition).unwrap();
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let (result_tx, result_rx) = crossbeam_channel::unbounded();
    let mut count = 0;
    assert!(node.subscribe("count", {
        let active = Arc::clone(&active);
        let max_active = Arc::clone(&max_active);
        move |_: StringMsg| {
            let current = active.fetch_add(1, Ordering::SeqCst) + 1;
            max_active.fetch_max(current, Ordering::SeqCst);
            // Give another publisher time to enter while this callback is active.
            thread::sleep(Duration::from_millis(1));
            count += 1;
            active.fetch_sub(1, Ordering::SeqCst);
            let _ = result_tx.send(count);
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
    for expected in 1..=PUBLISHERS * MESSAGES_PER_PUBLISHER {
        assert_eq!(result_rx.recv_deadline(deadline).unwrap(), expected);
    }
    // Bound completion waits before joining, including native cleanup.
    for _ in 0..PUBLISHERS {
        done_rx.recv_deadline(deadline).unwrap();
    }
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(active.load(Ordering::SeqCst), 0);
    assert_eq!(max_active.load(Ordering::SeqCst), 1);
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
                // Create non-Send Node/Publisher on the worker itself.
                let mut node = Node::with_partition(&partition).unwrap();
                let mut publisher = node.advertise::<StringMsg>("echo").unwrap();
                assert!(publisher.publish(&StringMsg {
                    data: "reply".into(),
                    ..Default::default()
                }));
            }
            // This must happen after nested publish returns, before the reply
            // is processed by the next invocation of this same callback.
            let _ = tx.send((msg.data, thread::current().id()));
        }
    }));
    let mut publisher = node.advertise::<StringMsg>("echo").unwrap();
    assert!(publisher.publish(&StringMsg {
        data: "initial".into(),
        ..Default::default()
    }));
    let (initial, worker) = rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
    assert_eq!(initial, "initial");
    assert_ne!(worker, thread::current().id());
    assert_eq!(
        rx.recv_timeout(CALLBACK_TIMEOUT).unwrap(),
        ("reply".into(), worker)
    );
}

#[test]
fn test_node_channel_overflow_and_unsubscribe() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let rx = node.subscribe_channel::<StringMsg>("queue", 2).unwrap();
    let mut publisher = node.advertise::<StringMsg>("queue").unwrap();
    for value in ["first", "second", "dropped"] {
        assert!(publisher.publish(&StringMsg {
            data: value.into(),
            ..Default::default()
        }));
    }
    assert!(node.unsubscribe("queue"));
    assert_eq!(rx.recv_timeout(CALLBACK_TIMEOUT).unwrap().data, "first");
    assert_eq!(rx.recv_timeout(CALLBACK_TIMEOUT).unwrap().data, "second");
    assert!(matches!(
        rx.recv_timeout(CALLBACK_TIMEOUT),
        Err(crossbeam_channel::RecvTimeoutError::Disconnected)
    ));
}

#[test]
fn test_node_channel_zero_capacity_and_dropped_receiver() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let rx = node.subscribe_channel::<StringMsg>("queue", 0).unwrap();
    let mut publisher = node.advertise::<StringMsg>("queue").unwrap();
    // No waiting receiver: publication must return without blocking.
    assert!(publisher.publish(&StringMsg::default()));
    assert!(rx.is_empty());
    drop(rx);
    assert!(publisher.publish(&StringMsg::default()));
}

#[test]
fn test_node_stops_workers_without_waiting_for_user_code() {
    for unsubscribe in [true, false] {
        let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
        let (called_tx, called_rx) = crossbeam_channel::unbounded();
        let (release_tx, release_rx) = crossbeam_channel::bounded(1);
        let (dropped_tx, dropped_rx) = crossbeam_channel::bounded(1);
        let guard = NotifyOnDrop(dropped_tx);
        assert!(node.subscribe("blocked", move |msg: StringMsg| {
            let _ = &guard;
            let _ = called_tx.send(msg.data);
            release_rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
        }));
        let mut publisher = node.advertise::<StringMsg>("blocked").unwrap();
        assert!(publisher.publish(&StringMsg {
            data: "running".into(),
            ..Default::default()
        }));
        assert_eq!(called_rx.recv_timeout(CALLBACK_TIMEOUT).unwrap(), "running");
        assert!(publisher.publish(&StringMsg {
            data: "pending".into(),
            ..Default::default()
        }));
        if unsubscribe {
            assert!(node.unsubscribe("blocked"));
        }
        drop(node);
        // Cancellation does not join a callback waiting on this very thread.
        assert!(matches!(
            dropped_rx.try_recv(),
            Err(crossbeam_channel::TryRecvError::Empty)
        ));
        release_tx.send(()).unwrap();
        dropped_rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
        assert!(matches!(
            called_rx.recv_timeout(CALLBACK_TIMEOUT),
            Err(crossbeam_channel::RecvTimeoutError::Disconnected)
        ));
    }
}

#[test]
fn test_node_failed_subscription_stops_worker() {
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
fn test_node_worker_queue_overflow_does_not_block_publish() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let (tx, rx) = crossbeam_channel::unbounded();
    let (release_tx, release_rx) = crossbeam_channel::bounded(1);
    assert!(node.subscribe("queue", move |msg: StringMsg| {
        let is_first = msg.data == "first";
        let _ = tx.send(msg.data);
        if is_first {
            release_rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
        }
    }));
    let mut publisher = node.advertise::<StringMsg>("queue").unwrap();
    assert!(publisher.publish(&StringMsg {
        data: "first".into(),
        ..Default::default()
    }));
    assert_eq!(rx.recv_timeout(CALLBACK_TIMEOUT).unwrap(), "first");
    // The documented capacity is 1024; the worker is blocked above.
    for value in 0..1025 {
        assert!(publisher.publish(&StringMsg {
            data: value.to_string(),
            ..Default::default()
        }));
    }
    release_tx.send(()).unwrap();
    let deadline = Instant::now() + CALLBACK_TIMEOUT;
    for value in 0..1024 {
        assert_eq!(rx.recv_deadline(deadline).unwrap(), value.to_string());
    }
    assert!(publisher.publish(&StringMsg {
        data: "after overflow".into(),
        ..Default::default()
    }));
    assert_eq!(rx.recv_deadline(deadline).unwrap(), "after overflow");
}

#[test]
fn test_node_callback_panic_isolated_from_other_subscribers() {
    let mut node = Node::with_partition(&Uuid::new_v4().to_string()).unwrap();
    let (tx, rx) = crossbeam_channel::bounded(1);
    let guard = NotifyOnDrop(tx);
    assert!(node.subscribe("shared", move |_: StringMsg| {
        let _ = &guard;
        panic!("intentional subscription worker panic");
    }));
    // Adding another subscription must preserve the first FFI callback address.
    let surviving_rx = node.subscribe_channel::<StringMsg>("shared", 2).unwrap();
    let mut publisher = node.advertise::<StringMsg>("shared").unwrap();
    assert!(publisher.publish(&StringMsg::default()));
    rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
    surviving_rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
    // The dead worker's disconnected queue does not stop native delivery.
    assert!(publisher.publish(&StringMsg::default()));
    surviving_rx.recv_timeout(CALLBACK_TIMEOUT).unwrap();
    assert!(node.unsubscribe("shared"));
    assert!(matches!(
        surviving_rx.recv_timeout(CALLBACK_TIMEOUT),
        Err(crossbeam_channel::RecvTimeoutError::Disconnected)
    ));
}
