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

//! Generated file from `ignition/msgs/road.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Road)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Road {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Road.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.Road.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Road.width)
    pub width: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Road.point)
    pub point: ::std::vec::Vec<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:ignition.msgs.Road.material)
    pub material: ::protobuf::MessageField<super::material::Material>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Road.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Road {
    fn default() -> &'a Road {
        <Road as ::protobuf::Message>::default_instance()
    }
}

impl Road {
    pub fn new() -> Road {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Road| { &m.header },
            |m: &mut Road| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Road| { &m.name },
            |m: &mut Road| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "width",
            |m: &Road| { &m.width },
            |m: &mut Road| { &mut m.width },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "point",
            |m: &Road| { &m.point },
            |m: &mut Road| { &mut m.point },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::material::Material>(
            "material",
            |m: &Road| { &m.material },
            |m: &mut Road| { &mut m.material },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Road>(
            "Road",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Road {
    const NAME: &'static str = "Road";

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
                    self.name = is.read_string()?;
                },
                25 => {
                    self.width = is.read_double()?;
                },
                34 => {
                    self.point.push(is.read_message()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.material)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.width != 0. {
            my_size += 1 + 8;
        }
        for value in &self.point {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.material.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.width != 0. {
            os.write_double(3, self.width)?;
        }
        for v in &self.point {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if let Some(v) = self.material.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> Road {
        Road::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.name.clear();
        self.width = 0.;
        self.point.clear();
        self.material.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Road {
        static instance: Road = Road {
            header: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            width: 0.,
            point: ::std::vec::Vec::new(),
            material: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Road {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Road").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Road {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Road {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18ignition/msgs/road.proto\x12\rignition.msgs\x1a\x1cignition/msgs/v\
    ector3d.proto\x1a\x1cignition/msgs/material.proto\x1a\x1aignition/msgs/h\
    eader.proto\"\xc3\x01\n\x04Road\x12-\n\x06header\x18\x01\x20\x01(\x0b2\
    \x15.ignition.msgs.HeaderR\x06header\x12\x12\n\x04name\x18\x02\x20\x01(\
    \tR\x04name\x12\x14\n\x05width\x18\x03\x20\x01(\x01R\x05width\x12-\n\x05\
    point\x18\x04\x20\x03(\x0b2\x17.ignition.msgs.Vector3dR\x05point\x123\n\
    \x08material\x18\x05\x20\x01(\x0b2\x17.ignition.msgs.MaterialR\x08materi\
    alB\x1f\n\x11com.ignition.msgsB\nRoadProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::vector3d::file_descriptor().clone());
            deps.push(super::material::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Road::generated_message_descriptor_data());
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
