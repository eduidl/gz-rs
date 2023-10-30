#![doc = include_str!("../README.md")]

use std::{fs, path::PathBuf};

use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::{Codegen, Customize, CustomizeCallback};

struct GenIgnMessage;

impl CustomizeCallback for GenIgnMessage {
    fn message(&self, _message: &MessageDescriptor) -> Customize {
        Customize::default().before("#[derive(::gz_msgs_common::IgnMessage)]")
    }
}

struct GenGzMessage;

impl CustomizeCallback for GenGzMessage {
    fn message(&self, _message: &MessageDescriptor) -> Customize {
        Customize::default().before("#[derive(::gz_msgs_common::GzMessage)]")
    }
}

pub fn build(branch: &str, is_ignition: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut protos = vec![];

    let proto_dir = PathBuf::new().join("3rdparty").join(branch).join("proto");
    let dir = proto_dir
        .join(if is_ignition { "ignition" } else { "gz" })
        .join("msgs");

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

    let out_dir = PathBuf::new().join("src").join(branch.replace('-', "_"));
    let _ = fs::create_dir(&out_dir);

    codegen
        .out_dir(&out_dir)
        .inputs(&protos)
        .include(&proto_dir)
        .run()?;

    Ok(())
}
