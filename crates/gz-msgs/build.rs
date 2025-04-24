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

#[cfg(not(feature = "generate"))]
fn main() {
    use std::env;

    use pkg_config::Config;

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    if cfg!(feature = "fortress") {
        Config::new()
            .probe("ignition-transport11")
            .expect("fortress feature requires ignition-transport11");
    } else if cfg!(feature = "garden") {
        Config::new()
            .probe("gz-transport12")
            .expect("garden feature requires gz-transport12");
    } else if cfg!(feature = "harmonic") {
        Config::new()
            .probe("gz-transport13")
            .expect("harmonic feature requires gz-transport13");
    } else if cfg!(feature = "ionic") {
        Config::new()
            .probe("gz-transport14")
            .expect("ionic feature requires gz-transport14");
    } else {
        // fallback

        let enable_feature = |feature: &str| {
            println!("cargo:rustc-cfg=feature=\"{}\"", feature);
        };

        if Config::new().probe("gz-transport14").is_ok() {
            enable_feature("ionic");
        } else if Config::new().probe("gz-transport13").is_ok() {
            enable_feature("harmonic");
        } else if Config::new().probe("gz-transport12").is_ok() {
            enable_feature("garden");
        } else if Config::new().probe("ignition-transport11").is_ok() {
            enable_feature("fortress");
        } else {
            panic!("Any Gazebo transport is not found");
        }
    }
}

#[cfg(feature = "generate")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use gz_msgs_build::build;

    build("ign-msgs8", true)?;
    build("gz-msgs9", false)?;
    build("gz-msgs10", false)?;
    build("gz-msgs11", false)?;

    Ok(())
}
