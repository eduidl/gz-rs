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

//! Generated file from `ignition/msgs/logical_camera_image.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.LogicalCameraImage)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LogicalCameraImage {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraImage.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraImage.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraImage.model)
    pub model: ::std::vec::Vec<logical_camera_image::Model>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.LogicalCameraImage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LogicalCameraImage {
    fn default() -> &'a LogicalCameraImage {
        <LogicalCameraImage as ::protobuf::Message>::default_instance()
    }
}

impl LogicalCameraImage {
    pub fn new() -> LogicalCameraImage {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &LogicalCameraImage| { &m.header },
            |m: &mut LogicalCameraImage| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &LogicalCameraImage| { &m.pose },
            |m: &mut LogicalCameraImage| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "model",
            |m: &LogicalCameraImage| { &m.model },
            |m: &mut LogicalCameraImage| { &mut m.model },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LogicalCameraImage>(
            "LogicalCameraImage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LogicalCameraImage {
    const NAME: &'static str = "LogicalCameraImage";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                26 => {
                    self.model.push(is.read_message()?);
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
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.model {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.model {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> LogicalCameraImage {
        LogicalCameraImage::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.pose.clear();
        self.model.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LogicalCameraImage {
        static instance: LogicalCameraImage = LogicalCameraImage {
            header: ::protobuf::MessageField::none(),
            pose: ::protobuf::MessageField::none(),
            model: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LogicalCameraImage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LogicalCameraImage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LogicalCameraImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogicalCameraImage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LogicalCameraImage`
pub mod logical_camera_image {
    #[derive(::gz_msgs_common::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.LogicalCameraImage.Model)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Model {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraImage.Model.name)
        pub name: ::std::string::String,
        // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraImage.Model.pose)
        pub pose: ::protobuf::MessageField<super::super::pose::Pose>,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.LogicalCameraImage.Model.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Model {
        fn default() -> &'a Model {
            <Model as ::protobuf::Message>::default_instance()
        }
    }

    impl Model {
        pub fn new() -> Model {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "name",
                |m: &Model| { &m.name },
                |m: &mut Model| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::pose::Pose>(
                "pose",
                |m: &Model| { &m.pose },
                |m: &mut Model| { &mut m.pose },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Model>(
                "LogicalCameraImage.Model",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Model {
        const NAME: &'static str = "Model";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.name = is.read_string()?;
                    },
                    18 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
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
            if !self.name.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.name);
            }
            if let Some(v) = self.pose.as_ref() {
                let len = v.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.name.is_empty() {
                os.write_string(1, &self.name)?;
            }
            if let Some(v) = self.pose.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

        fn new() -> Model {
            Model::new()
        }

        fn clear(&mut self) {
            self.name.clear();
            self.pose.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Model {
            static instance: Model = Model {
                name: ::std::string::String::new(),
                pose: ::protobuf::MessageField::none(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Model {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("LogicalCameraImage.Model").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Model {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Model {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(ignition/msgs/logical_camera_image.proto\x12\rignition.msgs\x1a\x18ig\
    nition/msgs/pose.proto\x1a\x1aignition/msgs/header.proto\"\xf1\x01\n\x12\
    LogicalCameraImage\x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15.ignition.m\
    sgs.HeaderR\x06header\x12'\n\x04pose\x18\x02\x20\x01(\x0b2\x13.ignition.\
    msgs.PoseR\x04pose\x12=\n\x05model\x18\x03\x20\x03(\x0b2'.ignition.msgs.\
    LogicalCameraImage.ModelR\x05model\x1aD\n\x05Model\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12'\n\x04pose\x18\x02\x20\x01(\x0b2\x13.igniti\
    on.msgs.PoseR\x04poseB-\n\x11com.ignition.msgsB\x18LogicalCameraImagePro\
    tosb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(LogicalCameraImage::generated_message_descriptor_data());
            messages.push(logical_camera_image::Model::generated_message_descriptor_data());
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
