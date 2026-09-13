# gz-transport

[![crates.io](https://img.shields.io/crates/v/gz-transport.svg)](https://crates.io/crates/gz-transport)

Rust wrapper for [Gazebo Transport](https://github.com/gazebosim/gz-transport).

| Gazebo version | Gazebo Transport version |
| -------------- | ------------------------ |
| Fortress (LTS) | 11                       |
| Garden (EOL)   | 12                       |
| Harmonic (LTS) | 13                       |
| Ionic          | 14                       |
| Jetty (LTS)    | 15                       |

This crate supports multiple versions of Gazebo Transport. The version is determined by the feature flag or by pkg-config.

If a feature flag (`fortress`, `garden`, `harmonic`, `ionic`, or `jetty`) is specified, the corresponding version of the gz-transport library will be linked. Specifying more than one will result in a compile error.

```toml
[dependencies]
gz-transport = { version = "0.11.0", features = ["harmonic"] }
```

If no feature flag is specified, the version is determined using pkg-config. When multiple versions are installed, the newest takes precedence. To use an older version, set the feature flag as above.

## Receiving messages

Prefer `Node::subscribe_channel` to process messages on a thread you control.
It is suitable for sequential state updates and expensive processing. No worker
thread is created by the library, and your receiving loop needs no `Send` or
`Sync` callback. For example:

```rust,no_run
use gz_msgs::stringmsg::StringMsg;
use gz_transport::Node;

let mut node = Node::new().unwrap();
let rx = node.subscribe_channel::<StringMsg>("topic", 10).unwrap();
let mut count = 0;
while let Ok(msg) = rx.recv_blocking() {
    count += 1;
    println!("Message {count}: {}", msg.data);
}
```

Native callbacks decode and enqueue owned messages without waiting for the
receiver. When full, the channel evicts the oldest queued message with a warning
and enqueues the new one, retaining the latest N messages. Capacity must be at
least one. This is a Rust-side queue, not a
transport delivery guarantee; messages may also be lost upstream.

The returned receiver is an `async_channel::Receiver<T>`. Use `recv_blocking()`
in synchronous code or `recv().await` in asynchronous code.

`Node::subscribe` is the direct callback API. It accepts
`Fn(T) + Send + Sync + 'static` and adds no Rust worker thread, message queue, or
callback lock. Keep callbacks short. Gazebo may call them concurrently or
re-enter them on the same thread. Local raw publication can invoke callbacks
before `publish` returns. Do not hold a lock needed by a nested callback while
publishing, or wait for another callback to finish. A successful publish does
not guarantee remote processing has completed.

Unsubscription and Node destruction do not wait for native handlers already
acquired by Gazebo. Those handlers retain their context and may still invoke
callbacks or enqueue messages afterward. Channel receivers observe disconnection
once all native owners release their senders and the queue has been drained.
Dropping a receiver does not itself unsubscribe the topic.

Direct callbacks and their captured-value destructors must not panic: a panic
reaching the non-unwinding C ABI boundary aborts the process. Panics in a
channel's receiving loop are handled by the application, outside Gazebo.

### Migrating callback code

Callbacks requiring `FnMut` or capturing non-`Sync` values such as `Cell` should
move their processing into a `subscribe_channel` receiving loop. For concurrent
callbacks, use atomics or synchronized state as appropriate; such synchronization
must account for possible re-entry. Direct callbacks no longer run on a dedicated
worker or use the former fixed-capacity 1024-message queue.
