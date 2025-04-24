#![doc = include_str!("../README.md")]

#[cfg(any(
    all(feature = "fortress", feature = "garden"),
    all(feature = "fortress", feature = "harmonic"),
    all(feature = "fortress", feature = "ionic"),
    all(feature = "garden", feature = "harmonic"),
    all(feature = "garden", feature = "ionic"),
    all(feature = "harmonic", feature = "ionic"),
))]
compile_error!(
    "Only one of the following features can be enabled: fortress, garden, harmonic, ionic"
);

pub use gz_msgs as msgs;
pub use gz_transport as transport;
