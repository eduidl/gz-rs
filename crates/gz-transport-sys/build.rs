use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    gz_build::check_exclusive_version_features();
    if env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let library = gz_build::find_transport_library();

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
