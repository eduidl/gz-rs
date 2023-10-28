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

//! Generated file from `gz/msgs/spherical_coordinates.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.SphericalCoordinates)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SphericalCoordinates {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.surface_model)
    pub surface_model: ::protobuf::EnumOrUnknown<spherical_coordinates::SurfaceModel>,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.latitude_deg)
    pub latitude_deg: f64,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.longitude_deg)
    pub longitude_deg: f64,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.elevation)
    pub elevation: f64,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.heading_deg)
    pub heading_deg: f64,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.entity)
    pub entity: ::protobuf::MessageField<super::entity::Entity>,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.surface_axis_equatorial)
    pub surface_axis_equatorial: f64,
    // @@protoc_insertion_point(field:gz.msgs.SphericalCoordinates.surface_axis_polar)
    pub surface_axis_polar: f64,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.SphericalCoordinates.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SphericalCoordinates {
    fn default() -> &'a SphericalCoordinates {
        <SphericalCoordinates as ::protobuf::Message>::default_instance()
    }
}

impl SphericalCoordinates {
    pub fn new() -> SphericalCoordinates {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &SphericalCoordinates| { &m.header },
            |m: &mut SphericalCoordinates| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "surface_model",
            |m: &SphericalCoordinates| { &m.surface_model },
            |m: &mut SphericalCoordinates| { &mut m.surface_model },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "latitude_deg",
            |m: &SphericalCoordinates| { &m.latitude_deg },
            |m: &mut SphericalCoordinates| { &mut m.latitude_deg },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "longitude_deg",
            |m: &SphericalCoordinates| { &m.longitude_deg },
            |m: &mut SphericalCoordinates| { &mut m.longitude_deg },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "elevation",
            |m: &SphericalCoordinates| { &m.elevation },
            |m: &mut SphericalCoordinates| { &mut m.elevation },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "heading_deg",
            |m: &SphericalCoordinates| { &m.heading_deg },
            |m: &mut SphericalCoordinates| { &mut m.heading_deg },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::entity::Entity>(
            "entity",
            |m: &SphericalCoordinates| { &m.entity },
            |m: &mut SphericalCoordinates| { &mut m.entity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "surface_axis_equatorial",
            |m: &SphericalCoordinates| { &m.surface_axis_equatorial },
            |m: &mut SphericalCoordinates| { &mut m.surface_axis_equatorial },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "surface_axis_polar",
            |m: &SphericalCoordinates| { &m.surface_axis_polar },
            |m: &mut SphericalCoordinates| { &mut m.surface_axis_polar },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SphericalCoordinates>(
            "SphericalCoordinates",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SphericalCoordinates {
    const NAME: &'static str = "SphericalCoordinates";

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
                    self.surface_model = is.read_enum_or_unknown()?;
                },
                25 => {
                    self.latitude_deg = is.read_double()?;
                },
                33 => {
                    self.longitude_deg = is.read_double()?;
                },
                41 => {
                    self.elevation = is.read_double()?;
                },
                49 => {
                    self.heading_deg = is.read_double()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.entity)?;
                },
                65 => {
                    self.surface_axis_equatorial = is.read_double()?;
                },
                73 => {
                    self.surface_axis_polar = is.read_double()?;
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
        if self.surface_model != ::protobuf::EnumOrUnknown::new(spherical_coordinates::SurfaceModel::EARTH_WGS84) {
            my_size += ::protobuf::rt::int32_size(2, self.surface_model.value());
        }
        if self.latitude_deg != 0. {
            my_size += 1 + 8;
        }
        if self.longitude_deg != 0. {
            my_size += 1 + 8;
        }
        if self.elevation != 0. {
            my_size += 1 + 8;
        }
        if self.heading_deg != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.entity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.surface_axis_equatorial != 0. {
            my_size += 1 + 8;
        }
        if self.surface_axis_polar != 0. {
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
        if self.surface_model != ::protobuf::EnumOrUnknown::new(spherical_coordinates::SurfaceModel::EARTH_WGS84) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.surface_model))?;
        }
        if self.latitude_deg != 0. {
            os.write_double(3, self.latitude_deg)?;
        }
        if self.longitude_deg != 0. {
            os.write_double(4, self.longitude_deg)?;
        }
        if self.elevation != 0. {
            os.write_double(5, self.elevation)?;
        }
        if self.heading_deg != 0. {
            os.write_double(6, self.heading_deg)?;
        }
        if let Some(v) = self.entity.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.surface_axis_equatorial != 0. {
            os.write_double(8, self.surface_axis_equatorial)?;
        }
        if self.surface_axis_polar != 0. {
            os.write_double(9, self.surface_axis_polar)?;
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

    fn new() -> SphericalCoordinates {
        SphericalCoordinates::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.surface_model = ::protobuf::EnumOrUnknown::new(spherical_coordinates::SurfaceModel::EARTH_WGS84);
        self.latitude_deg = 0.;
        self.longitude_deg = 0.;
        self.elevation = 0.;
        self.heading_deg = 0.;
        self.entity.clear();
        self.surface_axis_equatorial = 0.;
        self.surface_axis_polar = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SphericalCoordinates {
        static instance: SphericalCoordinates = SphericalCoordinates {
            header: ::protobuf::MessageField::none(),
            surface_model: ::protobuf::EnumOrUnknown::from_i32(0),
            latitude_deg: 0.,
            longitude_deg: 0.,
            elevation: 0.,
            heading_deg: 0.,
            entity: ::protobuf::MessageField::none(),
            surface_axis_equatorial: 0.,
            surface_axis_polar: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SphericalCoordinates {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SphericalCoordinates").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SphericalCoordinates {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SphericalCoordinates {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `SphericalCoordinates`
pub mod spherical_coordinates {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.SphericalCoordinates.SurfaceModel)
    pub enum SurfaceModel {
        // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinates.SurfaceModel.EARTH_WGS84)
        EARTH_WGS84 = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinates.SurfaceModel.MOON_SCS)
        MOON_SCS = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinates.SurfaceModel.CUSTOM_SURFACE)
        CUSTOM_SURFACE = 2,
    }

    impl ::protobuf::Enum for SurfaceModel {
        const NAME: &'static str = "SurfaceModel";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<SurfaceModel> {
            match value {
                0 => ::std::option::Option::Some(SurfaceModel::EARTH_WGS84),
                1 => ::std::option::Option::Some(SurfaceModel::MOON_SCS),
                2 => ::std::option::Option::Some(SurfaceModel::CUSTOM_SURFACE),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<SurfaceModel> {
            match str {
                "EARTH_WGS84" => ::std::option::Option::Some(SurfaceModel::EARTH_WGS84),
                "MOON_SCS" => ::std::option::Option::Some(SurfaceModel::MOON_SCS),
                "CUSTOM_SURFACE" => ::std::option::Option::Some(SurfaceModel::CUSTOM_SURFACE),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [SurfaceModel] = &[
            SurfaceModel::EARTH_WGS84,
            SurfaceModel::MOON_SCS,
            SurfaceModel::CUSTOM_SURFACE,
        ];
    }

    impl ::protobuf::EnumFull for SurfaceModel {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("SphericalCoordinates.SurfaceModel").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for SurfaceModel {
        fn default() -> Self {
            SurfaceModel::EARTH_WGS84
        }
    }

    impl SurfaceModel {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<SurfaceModel>("SphericalCoordinates.SurfaceModel")
        }
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:gz.msgs.SphericalCoordinatesType)
pub enum SphericalCoordinatesType {
    // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinatesType.SPHERICAL)
    SPHERICAL = 0,
    // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinatesType.ECEF)
    ECEF = 1,
    // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinatesType.GLOBAL)
    GLOBAL = 2,
    // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinatesType.LOCAL)
    LOCAL = 3,
    // @@protoc_insertion_point(enum_value:gz.msgs.SphericalCoordinatesType.LOCAL2)
    LOCAL2 = 4,
}

impl ::protobuf::Enum for SphericalCoordinatesType {
    const NAME: &'static str = "SphericalCoordinatesType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SphericalCoordinatesType> {
        match value {
            0 => ::std::option::Option::Some(SphericalCoordinatesType::SPHERICAL),
            1 => ::std::option::Option::Some(SphericalCoordinatesType::ECEF),
            2 => ::std::option::Option::Some(SphericalCoordinatesType::GLOBAL),
            3 => ::std::option::Option::Some(SphericalCoordinatesType::LOCAL),
            4 => ::std::option::Option::Some(SphericalCoordinatesType::LOCAL2),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<SphericalCoordinatesType> {
        match str {
            "SPHERICAL" => ::std::option::Option::Some(SphericalCoordinatesType::SPHERICAL),
            "ECEF" => ::std::option::Option::Some(SphericalCoordinatesType::ECEF),
            "GLOBAL" => ::std::option::Option::Some(SphericalCoordinatesType::GLOBAL),
            "LOCAL" => ::std::option::Option::Some(SphericalCoordinatesType::LOCAL),
            "LOCAL2" => ::std::option::Option::Some(SphericalCoordinatesType::LOCAL2),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [SphericalCoordinatesType] = &[
        SphericalCoordinatesType::SPHERICAL,
        SphericalCoordinatesType::ECEF,
        SphericalCoordinatesType::GLOBAL,
        SphericalCoordinatesType::LOCAL,
        SphericalCoordinatesType::LOCAL2,
    ];
}

impl ::protobuf::EnumFull for SphericalCoordinatesType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("SphericalCoordinatesType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for SphericalCoordinatesType {
    fn default() -> Self {
        SphericalCoordinatesType::SPHERICAL
    }
}

impl SphericalCoordinatesType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<SphericalCoordinatesType>("SphericalCoordinatesType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#gz/msgs/spherical_coordinates.proto\x12\x07gz.msgs\x1a\x14gz/msgs/ent\
    ity.proto\x1a\x14gz/msgs/header.proto\"\xe9\x03\n\x14SphericalCoordinate\
    s\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\
    \x12O\n\rsurface_model\x18\x02\x20\x01(\x0e2*.gz.msgs.SphericalCoordinat\
    es.SurfaceModelR\x0csurfaceModel\x12!\n\x0clatitude_deg\x18\x03\x20\x01(\
    \x01R\x0blatitudeDeg\x12#\n\rlongitude_deg\x18\x04\x20\x01(\x01R\x0clong\
    itudeDeg\x12\x1c\n\televation\x18\x05\x20\x01(\x01R\televation\x12\x1f\n\
    \x0bheading_deg\x18\x06\x20\x01(\x01R\nheadingDeg\x12'\n\x06entity\x18\
    \x07\x20\x01(\x0b2\x0f.gz.msgs.EntityR\x06entity\x126\n\x17surface_axis_\
    equatorial\x18\x08\x20\x01(\x01R\x15surfaceAxisEquatorial\x12,\n\x12surf\
    ace_axis_polar\x18\t\x20\x01(\x01R\x10surfaceAxisPolar\"A\n\x0cSurfaceMo\
    del\x12\x0f\n\x0bEARTH_WGS84\x10\0\x12\x0c\n\x08MOON_SCS\x10\x01\x12\x12\
    \n\x0eCUSTOM_SURFACE\x10\x02*V\n\x18SphericalCoordinatesType\x12\r\n\tSP\
    HERICAL\x10\0\x12\x08\n\x04ECEF\x10\x01\x12\n\n\x06GLOBAL\x10\x02\x12\t\
    \n\x05LOCAL\x10\x03\x12\n\n\x06LOCAL2\x10\x04B)\n\x0bcom.gz.msgsB\x1aSph\
    ericalCoordinatesProtosb\x06proto3\
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
            deps.push(super::entity::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SphericalCoordinates::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(2);
            enums.push(SphericalCoordinatesType::generated_enum_descriptor_data());
            enums.push(spherical_coordinates::SurfaceModel::generated_enum_descriptor_data());
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
