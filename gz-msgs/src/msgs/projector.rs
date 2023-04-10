// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.12.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `gz/msgs/projector.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Projector)
pub struct Projector {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Projector.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Projector.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Projector.texture)
    pub texture: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Projector.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:gz.msgs.Projector.fov)
    pub fov: f64,
    // @@protoc_insertion_point(field:gz.msgs.Projector.near_clip)
    pub near_clip: f64,
    // @@protoc_insertion_point(field:gz.msgs.Projector.far_clip)
    pub far_clip: f64,
    // @@protoc_insertion_point(field:gz.msgs.Projector.enabled)
    pub enabled: bool,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Projector.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Projector {
    fn default() -> &'a Projector {
        <Projector as ::protobuf::Message>::default_instance()
    }
}

impl Projector {
    pub fn new() -> Projector {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Projector| { &m.header },
            |m: &mut Projector| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Projector| { &m.name },
            |m: &mut Projector| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "texture",
            |m: &Projector| { &m.texture },
            |m: &mut Projector| { &mut m.texture },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &Projector| { &m.pose },
            |m: &mut Projector| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fov",
            |m: &Projector| { &m.fov },
            |m: &mut Projector| { &mut m.fov },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "near_clip",
            |m: &Projector| { &m.near_clip },
            |m: &mut Projector| { &mut m.near_clip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "far_clip",
            |m: &Projector| { &m.far_clip },
            |m: &mut Projector| { &mut m.far_clip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "enabled",
            |m: &Projector| { &m.enabled },
            |m: &mut Projector| { &mut m.enabled },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Projector>(
            "Projector",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Projector {
    const NAME: &'static str = "Projector";

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
                26 => {
                    self.texture = is.read_string()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                41 => {
                    self.fov = is.read_double()?;
                },
                49 => {
                    self.near_clip = is.read_double()?;
                },
                57 => {
                    self.far_clip = is.read_double()?;
                },
                64 => {
                    self.enabled = is.read_bool()?;
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
        if !self.texture.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.texture);
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.fov != 0. {
            my_size += 1 + 8;
        }
        if self.near_clip != 0. {
            my_size += 1 + 8;
        }
        if self.far_clip != 0. {
            my_size += 1 + 8;
        }
        if self.enabled != false {
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.texture.is_empty() {
            os.write_string(3, &self.texture)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.fov != 0. {
            os.write_double(5, self.fov)?;
        }
        if self.near_clip != 0. {
            os.write_double(6, self.near_clip)?;
        }
        if self.far_clip != 0. {
            os.write_double(7, self.far_clip)?;
        }
        if self.enabled != false {
            os.write_bool(8, self.enabled)?;
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

    fn new() -> Projector {
        Projector::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.name.clear();
        self.texture.clear();
        self.pose.clear();
        self.fov = 0.;
        self.near_clip = 0.;
        self.far_clip = 0.;
        self.enabled = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Projector {
        static instance: Projector = Projector {
            header: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            texture: ::std::string::String::new(),
            pose: ::protobuf::MessageField::none(),
            fov: 0.,
            near_clip: 0.,
            far_clip: 0.,
            enabled: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Projector {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Projector").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Projector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Projector {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17gz/msgs/projector.proto\x12\x07gz.msgs\x1a\x12gz/msgs/pose.proto\
    \x1a\x14gz/msgs/header.proto\"\xe9\x01\n\tProjector\x12'\n\x06header\x18\
    \x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12\x12\n\x04name\x18\
    \x02\x20\x01(\tR\x04name\x12\x18\n\x07texture\x18\x03\x20\x01(\tR\x07tex\
    ture\x12!\n\x04pose\x18\x04\x20\x01(\x0b2\r.gz.msgs.PoseR\x04pose\x12\
    \x10\n\x03fov\x18\x05\x20\x01(\x01R\x03fov\x12\x1b\n\tnear_clip\x18\x06\
    \x20\x01(\x01R\x08nearClip\x12\x19\n\x08far_clip\x18\x07\x20\x01(\x01R\
    \x07farClip\x12\x18\n\x07enabled\x18\x08\x20\x01(\x08R\x07enabledB\x1e\n\
    \x0bcom.gz.msgsB\x0fProjectorProtosb\x06proto3\
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
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Projector::generated_message_descriptor_data());
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
