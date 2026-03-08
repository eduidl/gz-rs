use pkg_config::{Config, Library};

/// (feature_name, versioned_pkg_config_name, Option<(unversioned_name, min_version, max_version)>)
///
/// Some Gazebo releases stopped using versioned pkg-config names (e.g. gz-transport15 ships
/// `gz-transport.pc` instead of `gz-transport15.pc`).  For those we first try the versioned name
/// for forward-compatibility, then fall back to the unversioned name with an explicit version range
/// so that future versions using the same unversioned name are not mistakenly identified.
const TRANSPORT_VERSIONS: &[(&str, &str, Option<(&str, &str, &str)>)] = &[
    ("jetty", "gz-transport15", Some(("gz-transport", "15", "16"))),
    ("ionic", "gz-transport14", None),
    ("harmonic", "gz-transport13", None),
    ("garden", "gz-transport12", None),
    ("fortress", "ignition-transport11", None),
];

fn feature_enabled(feature: &str) -> bool {
    std::env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

/// Try versioned name first; if not found and an unversioned alternative with a version range is
/// provided, try that too.
fn probe_transport(
    versioned: &str,
    unversioned: Option<(&str, &str, &str)>,
) -> Option<Library> {
    if let Ok(lib) = Config::new().probe(versioned) {
        return Some(lib);
    }
    if let Some((pkg, min, max)) = unversioned {
        if let Ok(lib) = Config::new().range_version(min..max).probe(pkg) {
            return Some(lib);
        }
    }
    None
}

/// Panics if more than one Gazebo version feature is enabled simultaneously.
pub fn check_exclusive_version_features() {
    let count = TRANSPORT_VERSIONS
        .iter()
        .filter(|(f, _, _)| feature_enabled(f))
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
    for (feature, pkg, alt) in TRANSPORT_VERSIONS {
        if feature_enabled(feature) {
            return probe_transport(pkg, *alt)
                .unwrap_or_else(|| panic!("{feature} feature requires {pkg}"));
        }
    }

    // fallback: auto-detect newest available
    for (feature, pkg, alt) in TRANSPORT_VERSIONS {
        if let Some(lib) = probe_transport(pkg, *alt) {
            println!("cargo:rustc-cfg=feature=\"{feature}\"");
            return lib;
        }
    }

    panic!("Any Gazebo transport is not found");
}
