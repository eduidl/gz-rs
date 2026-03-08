use pkg_config::{Config, Library};

/// (feature_name, major_version, try_unversioned)
///
/// The pkg-config prefix is determined by `prefix()` and combined with `major` to form the
/// versioned name (e.g. "gz-transport" + 15 → "gz-transport15").
/// Some Gazebo releases stopped using versioned pkg-config names (e.g. gz-transport15 ships
/// `gz-transport.pc` instead of `gz-transport15.pc`).  Set `try_unversioned` to also probe
/// the prefix alone (e.g. "gz-transport") with range `>= major, < major+1`.
const TRANSPORT_VERSIONS: &[(&str, u8, bool)] = &[
    ("jetty", 15, true),
    ("ionic", 14, false),
    ("harmonic", 13, false),
    ("garden", 12, false),
    ("fortress", 11, false),
];

fn resolve_transport_prefix(feature: &str) -> &'static str {
    if feature == "fortress" {
        "ignition-transport"
    } else {
        "gz-transport"
    }
}

fn feature_enabled(feature: &str) -> bool {
    std::env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

/// Try the versioned name first; if not found and `try_unversioned` is set, also try the prefix
/// alone with a version range `>= major, < major+1`.
fn probe_transport(feature: &str, major: u8, try_unversioned: bool) -> Option<Library> {
    let prefix = resolve_transport_prefix(feature);
    let versioned = format!("{prefix}{major}");
    if let Ok(lib) = Config::new().probe(&versioned) {
        return Some(lib);
    }
    if try_unversioned {
        let min = major.to_string();
        let max = (major + 1).to_string();
        if let Ok(lib) = Config::new()
            .range_version(min.as_str()..max.as_str())
            .probe(prefix)
        {
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
        panic!(
            "Only one of the following features can be enabled: fortress, garden, harmonic, ionic, jetty"
        );
    }
}

/// Returns the pkg-config `Library` for the selected (or auto-detected) Gazebo transport.
///
/// If a version feature is explicitly enabled, probes that library and panics if not found.
/// Otherwise probes newest-to-oldest and emits `cargo:rustc-cfg=feature="<name>"` for the
/// first match.  Panics if no Gazebo transport is found at all.
pub fn find_transport_library() -> Library {
    for &(feature, major, try_unversioned) in TRANSPORT_VERSIONS {
        if feature_enabled(feature) {
            return probe_transport(feature, major, try_unversioned).unwrap_or_else(|| {
                panic!(
                    "{feature} feature requires {prefix}{major}",
                    prefix = resolve_transport_prefix(feature)
                )
            });
        }
    }

    // fallback: auto-detect newest available
    for &(feature, major, try_unversioned) in TRANSPORT_VERSIONS {
        if let Some(lib) = probe_transport(feature, major, try_unversioned) {
            println!("cargo:rustc-cfg=feature=\"{feature}\"");
            return lib;
        }
    }

    panic!("Any Gazebo transport is not found");
}
