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
gz-transport = { version = "0.10.0", features = ["harmonic"] }
```

If no feature flag is specified, the version is determined using pkg-config. When multiple versions are installed, the newest takes precedence. To use an older version, set the feature flag as above.
