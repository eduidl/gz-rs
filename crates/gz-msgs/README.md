# gz-msgs

[![crates.io](https://img.shields.io/crates/v/gz-msgs.svg)](https://crates.io/crates/gz-msgs)

Rust implementation of [Gazebo Messages](https://github.com/gazebosim/gz-msgs) using [rust-protobuf](https://crates.io/crates/protobuf).

| Gazebo version | Gazebo Messages version                                                 |
| -------------- | ----------------------------------------------------------------------- |
| Fortress       | [8.7.0](https://github.com/gazebosim/gz-msgs/tree/ignition-msgs8_8.7.0) |
| Garden (EOL)   | [9.5.1](https://github.com/gazebosim/gz-msgs/tree/gz-msgs9_9.5.1)       |
| Harmonic       | [10.3.2](https://github.com/gazebosim/gz-msgs/tree/gz-msgs10_10.3.2)    |
| Ionic          | [11.0.2](https://github.com/gazebosim/gz-msgs/tree/gz-msgs11_11.0.2)    |

This crate is supporting multiple versions of Gazebo messages. The version is determined by the feature flag or using pkg-config.

If you specify any feature flag (`fortress`, `garden`, `harmonic` or `ionic`), the messages from the specified version will be used. Naturally, if more than one is specified, a compile error will occur.

```toml
[dependencies]
gz-msgs = { version = "0.9.0", features = ["harmonic"] }
```

Otherwise, no feature flag is specified, the version is determined by using pkg-config. When multiple versions are installed, the newer version takes precedence. If you want to use an older version, set the feature flag as above.

## Note

For consistency with [gz-transport](https://docs.rs/gz-transport/latest/gz_transport/), pkg-config checkes the version of Gazebo Transport instead Gazebo Messages. If `gz-garden` and `libgz-msgs10` are installed, `garden` will be used.
