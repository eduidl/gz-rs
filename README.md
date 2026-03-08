# gz-rs

| crates       | version                                                                                                 |
| ------------ | ------------------------------------------------------------------------------------------------------- |
| gz           | [![crates.io](https://img.shields.io/crates/v/gz.svg)](https://crates.io/crates/gz)                     |
| gz-msgs      | [![crates.io](https://img.shields.io/crates/v/gz-msgs.svg)](https://crates.io/crates/gz-msgs)           |
| gz-transport | [![crates.io](https://img.shields.io/crates/v/gz-transport.svg)](https://crates.io/crates/gz-transport) |

Rust bindings for Gazebo (not Gazebo Classic).

## About Gazebo versions

This crate supports the following versions of Gazebo.

- Fortress (LTS)
- Garden (EOL)
- Harmonic (LTS)
- Ionic
- Jetty (LTS)

The Gazebo version can be specified via a feature flag (`fortress`, `garden`, `harmonic`, `ionic`, or `jetty`). If no feature flag is specified, the version is determined using pkg-config. When multiple versions are installed, the newest takes precedence. To use an older version, set the feature flag as above.

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

```text
$ cargo run -p gz-examples --bin sub
Subscribed: Count 0
Subscribed: Count 1
Subscribed: Count 2
Subscribed: Count 3
Subscribed: Count 4
...
```

In a separate terminal, run:

```text
$ cargo run -p gz-examples --bin pub
Publishing: Count 0
Publishing: Count 1
Publishing: Count 2
Publishing: Count 3
Publishing: Count 4
...
```
