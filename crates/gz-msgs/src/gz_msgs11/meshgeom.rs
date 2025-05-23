// This file is generated by rust-protobuf 3.7.2. Do not edit
// .proto file is parsed by protoc 3.21.12
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `gz/msgs/meshgeom.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.MeshGeom)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MeshGeom {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.MeshGeom.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.MeshGeom.filename)
    pub filename: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.MeshGeom.scale)
    pub scale: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:gz.msgs.MeshGeom.submesh)
    pub submesh: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.MeshGeom.center_submesh)
    pub center_submesh: bool,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.MeshGeom.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MeshGeom {
    fn default() -> &'a MeshGeom {
        <MeshGeom as ::protobuf::Message>::default_instance()
    }
}

impl MeshGeom {
    pub fn new() -> MeshGeom {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &MeshGeom| { &m.header },
            |m: &mut MeshGeom| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "filename",
            |m: &MeshGeom| { &m.filename },
            |m: &mut MeshGeom| { &mut m.filename },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "scale",
            |m: &MeshGeom| { &m.scale },
            |m: &mut MeshGeom| { &mut m.scale },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "submesh",
            |m: &MeshGeom| { &m.submesh },
            |m: &mut MeshGeom| { &mut m.submesh },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "center_submesh",
            |m: &MeshGeom| { &m.center_submesh },
            |m: &mut MeshGeom| { &mut m.center_submesh },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MeshGeom>(
            "MeshGeom",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MeshGeom {
    const NAME: &'static str = "MeshGeom";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                18 => {
                    self.filename = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.scale)?;
                },
                34 => {
                    self.submesh = is.read_string()?;
                },
                40 => {
                    self.center_submesh = is.read_bool()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.filename.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.filename);
        }
        if let Some(v) = self.scale.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.submesh.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.submesh);
        }
        if self.center_submesh != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.filename.is_empty() {
            os.write_string(2, &self.filename)?;
        }
        if let Some(v) = self.scale.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if !self.submesh.is_empty() {
            os.write_string(4, &self.submesh)?;
        }
        if self.center_submesh != false {
            os.write_bool(5, self.center_submesh)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MeshGeom {
        MeshGeom::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.filename.clear();
        self.scale.clear();
        self.submesh.clear();
        self.center_submesh = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MeshGeom {
        static instance: MeshGeom = MeshGeom {
            header: ::protobuf::MessageField::none(),
            filename: ::std::string::String::new(),
            scale: ::protobuf::MessageField::none(),
            submesh: ::std::string::String::new(),
            center_submesh: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MeshGeom {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MeshGeom").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MeshGeom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MeshGeom {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16gz/msgs/meshgeom.proto\x12\x07gz.msgs\x1a\x16gz/msgs/vector3d.prot\
    o\x1a\x14gz/msgs/header.proto\"\xb9\x01\n\x08MeshGeom\x12'\n\x06header\
    \x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12\x1a\n\x08filena\
    me\x18\x02\x20\x01(\tR\x08filename\x12'\n\x05scale\x18\x03\x20\x01(\x0b2\
    \x11.gz.msgs.Vector3dR\x05scale\x12\x18\n\x07submesh\x18\x04\x20\x01(\tR\
    \x07submesh\x12%\n\x0ecenter_submesh\x18\x05\x20\x01(\x08R\rcenterSubmes\
    hB\x1d\n\x0bcom.gz.msgsB\x0eMeshGeomProtosb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::vector3d::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MeshGeom::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
