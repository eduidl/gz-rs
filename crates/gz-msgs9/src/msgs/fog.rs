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

//! Generated file from `gz/msgs/fog.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Fog)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Fog {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Fog.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Fog.type)
    pub type_: ::protobuf::EnumOrUnknown<fog::FogType>,
    // @@protoc_insertion_point(field:gz.msgs.Fog.color)
    pub color: ::protobuf::MessageField<super::color::Color>,
    // @@protoc_insertion_point(field:gz.msgs.Fog.density)
    pub density: f32,
    // @@protoc_insertion_point(field:gz.msgs.Fog.start)
    pub start: f32,
    // @@protoc_insertion_point(field:gz.msgs.Fog.end)
    pub end: f32,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Fog.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Fog {
    fn default() -> &'a Fog {
        <Fog as ::protobuf::Message>::default_instance()
    }
}

impl Fog {
    pub fn new() -> Fog {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Fog| { &m.header },
            |m: &mut Fog| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Fog| { &m.type_ },
            |m: &mut Fog| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::color::Color>(
            "color",
            |m: &Fog| { &m.color },
            |m: &mut Fog| { &mut m.color },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "density",
            |m: &Fog| { &m.density },
            |m: &mut Fog| { &mut m.density },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "start",
            |m: &Fog| { &m.start },
            |m: &mut Fog| { &mut m.start },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end",
            |m: &Fog| { &m.end },
            |m: &mut Fog| { &mut m.end },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Fog>(
            "Fog",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Fog {
    const NAME: &'static str = "Fog";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                16 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.color)?;
                },
                37 => {
                    self.density = is.read_float()?;
                },
                45 => {
                    self.start = is.read_float()?;
                },
                53 => {
                    self.end = is.read_float()?;
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(fog::FogType::NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.type_.value());
        }
        if let Some(v) = self.color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.density != 0. {
            my_size += 1 + 4;
        }
        if self.start != 0. {
            my_size += 1 + 4;
        }
        if self.end != 0. {
            my_size += 1 + 4;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(fog::FogType::NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if let Some(v) = self.color.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.density != 0. {
            os.write_float(4, self.density)?;
        }
        if self.start != 0. {
            os.write_float(5, self.start)?;
        }
        if self.end != 0. {
            os.write_float(6, self.end)?;
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

    fn new() -> Fog {
        Fog::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(fog::FogType::NONE);
        self.color.clear();
        self.density = 0.;
        self.start = 0.;
        self.end = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Fog {
        static instance: Fog = Fog {
            header: ::protobuf::MessageField::none(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            color: ::protobuf::MessageField::none(),
            density: 0.,
            start: 0.,
            end: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Fog {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Fog").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Fog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Fog {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Fog`
pub mod fog {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.Fog.FogType)
    pub enum FogType {
        // @@protoc_insertion_point(enum_value:gz.msgs.Fog.FogType.NONE)
        NONE = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.Fog.FogType.LINEAR)
        LINEAR = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.Fog.FogType.EXPONENTIAL)
        EXPONENTIAL = 2,
        // @@protoc_insertion_point(enum_value:gz.msgs.Fog.FogType.EXPONENTIAL2)
        EXPONENTIAL2 = 3,
    }

    impl ::protobuf::Enum for FogType {
        const NAME: &'static str = "FogType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<FogType> {
            match value {
                0 => ::std::option::Option::Some(FogType::NONE),
                1 => ::std::option::Option::Some(FogType::LINEAR),
                2 => ::std::option::Option::Some(FogType::EXPONENTIAL),
                3 => ::std::option::Option::Some(FogType::EXPONENTIAL2),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<FogType> {
            match str {
                "NONE" => ::std::option::Option::Some(FogType::NONE),
                "LINEAR" => ::std::option::Option::Some(FogType::LINEAR),
                "EXPONENTIAL" => ::std::option::Option::Some(FogType::EXPONENTIAL),
                "EXPONENTIAL2" => ::std::option::Option::Some(FogType::EXPONENTIAL2),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [FogType] = &[
            FogType::NONE,
            FogType::LINEAR,
            FogType::EXPONENTIAL,
            FogType::EXPONENTIAL2,
        ];
    }

    impl ::protobuf::EnumFull for FogType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Fog.FogType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for FogType {
        fn default() -> Self {
            FogType::NONE
        }
    }

    impl FogType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FogType>("Fog.FogType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11gz/msgs/fog.proto\x12\x07gz.msgs\x1a\x13gz/msgs/color.proto\x1a\
    \x14gz/msgs/header.proto\"\x84\x02\n\x03Fog\x12'\n\x06header\x18\x01\x20\
    \x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12(\n\x04type\x18\x02\x20\x01(\
    \x0e2\x14.gz.msgs.Fog.FogTypeR\x04type\x12$\n\x05color\x18\x03\x20\x01(\
    \x0b2\x0e.gz.msgs.ColorR\x05color\x12\x18\n\x07density\x18\x04\x20\x01(\
    \x02R\x07density\x12\x14\n\x05start\x18\x05\x20\x01(\x02R\x05start\x12\
    \x10\n\x03end\x18\x06\x20\x01(\x02R\x03end\"B\n\x07FogType\x12\x08\n\x04\
    NONE\x10\0\x12\n\n\x06LINEAR\x10\x01\x12\x0f\n\x0bEXPONENTIAL\x10\x02\
    \x12\x10\n\x0cEXPONENTIAL2\x10\x03B\x18\n\x0bcom.gz.msgsB\tFogProtosb\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::color::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Fog::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(fog::FogType::generated_enum_descriptor_data());
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
