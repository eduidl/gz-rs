# Gazebo Transport integration tests

This non-published workspace package keeps the service request regression tests
and their native C++ echo service together. It depends on the transport crates;
the published crates do not depend on this package.

Run with `cargo test -p gz-transport-tests`, or as part of `cargo test --workspace`.
Gazebo is detected automatically. To select a version explicitly, use a feature
such as `cargo test -p gz-transport-tests --features jetty`.
