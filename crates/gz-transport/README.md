# gz-transport

[![crates.io](https://img.shields.io/crates/v/gz-transport.svg)](https://crates.io/crates/gz-transport)

Rust wrapper for [Gazebo Transport](https://github.com/gazebosim/gz-transport).

| Gazebo version | Gazebo Transport version |
| -------------- | ------------------------ |
| Fortress       | 11                       |
| Garden (EOL)   | 12                       |
| Harmonic       | 13                       |
| Ionic          | 14                       |

This crate is supporting multiple versions of Gazebo Transport. The version is determined by the feature flag or using pkg-config.
If any feature flag (`fortress`, `garden`, `harmonic` or `ionic`) is specified, the related version of gz-transport library will be linked. Naturally, if more than one is specified, a compile error will occur.

```toml
[dependencies]
gz-transport = { version = "0.9.0", features = ["harmonic"] }
```

Otherwise, no feature flag is specified, the version if determined by using pkg-config. When multiple versions are installed, the newer version takes precedence. If you want to use an older version, set the feature flag as above.
