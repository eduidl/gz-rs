# gz-rs

[![crates.io](https://img.shields.io/crates/v/gz.svg)](https://crates.io/crates/gz)
[![crates.io](https://img.shields.io/crates/v/gz-msgs.svg)](https://crates.io/crates/gz-msgs)

Rust binding for Gazebo (not Gazebo Classic).

## Dependencies

|          | gz-msgs | gz-transport |
| -------- | ------- | ------------ |
| Fortress | 8       | 11           |
| Garden   | 9       | 12           |
| Harmonic | 10      | 13           |

## Examples

```no_rust
$ cargo run -p gz-examples --bin sub --features [fortress|garden|harmonic]
Hello, world! 0
Hello, world! 1
Hello, world! 2
Hello, world! 3
Hello, world! 4
:
```

Execute the following in another terminal

```no_rust
$ cargo run -p gz-examples --bin pub --features [fortress|garden|harmonic]
Publishing: 0
Publishing: 1
Publishing: 2
Publishing: 3
Publishing: 4
:
```
