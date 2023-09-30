use std::{env, fs};

use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::{Codegen, Customize, CustomizeCallback};

struct GenIgnMessage;

impl CustomizeCallback for GenIgnMessage {
    fn message(&self, _message: &MessageDescriptor) -> protobuf_codegen::Customize {
        Customize::default().before("#[derive(::gz_msgs_common::IgnMessage)]")
    }
}

struct GenGzMessage;

impl CustomizeCallback for GenGzMessage {
    fn message(&self, _message: &MessageDescriptor) -> protobuf_codegen::Customize {
        Customize::default().before("#[derive(::gz_msgs_common::GzMessage)]")
    }
}

pub fn build(is_ignition: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let mut protos = vec![];

    let dir = if is_ignition {
        "3rdparty/gz-msgs/proto/ignition/msgs"
    } else {
        "3rdparty/gz-msgs/proto/gz/msgs"
    };

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() && path.extension() == Some("proto".as_ref()) {
            let path_str = path.to_str().unwrap();
            println!("cargo:rerun-if-changed={}", path_str);
            protos.push(path);
        }
    }

    let mut codegen = Codegen::new();

    if is_ignition {
        codegen.customize_callback(GenIgnMessage)
    } else {
        codegen.customize_callback(GenGzMessage)
    };

    let ret = codegen
        .out_dir("./src/msgs")
        .inputs(&protos)
        .include("3rdparty/gz-msgs/proto")
        .run();

    if env::var("DOCS_RS").is_err() {
        ret?;
    }

    Ok(())
}
