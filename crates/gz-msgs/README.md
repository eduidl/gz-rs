# gz-msgs

[![crates.io](https://img.shields.io/crates/v/gz-msgs.svg)](https://crates.io/crates/gz-msgs)

Rust implementation of [Gazebo Messages](https://github.com/gazebosim/gz-msgs) using [rust-protobuf](https://crates.io/crates/protobuf).

| Gazebo version | Gazebo Messages version                                                 |
| -------------- | ----------------------------------------------------------------------- |
| Fortress (LTS) | [8.7.1](https://github.com/gazebosim/gz-msgs/tree/ignition-msgs8_8.7.1) |
| Garden (EOL)   | [9.5.1](https://github.com/gazebosim/gz-msgs/tree/gz-msgs9_9.5.1)       |
| Harmonic (LTS) | [10.3.2](https://github.com/gazebosim/gz-msgs/tree/gz-msgs10_10.3.2)    |
| Ionic          | [11.1.0](https://github.com/gazebosim/gz-msgs/tree/gz-msgs11_11.1.0)    |
| Jetty (LTS)    | [12.0.1](https://github.com/gazebosim/gz-msgs/tree/gz-msgs12_12.0.1)    |

This crate supports multiple versions of Gazebo Messages. The version is determined by the feature flag or by pkg-config.

If a feature flag (`fortress`, `garden`, `harmonic`, `ionic`, or `jetty`) is specified, messages from that version will be used. Specifying more than one will result in a compile error.

```toml
[dependencies]
gz-msgs = { version = "0.10.0", features = ["harmonic"] }
```

If no feature flag is specified, the version is determined using pkg-config. When multiple versions are installed, the newest takes precedence. To use an older version, set the feature flag as above.

## Note

For consistency with [gz-transport](https://docs.rs/gz-transport/latest/gz_transport/), pkg-config checks the version of Gazebo Transport instead of Gazebo Messages. For example, if Garden's `gz-transport12` is installed alongside Harmonic's `libgz-msgs10`, the `garden` version will be selected.
