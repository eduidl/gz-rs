#[cfg(not(feature = "generate"))]
fn main() {
    gz_build::check_exclusive_version_features();
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    gz_build::find_transport_library();
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
