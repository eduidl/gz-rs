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
gz-transport = { version = "0.10.1", features = ["harmonic"] }
```

If no feature flag is specified, the version is determined using pkg-config. When multiple versions are installed, the newest takes precedence. To use an older version, set the feature flag as above.

## Receiving messages

`Node::subscribe_channel` returns a bounded channel of owned messages. Process it
on a thread you control; no subscription worker is created. Native callbacks
only decode messages and enqueue them without waiting for your processing.

`Node::subscribe` accepts `FnMut(T) + Send + 'static` and runs it asynchronously on
one dedicated worker per subscription, using a queue of 1024 messages. The
callback exclusively owns its state and does not need `Sync` or a wrapping Mutex.
Even local publishing returns without waiting for the user callback. Publishing
from a callback back to its topic queues a later invocation; it must not wait for
that invocation to complete.

Both APIs drop the incoming message with a warning when their queue is full.
This changes `subscribe_channel` overflow behavior: existing queued messages are
retained instead of evicting the oldest message.

Unsubscription and Node destruction signal workers to stop without joining them.
An already dispatched callback may finish afterward, then pending worker messages
are discarded. A callback panic ends that worker. Channel receivers can still
drain queued messages after unsubscription, then observe disconnection.
