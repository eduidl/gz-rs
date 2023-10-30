use pkg_config::Config;

#[cfg(all(feature = "fortress", feature = "garden"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");
#[cfg(all(feature = "garden", feature = "harmonic"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");
#[cfg(all(feature = "fortress", feature = "harmonic"))]
compile_error!("Only one of the following features can be enabled: fortress, garden, harmonic");

#[cfg(not(feature = "generate"))]
fn main() {
    if cfg!(docsrs) {
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
    } else {
        // fallback

        let enable_feature = |feature: &str| {
            println!("cargo:rustc-cfg=feature=\"{}\"", feature);
        };

        if Config::new().probe("gz-transport13").is_ok() {
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

    Ok(())
}
