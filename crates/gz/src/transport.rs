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
/// gz::transport::wait_for_shutdown();
/// ```
pub fn wait_for_shutdown() {
    unsafe { gz_transport_sys::waitForShutdown() };
}
