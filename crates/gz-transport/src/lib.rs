#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::nursery
)]

#[cfg(all(feature = "fortress", feature = "garden"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");
#[cfg(all(feature = "garden", feature = "harmonic"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");
#[cfg(all(feature = "fortress", feature = "harmonic"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");

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
