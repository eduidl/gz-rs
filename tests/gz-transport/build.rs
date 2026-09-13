fn main() {
    gz_build::check_exclusive_version_features();
    let library = gz_build::find_transport_library();
    println!("cargo:rerun-if-changed=src/service.cc");
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .file("src/service.cc")
        .includes(library.include_paths)
        .compile("gz_transport_test_service");
}
