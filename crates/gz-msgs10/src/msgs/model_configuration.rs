// This file is generated by rust-protobuf 3.3.0. Do not edit
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

//! Generated file from `gz/msgs/model_configuration.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.ModelConfiguration)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ModelConfiguration {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.ModelConfiguration.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.ModelConfiguration.time)
    pub time: ::protobuf::MessageField<super::time::Time>,
    // @@protoc_insertion_point(field:gz.msgs.ModelConfiguration.joint_names)
    pub joint_names: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:gz.msgs.ModelConfiguration.joint_positions)
    pub joint_positions: ::std::vec::Vec<f64>,
    // @@protoc_insertion_point(field:gz.msgs.ModelConfiguration.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:gz.msgs.ModelConfiguration.link_name)
    pub link_name: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.ModelConfiguration.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ModelConfiguration {
    fn default() -> &'a ModelConfiguration {
        <ModelConfiguration as ::protobuf::Message>::default_instance()
    }
}

impl ModelConfiguration {
    pub fn new() -> ModelConfiguration {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &ModelConfiguration| { &m.header },
            |m: &mut ModelConfiguration| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::time::Time>(
            "time",
            |m: &ModelConfiguration| { &m.time },
            |m: &mut ModelConfiguration| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "joint_names",
            |m: &ModelConfiguration| { &m.joint_names },
            |m: &mut ModelConfiguration| { &mut m.joint_names },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "joint_positions",
            |m: &ModelConfiguration| { &m.joint_positions },
            |m: &mut ModelConfiguration| { &mut m.joint_positions },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &ModelConfiguration| { &m.pose },
            |m: &mut ModelConfiguration| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "link_name",
            |m: &ModelConfiguration| { &m.link_name },
            |m: &mut ModelConfiguration| { &mut m.link_name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ModelConfiguration>(
            "ModelConfiguration",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ModelConfiguration {
    const NAME: &'static str = "ModelConfiguration";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.time)?;
                },
                26 => {
                    self.joint_names.push(is.read_string()?);
                },
                34 => {
                    is.read_repeated_packed_double_into(&mut self.joint_positions)?;
                },
                33 => {
                    self.joint_positions.push(is.read_double()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                50 => {
                    self.link_name = is.read_string()?;
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
        if let Some(v) = self.time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.joint_names {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += 9 * self.joint_positions.len() as u64;
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.link_name.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.link_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.time.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.joint_names {
            os.write_string(3, &v)?;
        };
        for v in &self.joint_positions {
            os.write_double(4, *v)?;
        };
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if !self.link_name.is_empty() {
            os.write_string(6, &self.link_name)?;
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

    fn new() -> ModelConfiguration {
        ModelConfiguration::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.time.clear();
        self.joint_names.clear();
        self.joint_positions.clear();
        self.pose.clear();
        self.link_name.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ModelConfiguration {
        static instance: ModelConfiguration = ModelConfiguration {
            header: ::protobuf::MessageField::none(),
            time: ::protobuf::MessageField::none(),
            joint_names: ::std::vec::Vec::new(),
            joint_positions: ::std::vec::Vec::new(),
            pose: ::protobuf::MessageField::none(),
            link_name: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ModelConfiguration {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ModelConfiguration").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ModelConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModelConfiguration {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!gz/msgs/model_configuration.proto\x12\x07gz.msgs\x1a\x12gz/msgs/time.\
    proto\x1a\x12gz/msgs/pose.proto\x1a\x14gz/msgs/header.proto\"\xea\x01\n\
    \x12ModelConfiguration\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msg\
    s.HeaderR\x06header\x12!\n\x04time\x18\x02\x20\x01(\x0b2\r.gz.msgs.TimeR\
    \x04time\x12\x1f\n\x0bjoint_names\x18\x03\x20\x03(\tR\njointNames\x12'\n\
    \x0fjoint_positions\x18\x04\x20\x03(\x01R\x0ejointPositions\x12!\n\x04po\
    se\x18\x05\x20\x01(\x0b2\r.gz.msgs.PoseR\x04pose\x12\x1b\n\tlink_name\
    \x18\x06\x20\x01(\tR\x08linkNameB'\n\x0bcom.gz.msgsB\x18ModelConfigurati\
    onProtosb\x06proto3\
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
            deps.push(super::time::file_descriptor().clone());
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ModelConfiguration::generated_message_descriptor_data());
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
