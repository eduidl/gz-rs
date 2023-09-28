use std::{env, error::Error};

use pkg_config::{Config, Library};

fn find_library() -> Result<Library, &'static str> {
    let candidates = ["gz-transport13", "gz-transport12", "ignition-transport11"];

    for candidate in candidates {
        if let Ok(library) = Config::new().probe(candidate) {
            return Ok(library);
        }
    }

    Err("gz-transport or ignition-transport not found")
}

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let library = find_library()?;

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
