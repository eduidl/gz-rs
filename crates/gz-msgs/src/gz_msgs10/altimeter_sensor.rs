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

//! Generated file from `gz/msgs/altimeter_sensor.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.AltimeterSensor)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AltimeterSensor {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.AltimeterSensor.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.AltimeterSensor.vertical_position_noise)
    pub vertical_position_noise: ::protobuf::MessageField<super::sensor_noise::SensorNoise>,
    // @@protoc_insertion_point(field:gz.msgs.AltimeterSensor.vertical_velocity_noise)
    pub vertical_velocity_noise: ::protobuf::MessageField<super::sensor_noise::SensorNoise>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.AltimeterSensor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AltimeterSensor {
    fn default() -> &'a AltimeterSensor {
        <AltimeterSensor as ::protobuf::Message>::default_instance()
    }
}

impl AltimeterSensor {
    pub fn new() -> AltimeterSensor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &AltimeterSensor| { &m.header },
            |m: &mut AltimeterSensor| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::sensor_noise::SensorNoise>(
            "vertical_position_noise",
            |m: &AltimeterSensor| { &m.vertical_position_noise },
            |m: &mut AltimeterSensor| { &mut m.vertical_position_noise },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::sensor_noise::SensorNoise>(
            "vertical_velocity_noise",
            |m: &AltimeterSensor| { &m.vertical_velocity_noise },
            |m: &mut AltimeterSensor| { &mut m.vertical_velocity_noise },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AltimeterSensor>(
            "AltimeterSensor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AltimeterSensor {
    const NAME: &'static str = "AltimeterSensor";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.vertical_position_noise)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.vertical_velocity_noise)?;
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
        if let Some(v) = self.vertical_position_noise.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.vertical_velocity_noise.as_ref() {
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
        if let Some(v) = self.vertical_position_noise.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.vertical_velocity_noise.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> AltimeterSensor {
        AltimeterSensor::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.vertical_position_noise.clear();
        self.vertical_velocity_noise.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AltimeterSensor {
        static instance: AltimeterSensor = AltimeterSensor {
            header: ::protobuf::MessageField::none(),
            vertical_position_noise: ::protobuf::MessageField::none(),
            vertical_velocity_noise: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AltimeterSensor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AltimeterSensor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AltimeterSensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AltimeterSensor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egz/msgs/altimeter_sensor.proto\x12\x07gz.msgs\x1a\x14gz/msgs/heade\
    r.proto\x1a\x1agz/msgs/sensor_noise.proto\"\xd6\x01\n\x0fAltimeterSensor\
    \x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\
    \x12L\n\x17vertical_position_noise\x18\x02\x20\x01(\x0b2\x14.gz.msgs.Sen\
    sorNoiseR\x15verticalPositionNoise\x12L\n\x17vertical_velocity_noise\x18\
    \x03\x20\x01(\x0b2\x14.gz.msgs.SensorNoiseR\x15verticalVelocityNoiseB$\n\
    \x0bcom.gz.msgsB\x15AltimeterSensorProtosb\x06proto3\
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
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::sensor_noise::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AltimeterSensor::generated_message_descriptor_data());
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
