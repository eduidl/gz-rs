#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc = "### Note")]
#![cfg_attr(
    docsrs,
    doc = "In docs.rs, documentation for all features are provided. However, when you use this crate, you can only use one of the modules, and its contents are re-exported directly under the crate."
)]

pub use gz_msgs_common::protobuf;

#[cfg(any(feature = "fortress", feature = "generate", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "fortress")))]
/// [Gazebo Message 8.7.0](https://github.com/gazebosim/gz-msgs/tree/ignition-msgs8_8.7.0)
pub mod ign_msgs8;
#[cfg(feature = "fortress")]
pub use ign_msgs8::*;

#[cfg(any(feature = "garden", feature = "generate", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "garden")))]
/// [Gazebo Message 9.5.1](https://github.com/gazebosim/gz-msgs/tree/gz-msgs9_9.5.1)
pub mod gz_msgs9;
#[cfg(feature = "garden")]
pub use gz_msgs9::*;

#[cfg(any(feature = "harmonic", feature = "generate", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "harmonic")))]
/// [Gazebo Message 10.3.2](https://github.com/gazebosim/gz-msgs/tree/gz-msgs10_10.3.2)
pub mod gz_msgs10;
#[cfg(feature = "harmonic")]
pub use gz_msgs10::*;

#[cfg(any(feature = "ionic", feature = "generate", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "ionic")))]
/// [Gazebo Message 11.0.2](https://github.com/gazebosim/gz-msgs/tree/gz-msgs11_11.0.2)
pub mod gz_msgs11;
#[cfg(feature = "ionic")]
pub use gz_msgs11::*;
