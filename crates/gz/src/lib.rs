#![doc = include_str!("../README.md")]

#[cfg(all(feature = "fortress", feature = "garden"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");
#[cfg(all(feature = "garden", feature = "harmonic"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");
#[cfg(all(feature = "fortress", feature = "harmonic"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");

pub use gz_msgs as msgs;
pub use gz_transport as transport;
