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

//! Generated file from `ignition/msgs/logical_camera_sensor.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.LogicalCameraSensor)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LogicalCameraSensor {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraSensor.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraSensor.near_clip)
    pub near_clip: f64,
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraSensor.far_clip)
    pub far_clip: f64,
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraSensor.horizontal_fov)
    pub horizontal_fov: f64,
    // @@protoc_insertion_point(field:ignition.msgs.LogicalCameraSensor.aspect_ratio)
    pub aspect_ratio: f64,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.LogicalCameraSensor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LogicalCameraSensor {
    fn default() -> &'a LogicalCameraSensor {
        <LogicalCameraSensor as ::protobuf::Message>::default_instance()
    }
}

impl LogicalCameraSensor {
    pub fn new() -> LogicalCameraSensor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &LogicalCameraSensor| { &m.header },
            |m: &mut LogicalCameraSensor| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "near_clip",
            |m: &LogicalCameraSensor| { &m.near_clip },
            |m: &mut LogicalCameraSensor| { &mut m.near_clip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "far_clip",
            |m: &LogicalCameraSensor| { &m.far_clip },
            |m: &mut LogicalCameraSensor| { &mut m.far_clip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "horizontal_fov",
            |m: &LogicalCameraSensor| { &m.horizontal_fov },
            |m: &mut LogicalCameraSensor| { &mut m.horizontal_fov },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "aspect_ratio",
            |m: &LogicalCameraSensor| { &m.aspect_ratio },
            |m: &mut LogicalCameraSensor| { &mut m.aspect_ratio },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LogicalCameraSensor>(
            "LogicalCameraSensor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LogicalCameraSensor {
    const NAME: &'static str = "LogicalCameraSensor";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                17 => {
                    self.near_clip = is.read_double()?;
                },
                25 => {
                    self.far_clip = is.read_double()?;
                },
                33 => {
                    self.horizontal_fov = is.read_double()?;
                },
                41 => {
                    self.aspect_ratio = is.read_double()?;
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
        if self.near_clip != 0. {
            my_size += 1 + 8;
        }
        if self.far_clip != 0. {
            my_size += 1 + 8;
        }
        if self.horizontal_fov != 0. {
            my_size += 1 + 8;
        }
        if self.aspect_ratio != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.near_clip != 0. {
            os.write_double(2, self.near_clip)?;
        }
        if self.far_clip != 0. {
            os.write_double(3, self.far_clip)?;
        }
        if self.horizontal_fov != 0. {
            os.write_double(4, self.horizontal_fov)?;
        }
        if self.aspect_ratio != 0. {
            os.write_double(5, self.aspect_ratio)?;
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

    fn new() -> LogicalCameraSensor {
        LogicalCameraSensor::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.near_clip = 0.;
        self.far_clip = 0.;
        self.horizontal_fov = 0.;
        self.aspect_ratio = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LogicalCameraSensor {
        static instance: LogicalCameraSensor = LogicalCameraSensor {
            header: ::protobuf::MessageField::none(),
            near_clip: 0.,
            far_clip: 0.,
            horizontal_fov: 0.,
            aspect_ratio: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LogicalCameraSensor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LogicalCameraSensor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LogicalCameraSensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogicalCameraSensor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)ignition/msgs/logical_camera_sensor.proto\x12\rignition.msgs\x1a\x1ai\
    gnition/msgs/header.proto\"\xc6\x01\n\x13LogicalCameraSensor\x12-\n\x06h\
    eader\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x12\x1b\
    \n\tnear_clip\x18\x02\x20\x01(\x01R\x08nearClip\x12\x19\n\x08far_clip\
    \x18\x03\x20\x01(\x01R\x07farClip\x12%\n\x0ehorizontal_fov\x18\x04\x20\
    \x01(\x01R\rhorizontalFov\x12!\n\x0caspect_ratio\x18\x05\x20\x01(\x01R\
    \x0baspectRatioB.\n\x11com.ignition.msgsB\x19LogicalCameraSensorProtosb\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LogicalCameraSensor::generated_message_descriptor_data());
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
