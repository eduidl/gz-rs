#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::nursery
)]


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

mod node;
mod publisher;
mod string;

pub use node::Node;
pub use publisher::Publisher;

/// Block the current thread until a SIGINT or SIGTERM is received.
///
/// # Example
///
/// ```no_run
/// gz_transport::wait_for_shutdown();
/// ```
pub fn wait_for_shutdown() {
    unsafe { gz_transport_sys::waitForShutdown() };
}
