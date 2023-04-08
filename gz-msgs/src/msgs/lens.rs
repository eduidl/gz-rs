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

//! Generated file from `gz/msgs/lens.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:gz.msgs.Lens)
pub struct Lens {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Lens.type)
    pub type_: ::protobuf::EnumOrUnknown<lens::Type>,
    // @@protoc_insertion_point(field:gz.msgs.Lens.scale_to_hfov)
    pub scale_to_hfov: bool,
    // @@protoc_insertion_point(field:gz.msgs.Lens.c1)
    pub c1: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.c2)
    pub c2: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.c3)
    pub c3: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.focal_length)
    pub focal_length: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.function_type)
    pub function_type: ::protobuf::EnumOrUnknown<lens::FunctionType>,
    // @@protoc_insertion_point(field:gz.msgs.Lens.cutoff_angle)
    pub cutoff_angle: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.environment_texture_size)
    pub environment_texture_size: i32,
    // @@protoc_insertion_point(field:gz.msgs.Lens.intrinsics_fx)
    pub intrinsics_fx: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.intrinsics_fy)
    pub intrinsics_fy: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.intrinsics_cx)
    pub intrinsics_cx: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.intrinsics_cy)
    pub intrinsics_cy: f64,
    // @@protoc_insertion_point(field:gz.msgs.Lens.intrinsics_skew)
    pub intrinsics_skew: f64,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Lens.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Lens {
    fn default() -> &'a Lens {
        <Lens as ::protobuf::Message>::default_instance()
    }
}

impl Lens {
    pub fn new() -> Lens {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(14);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Lens| { &m.type_ },
            |m: &mut Lens| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scale_to_hfov",
            |m: &Lens| { &m.scale_to_hfov },
            |m: &mut Lens| { &mut m.scale_to_hfov },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "c1",
            |m: &Lens| { &m.c1 },
            |m: &mut Lens| { &mut m.c1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "c2",
            |m: &Lens| { &m.c2 },
            |m: &mut Lens| { &mut m.c2 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "c3",
            |m: &Lens| { &m.c3 },
            |m: &mut Lens| { &mut m.c3 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "focal_length",
            |m: &Lens| { &m.focal_length },
            |m: &mut Lens| { &mut m.focal_length },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "function_type",
            |m: &Lens| { &m.function_type },
            |m: &mut Lens| { &mut m.function_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cutoff_angle",
            |m: &Lens| { &m.cutoff_angle },
            |m: &mut Lens| { &mut m.cutoff_angle },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "environment_texture_size",
            |m: &Lens| { &m.environment_texture_size },
            |m: &mut Lens| { &mut m.environment_texture_size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "intrinsics_fx",
            |m: &Lens| { &m.intrinsics_fx },
            |m: &mut Lens| { &mut m.intrinsics_fx },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "intrinsics_fy",
            |m: &Lens| { &m.intrinsics_fy },
            |m: &mut Lens| { &mut m.intrinsics_fy },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "intrinsics_cx",
            |m: &Lens| { &m.intrinsics_cx },
            |m: &mut Lens| { &mut m.intrinsics_cx },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "intrinsics_cy",
            |m: &Lens| { &m.intrinsics_cy },
            |m: &mut Lens| { &mut m.intrinsics_cy },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "intrinsics_skew",
            |m: &Lens| { &m.intrinsics_skew },
            |m: &mut Lens| { &mut m.intrinsics_skew },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Lens>(
            "Lens",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Lens {
    const NAME: &'static str = "Lens";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.scale_to_hfov = is.read_bool()?;
                },
                25 => {
                    self.c1 = is.read_double()?;
                },
                33 => {
                    self.c2 = is.read_double()?;
                },
                41 => {
                    self.c3 = is.read_double()?;
                },
                49 => {
                    self.focal_length = is.read_double()?;
                },
                56 => {
                    self.function_type = is.read_enum_or_unknown()?;
                },
                65 => {
                    self.cutoff_angle = is.read_double()?;
                },
                72 => {
                    self.environment_texture_size = is.read_int32()?;
                },
                81 => {
                    self.intrinsics_fx = is.read_double()?;
                },
                89 => {
                    self.intrinsics_fy = is.read_double()?;
                },
                97 => {
                    self.intrinsics_cx = is.read_double()?;
                },
                105 => {
                    self.intrinsics_cy = is.read_double()?;
                },
                113 => {
                    self.intrinsics_skew = is.read_double()?;
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(lens::Type::TYPE_NOT_SPECIFIED) {
            my_size += ::protobuf::rt::int32_size(1, self.type_.value());
        }
        if self.scale_to_hfov != false {
            my_size += 1 + 1;
        }
        if self.c1 != 0. {
            my_size += 1 + 8;
        }
        if self.c2 != 0. {
            my_size += 1 + 8;
        }
        if self.c3 != 0. {
            my_size += 1 + 8;
        }
        if self.focal_length != 0. {
            my_size += 1 + 8;
        }
        if self.function_type != ::protobuf::EnumOrUnknown::new(lens::FunctionType::FUNCTION_NOT_SPECIFIED) {
            my_size += ::protobuf::rt::int32_size(7, self.function_type.value());
        }
        if self.cutoff_angle != 0. {
            my_size += 1 + 8;
        }
        if self.environment_texture_size != 0 {
            my_size += ::protobuf::rt::int32_size(9, self.environment_texture_size);
        }
        if self.intrinsics_fx != 0. {
            my_size += 1 + 8;
        }
        if self.intrinsics_fy != 0. {
            my_size += 1 + 8;
        }
        if self.intrinsics_cx != 0. {
            my_size += 1 + 8;
        }
        if self.intrinsics_cy != 0. {
            my_size += 1 + 8;
        }
        if self.intrinsics_skew != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.type_ != ::protobuf::EnumOrUnknown::new(lens::Type::TYPE_NOT_SPECIFIED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if self.scale_to_hfov != false {
            os.write_bool(2, self.scale_to_hfov)?;
        }
        if self.c1 != 0. {
            os.write_double(3, self.c1)?;
        }
        if self.c2 != 0. {
            os.write_double(4, self.c2)?;
        }
        if self.c3 != 0. {
            os.write_double(5, self.c3)?;
        }
        if self.focal_length != 0. {
            os.write_double(6, self.focal_length)?;
        }
        if self.function_type != ::protobuf::EnumOrUnknown::new(lens::FunctionType::FUNCTION_NOT_SPECIFIED) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.function_type))?;
        }
        if self.cutoff_angle != 0. {
            os.write_double(8, self.cutoff_angle)?;
        }
        if self.environment_texture_size != 0 {
            os.write_int32(9, self.environment_texture_size)?;
        }
        if self.intrinsics_fx != 0. {
            os.write_double(10, self.intrinsics_fx)?;
        }
        if self.intrinsics_fy != 0. {
            os.write_double(11, self.intrinsics_fy)?;
        }
        if self.intrinsics_cx != 0. {
            os.write_double(12, self.intrinsics_cx)?;
        }
        if self.intrinsics_cy != 0. {
            os.write_double(13, self.intrinsics_cy)?;
        }
        if self.intrinsics_skew != 0. {
            os.write_double(14, self.intrinsics_skew)?;
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

    fn new() -> Lens {
        Lens::new()
    }

    fn clear(&mut self) {
        self.type_ = ::protobuf::EnumOrUnknown::new(lens::Type::TYPE_NOT_SPECIFIED);
        self.scale_to_hfov = false;
        self.c1 = 0.;
        self.c2 = 0.;
        self.c3 = 0.;
        self.focal_length = 0.;
        self.function_type = ::protobuf::EnumOrUnknown::new(lens::FunctionType::FUNCTION_NOT_SPECIFIED);
        self.cutoff_angle = 0.;
        self.environment_texture_size = 0;
        self.intrinsics_fx = 0.;
        self.intrinsics_fy = 0.;
        self.intrinsics_cx = 0.;
        self.intrinsics_cy = 0.;
        self.intrinsics_skew = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Lens {
        static instance: Lens = Lens {
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            scale_to_hfov: false,
            c1: 0.,
            c2: 0.,
            c3: 0.,
            focal_length: 0.,
            function_type: ::protobuf::EnumOrUnknown::from_i32(0),
            cutoff_angle: 0.,
            environment_texture_size: 0,
            intrinsics_fx: 0.,
            intrinsics_fy: 0.,
            intrinsics_cx: 0.,
            intrinsics_cy: 0.,
            intrinsics_skew: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Lens {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Lens").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Lens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Lens {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Lens`
pub mod lens {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.Lens.Type)
    pub enum Type {
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.Type.TYPE_NOT_SPECIFIED)
        TYPE_NOT_SPECIFIED = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.Type.GNOMONICAL)
        GNOMONICAL = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.Type.STEREOGRAPHIC)
        STEREOGRAPHIC = 2,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.Type.EQUIDISTANT)
        EQUIDISTANT = 3,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.Type.EQUISOLID_ANGLE)
        EQUISOLID_ANGLE = 4,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.Type.ORTHOGRAPHIC)
        ORTHOGRAPHIC = 5,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.Type.CUSTOM)
        CUSTOM = 6,
    }

    impl ::protobuf::Enum for Type {
        const NAME: &'static str = "Type";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Type> {
            match value {
                0 => ::std::option::Option::Some(Type::TYPE_NOT_SPECIFIED),
                1 => ::std::option::Option::Some(Type::GNOMONICAL),
                2 => ::std::option::Option::Some(Type::STEREOGRAPHIC),
                3 => ::std::option::Option::Some(Type::EQUIDISTANT),
                4 => ::std::option::Option::Some(Type::EQUISOLID_ANGLE),
                5 => ::std::option::Option::Some(Type::ORTHOGRAPHIC),
                6 => ::std::option::Option::Some(Type::CUSTOM),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Type] = &[
            Type::TYPE_NOT_SPECIFIED,
            Type::GNOMONICAL,
            Type::STEREOGRAPHIC,
            Type::EQUIDISTANT,
            Type::EQUISOLID_ANGLE,
            Type::ORTHOGRAPHIC,
            Type::CUSTOM,
        ];
    }

    impl ::protobuf::EnumFull for Type {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Lens.Type").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Type {
        fn default() -> Self {
            Type::TYPE_NOT_SPECIFIED
        }
    }

    impl Type {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Type>("Lens.Type")
        }
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.Lens.FunctionType)
    pub enum FunctionType {
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.FunctionType.FUNCTION_NOT_SPECIFIED)
        FUNCTION_NOT_SPECIFIED = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.FunctionType.SIN)
        SIN = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.FunctionType.TAN)
        TAN = 2,
        // @@protoc_insertion_point(enum_value:gz.msgs.Lens.FunctionType.ID)
        ID = 3,
    }

    impl ::protobuf::Enum for FunctionType {
        const NAME: &'static str = "FunctionType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<FunctionType> {
            match value {
                0 => ::std::option::Option::Some(FunctionType::FUNCTION_NOT_SPECIFIED),
                1 => ::std::option::Option::Some(FunctionType::SIN),
                2 => ::std::option::Option::Some(FunctionType::TAN),
                3 => ::std::option::Option::Some(FunctionType::ID),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [FunctionType] = &[
            FunctionType::FUNCTION_NOT_SPECIFIED,
            FunctionType::SIN,
            FunctionType::TAN,
            FunctionType::ID,
        ];
    }

    impl ::protobuf::EnumFull for FunctionType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Lens.FunctionType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for FunctionType {
        fn default() -> Self {
            FunctionType::FUNCTION_NOT_SPECIFIED
        }
    }

    impl FunctionType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FunctionType>("Lens.FunctionType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12gz/msgs/lens.proto\x12\x07gz.msgs\"\xce\x05\n\x04Lens\x12&\n\x04ty\
    pe\x18\x01\x20\x01(\x0e2\x12.gz.msgs.Lens.TypeR\x04type\x12\"\n\rscale_t\
    o_hfov\x18\x02\x20\x01(\x08R\x0bscaleToHfov\x12\x0e\n\x02c1\x18\x03\x20\
    \x01(\x01R\x02c1\x12\x0e\n\x02c2\x18\x04\x20\x01(\x01R\x02c2\x12\x0e\n\
    \x02c3\x18\x05\x20\x01(\x01R\x02c3\x12!\n\x0cfocal_length\x18\x06\x20\
    \x01(\x01R\x0bfocalLength\x12?\n\rfunction_type\x18\x07\x20\x01(\x0e2\
    \x1a.gz.msgs.Lens.FunctionTypeR\x0cfunctionType\x12!\n\x0ccutoff_angle\
    \x18\x08\x20\x01(\x01R\x0bcutoffAngle\x128\n\x18environment_texture_size\
    \x18\t\x20\x01(\x05R\x16environmentTextureSize\x12#\n\rintrinsics_fx\x18\
    \n\x20\x01(\x01R\x0cintrinsicsFx\x12#\n\rintrinsics_fy\x18\x0b\x20\x01(\
    \x01R\x0cintrinsicsFy\x12#\n\rintrinsics_cx\x18\x0c\x20\x01(\x01R\x0cint\
    rinsicsCx\x12#\n\rintrinsics_cy\x18\r\x20\x01(\x01R\x0cintrinsicsCy\x12'\
    \n\x0fintrinsics_skew\x18\x0e\x20\x01(\x01R\x0eintrinsicsSkew\"\x85\x01\
    \n\x04Type\x12\x16\n\x12TYPE_NOT_SPECIFIED\x10\0\x12\x0e\n\nGNOMONICAL\
    \x10\x01\x12\x11\n\rSTEREOGRAPHIC\x10\x02\x12\x0f\n\x0bEQUIDISTANT\x10\
    \x03\x12\x13\n\x0fEQUISOLID_ANGLE\x10\x04\x12\x10\n\x0cORTHOGRAPHIC\x10\
    \x05\x12\n\n\x06CUSTOM\x10\x06\"D\n\x0cFunctionType\x12\x1a\n\x16FUNCTIO\
    N_NOT_SPECIFIED\x10\0\x12\x07\n\x03SIN\x10\x01\x12\x07\n\x03TAN\x10\x02\
    \x12\x06\n\x02ID\x10\x03B\x19\n\x0bcom.gz.msgsB\nLensProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Lens::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(2);
            enums.push(lens::Type::generated_enum_descriptor_data());
            enums.push(lens::FunctionType::generated_enum_descriptor_data());
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