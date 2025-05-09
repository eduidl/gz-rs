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

//! Generated file from `gz/msgs/dvl_kinematic_estimate.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.DVLKinematicEstimate)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DVLKinematicEstimate {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.DVLKinematicEstimate.reference)
    pub reference: ::protobuf::EnumOrUnknown<dvlkinematic_estimate::ReferenceType>,
    // @@protoc_insertion_point(field:gz.msgs.DVLKinematicEstimate.mean)
    pub mean: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:gz.msgs.DVLKinematicEstimate.covariance)
    pub covariance: ::std::vec::Vec<f64>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.DVLKinematicEstimate.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DVLKinematicEstimate {
    fn default() -> &'a DVLKinematicEstimate {
        <DVLKinematicEstimate as ::protobuf::Message>::default_instance()
    }
}

impl DVLKinematicEstimate {
    pub fn new() -> DVLKinematicEstimate {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reference",
            |m: &DVLKinematicEstimate| { &m.reference },
            |m: &mut DVLKinematicEstimate| { &mut m.reference },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "mean",
            |m: &DVLKinematicEstimate| { &m.mean },
            |m: &mut DVLKinematicEstimate| { &mut m.mean },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "covariance",
            |m: &DVLKinematicEstimate| { &m.covariance },
            |m: &mut DVLKinematicEstimate| { &mut m.covariance },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DVLKinematicEstimate>(
            "DVLKinematicEstimate",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DVLKinematicEstimate {
    const NAME: &'static str = "DVLKinematicEstimate";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.reference = is.read_enum_or_unknown()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.mean)?;
                },
                26 => {
                    is.read_repeated_packed_double_into(&mut self.covariance)?;
                },
                25 => {
                    self.covariance.push(is.read_double()?);
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
        if self.reference != ::protobuf::EnumOrUnknown::new(dvlkinematic_estimate::ReferenceType::DVL_REFERENCE_UNSPECIFIED) {
            my_size += ::protobuf::rt::int32_size(1, self.reference.value());
        }
        if let Some(v) = self.mean.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_double_size(3, &self.covariance);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.reference != ::protobuf::EnumOrUnknown::new(dvlkinematic_estimate::ReferenceType::DVL_REFERENCE_UNSPECIFIED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.reference))?;
        }
        if let Some(v) = self.mean.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_repeated_packed_double(3, &self.covariance)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DVLKinematicEstimate {
        DVLKinematicEstimate::new()
    }

    fn clear(&mut self) {
        self.reference = ::protobuf::EnumOrUnknown::new(dvlkinematic_estimate::ReferenceType::DVL_REFERENCE_UNSPECIFIED);
        self.mean.clear();
        self.covariance.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DVLKinematicEstimate {
        static instance: DVLKinematicEstimate = DVLKinematicEstimate {
            reference: ::protobuf::EnumOrUnknown::from_i32(0),
            mean: ::protobuf::MessageField::none(),
            covariance: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DVLKinematicEstimate {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DVLKinematicEstimate").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DVLKinematicEstimate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DVLKinematicEstimate {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `DVLKinematicEstimate`
pub mod dvlkinematic_estimate {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.DVLKinematicEstimate.ReferenceType)
    pub enum ReferenceType {
        // @@protoc_insertion_point(enum_value:gz.msgs.DVLKinematicEstimate.ReferenceType.DVL_REFERENCE_UNSPECIFIED)
        DVL_REFERENCE_UNSPECIFIED = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.DVLKinematicEstimate.ReferenceType.DVL_REFERENCE_EARTH)
        DVL_REFERENCE_EARTH = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.DVLKinematicEstimate.ReferenceType.DVL_REFERENCE_SHIP)
        DVL_REFERENCE_SHIP = 2,
    }

    impl ::protobuf::Enum for ReferenceType {
        const NAME: &'static str = "ReferenceType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<ReferenceType> {
            match value {
                0 => ::std::option::Option::Some(ReferenceType::DVL_REFERENCE_UNSPECIFIED),
                1 => ::std::option::Option::Some(ReferenceType::DVL_REFERENCE_EARTH),
                2 => ::std::option::Option::Some(ReferenceType::DVL_REFERENCE_SHIP),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<ReferenceType> {
            match str {
                "DVL_REFERENCE_UNSPECIFIED" => ::std::option::Option::Some(ReferenceType::DVL_REFERENCE_UNSPECIFIED),
                "DVL_REFERENCE_EARTH" => ::std::option::Option::Some(ReferenceType::DVL_REFERENCE_EARTH),
                "DVL_REFERENCE_SHIP" => ::std::option::Option::Some(ReferenceType::DVL_REFERENCE_SHIP),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [ReferenceType] = &[
            ReferenceType::DVL_REFERENCE_UNSPECIFIED,
            ReferenceType::DVL_REFERENCE_EARTH,
            ReferenceType::DVL_REFERENCE_SHIP,
        ];
    }

    impl ::protobuf::EnumFull for ReferenceType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("DVLKinematicEstimate.ReferenceType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for ReferenceType {
        fn default() -> Self {
            ReferenceType::DVL_REFERENCE_UNSPECIFIED
        }
    }

    impl ReferenceType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ReferenceType>("DVLKinematicEstimate.ReferenceType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$gz/msgs/dvl_kinematic_estimate.proto\x12\x07gz.msgs\x1a\x16gz/msgs/ve\
    ctor3d.proto\"\x89\x02\n\x14DVLKinematicEstimate\x12I\n\treference\x18\
    \x01\x20\x01(\x0e2+.gz.msgs.DVLKinematicEstimate.ReferenceTypeR\treferen\
    ce\x12%\n\x04mean\x18\x02\x20\x01(\x0b2\x11.gz.msgs.Vector3dR\x04mean\
    \x12\x1e\n\ncovariance\x18\x03\x20\x03(\x01R\ncovariance\"_\n\rReference\
    Type\x12\x1d\n\x19DVL_REFERENCE_UNSPECIFIED\x10\0\x12\x17\n\x13DVL_REFER\
    ENCE_EARTH\x10\x01\x12\x16\n\x12DVL_REFERENCE_SHIP\x10\x02B)\n\x0bcom.gz\
    .msgsB\x1aDVLKinematicEstimateProtosb\x06proto3\
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
            deps.push(super::vector3d::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DVLKinematicEstimate::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(dvlkinematic_estimate::ReferenceType::generated_enum_descriptor_data());
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
