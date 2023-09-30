#![doc = include_str!("../../../README.md")]
#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::nursery
)]

pub use crossbeam_channel;

/// Gazebo transport bindings
pub mod transport;
