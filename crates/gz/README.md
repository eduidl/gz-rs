# gz

[![crates.io](https://img.shields.io/crates/v/gz.svg)](https://crates.io/crates/gz)

This crate contains following crates and re-exports them.

- [gz-msgs](https://crates.io/crates/gz-msgs): Rust implementation of Gazebo Messages
- [gz-transport](https://crates.io/crates/gz-transport): Rust wrapper of Gazebo Transport

## Gazebo version

This crate is supporting following versions of Gazebo.

- Fortress
- Garden (EOL)
- Harmonic
- Ionic

Gazebo version can be specified by a feature flag (`fortress`, `garden`, `harmonic` or `ionic`). If not specified, the version is determined by using pkg-config. When multiple versions are installed, the newer version takes precedence. If you want to use an older version, set the feature flag as above.

```toml
[dependencies]
gz = { version = "0.10.1", features = ["harmonic"] }
```

## Dependencies

- Gazebo
  - gz-msgs
  - gz-transport
- pkg-config

## Examples

### Example: Publish

```rust
use gz::{msgs::stringmsg::StringMsg, transport::Node};

let mut node = Node::new().unwrap();
let mut publisher = node.advertise("topic_name").unwrap();

let topic = StringMsg {
    data: "Hello, world!".to_string(),
    ..Default::default()
};

assert!(publisher.publish(&topic));
```

### Example: Subscribe (channel, recommended)

Use `subscribe_channel` to process messages sequentially on your own thread.
The bounded queue retains the latest N messages, evicting the oldest when full.

```rust,no_run
use gz::{msgs::stringmsg::StringMsg, transport::Node};

let mut node = Node::new().unwrap();
let rx = node.subscribe_channel::<StringMsg>("topic_name", 10).unwrap();

while let Ok(msg) = rx.recv_blocking() {
    println!("Received: {}", msg.data);
}
```

### Example: Subscribe (direct callback)

`subscribe` requires `Fn + Send + Sync` and runs directly on Gazebo's calling
thread, without a Rust worker or queue. Callbacks must support concurrent calls
and re-entry, and should return quickly. Prefer channels when updating local
mutable state or doing expensive work. See the transport crate's documentation
for panic and unsubscription behavior.

```rust,no_run
use gz::{msgs::stringmsg::StringMsg, transport::Node};

let mut node = Node::new().unwrap();
assert!(node.subscribe("topic_name", |msg: StringMsg| {
    println!("Subscribed: {}", msg.data);
}));

gz::transport::wait_for_shutdown();
```
