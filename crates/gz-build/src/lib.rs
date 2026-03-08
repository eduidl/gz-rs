use pkg_config::{Config, Library};

/// (feature_name, pkg_config_name) pairs, newest-first.
const TRANSPORT_VERSIONS: &[(&str, &str)] = &[
    ("jetty", "gz-transport15"),
    ("ionic", "gz-transport14"),
    ("harmonic", "gz-transport13"),
    ("garden", "gz-transport12"),
    ("fortress", "ignition-transport11"),
];

fn feature_enabled(feature: &str) -> bool {
    std::env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

/// Panics if more than one Gazebo version feature is enabled simultaneously.
pub fn check_exclusive_version_features() {
    let count = TRANSPORT_VERSIONS
        .iter()
        .filter(|(f, _)| feature_enabled(f))
        .count();
    if count > 1 {
        panic!("Only one of the following features can be enabled: fortress, garden, harmonic, ionic, jetty");
    }
}

/// Returns the pkg-config `Library` for the selected (or auto-detected) Gazebo transport.
///
/// If a version feature is explicitly enabled, probes that library and panics if not found.
/// Otherwise probes newest-to-oldest and emits `cargo:rustc-cfg=feature="<name>"` for the
/// first match.  Panics if no Gazebo transport is found at all.
pub fn find_transport_library() -> Library {
    for (feature, pkg) in TRANSPORT_VERSIONS {
        if feature_enabled(feature) {
            return Config::new()
                .probe(pkg)
                .unwrap_or_else(|_| panic!("{feature} feature requires {pkg}"));
        }
    }

    // fallback: auto-detect newest available
    for (feature, pkg) in TRANSPORT_VERSIONS {
        if let Ok(lib) = Config::new().probe(pkg) {
            println!("cargo:rustc-cfg=feature=\"{feature}\"");
            return lib;
        }
    }

    panic!("Any Gazebo transport is not found");
}
