use std::error::Error;

use pkg_config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let library = Config::new().probe("gz-transport12")?;

    for path in library.link_paths.iter() {
        println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    }

    println!("cargo:rerun-if-changed=src/wrapper.cc");
    println!("cargo:rerun-if-changed=src/wrapper.h");

    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cc")
        .includes(library.include_paths)
        .compile("ignition_wrapper.a");

    Ok(())
}
