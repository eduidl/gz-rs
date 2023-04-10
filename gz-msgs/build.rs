use std::{env, fs};

use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::{Codegen, Customize, CustomizeCallback};

struct GenGzMessage;

impl CustomizeCallback for GenGzMessage {
    fn message(&self, _message: &MessageDescriptor) -> protobuf_codegen::Customize {
        Customize::default().before("#[derive(::gz_msgs_derive::GzMessage)]")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let mut protos = vec![];

    for entry in fs::read_dir("3rdparty/gz-msgs/proto/gz/msgs")? {
        let path = entry?.path();
        if path.is_file() && path.extension() == Some("proto".as_ref()) {
            let path_str = path.to_str().unwrap();
            println!("cargo:rerun-if-changed={}", path_str);
            protos.push(path);
        }
    }

    let ret = Codegen::new()
        .out_dir("./src/msgs")
        .inputs(&protos)
        .include("3rdparty/gz-msgs/proto")
        .customize_callback(GenGzMessage)
        .run();

    if env::var("DOCS_RS").is_err() {
        ret?;
    }

    Ok(())
}
