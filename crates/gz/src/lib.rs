#![doc = include_str!("../../../README.md")]
#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::nursery
)]

pub use crossbeam_channel;
#[cfg(feature = "harmonic")]
pub use gz_msgs10 as msgs;
#[cfg(feature = "fortress")]
pub use gz_msgs8 as msgs;
#[cfg(feature = "garden")]
pub use gz_msgs9 as msgs;

/// Gazebo transport bindings
pub mod transport;
