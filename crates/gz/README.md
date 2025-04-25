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
gz = { version = "0.9.0", features = ["harmonic"] }
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

### Example: Subscribe (callback)

```rust,no_run
use gz::{msgs::stringmsg::StringMsg, transport::Node};

let mut node = Node::new().unwrap();
node.subscribe("topic_name", |msg: StringMsg| {
    println!("Subscribed: {}", msg.data);
});

gz::transport::wait_for_shutdown();
```

### Example: Subscribe (channel)

`subscribe_channel` also uses callbacks internally. However, `subscribe_channel` may be easier to use when ownership is involved.

```rust,no_run
use gz::{msgs::stringmsg::StringMsg, transport::Node};

let mut node = Node::new().unwrap();
let rx = node.subscribe_channel::<StringMsg>("topic_name", 10).unwrap();

for msg in rx {
    println!("Received: {}", msg.data);
}
```
