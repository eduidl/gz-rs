use std::{env, error::Error};

use pkg_config::{Config, Library};

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

fn find_library() -> Library {
    if cfg!(feature = "fortress") {
        Config::new()
            .probe("ignition-transport11")
            .expect("fortress feature requires ignition-transport11")
    } else if cfg!(feature = "garden") {
        Config::new()
            .probe("gz-transport12")
            .expect("garden feature requires gz-transport12")
    } else if cfg!(feature = "harmonic") {
        Config::new()
            .probe("gz-transport13")
            .expect("harmonic feature requires gz-transport13")
    } else if cfg!(feature = "ionic") {
        Config::new()
            .probe("gz-transport14")
            .expect("ionic feature requires gz-transport14")
    } else {
        // fallback

        let enable_feature = |feature: &str| {
            println!("cargo:rustc-cfg=feature=\"{}\"", feature);
        };

        if let Ok(lib) = Config::new().probe("gz-transport14") {
            enable_feature("ionic");
            lib
        } else if let Ok(lib) = Config::new().probe("gz-transport13") {
            enable_feature("harmonic");
            lib
        } else if let Ok(lib) = Config::new().probe("gz-transport12") {
            enable_feature("garden");
            lib
        } else if let Ok(lib) = Config::new().probe("ignition-transport11") {
            enable_feature("fortress");
            lib
        } else {
            panic!("Any Gazebo transport is not found");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let library = find_library();

    for path in library.link_paths.iter() {
        println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    }

    println!("cargo:rerun-if-changed=src/wrapper.cc");
    println!("cargo:rerun-if-changed=src/wrapper.h");

    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .file("src/wrapper.cc")
        .includes(library.include_paths)
        .compile("ignition_wrapper.a");

    Ok(())
}
