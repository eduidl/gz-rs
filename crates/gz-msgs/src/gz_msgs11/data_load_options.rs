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

//! Generated file from `gz/msgs/data_load_options.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.DataLoadPathOptions)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DataLoadPathOptions {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.path)
    pub path: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.time)
    pub time: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.static_time)
    pub static_time: bool,
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.x)
    pub x: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.y)
    pub y: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.z)
    pub z: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.units)
    pub units: ::protobuf::EnumOrUnknown<data_load_path_options::DataAngularUnits>,
    // @@protoc_insertion_point(field:gz.msgs.DataLoadPathOptions.coordinate_type)
    pub coordinate_type: ::protobuf::EnumOrUnknown<super::spherical_coordinates::SphericalCoordinatesType>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.DataLoadPathOptions.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DataLoadPathOptions {
    fn default() -> &'a DataLoadPathOptions {
        <DataLoadPathOptions as ::protobuf::Message>::default_instance()
    }
}

impl DataLoadPathOptions {
    pub fn new() -> DataLoadPathOptions {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "path",
            |m: &DataLoadPathOptions| { &m.path },
            |m: &mut DataLoadPathOptions| { &mut m.path },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time",
            |m: &DataLoadPathOptions| { &m.time },
            |m: &mut DataLoadPathOptions| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "static_time",
            |m: &DataLoadPathOptions| { &m.static_time },
            |m: &mut DataLoadPathOptions| { &mut m.static_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "x",
            |m: &DataLoadPathOptions| { &m.x },
            |m: &mut DataLoadPathOptions| { &mut m.x },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "y",
            |m: &DataLoadPathOptions| { &m.y },
            |m: &mut DataLoadPathOptions| { &mut m.y },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "z",
            |m: &DataLoadPathOptions| { &m.z },
            |m: &mut DataLoadPathOptions| { &mut m.z },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "units",
            |m: &DataLoadPathOptions| { &m.units },
            |m: &mut DataLoadPathOptions| { &mut m.units },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "coordinate_type",
            |m: &DataLoadPathOptions| { &m.coordinate_type },
            |m: &mut DataLoadPathOptions| { &mut m.coordinate_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DataLoadPathOptions>(
            "DataLoadPathOptions",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DataLoadPathOptions {
    const NAME: &'static str = "DataLoadPathOptions";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.path = is.read_string()?;
                },
                18 => {
                    self.time = is.read_string()?;
                },
                24 => {
                    self.static_time = is.read_bool()?;
                },
                34 => {
                    self.x = is.read_string()?;
                },
                42 => {
                    self.y = is.read_string()?;
                },
                50 => {
                    self.z = is.read_string()?;
                },
                56 => {
                    self.units = is.read_enum_or_unknown()?;
                },
                64 => {
                    self.coordinate_type = is.read_enum_or_unknown()?;
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
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.path);
        }
        if !self.time.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.time);
        }
        if self.static_time != false {
            my_size += 1 + 1;
        }
        if !self.x.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.x);
        }
        if !self.y.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.y);
        }
        if !self.z.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.z);
        }
        if self.units != ::protobuf::EnumOrUnknown::new(data_load_path_options::DataAngularUnits::RADIANS) {
            my_size += ::protobuf::rt::int32_size(7, self.units.value());
        }
        if self.coordinate_type != ::protobuf::EnumOrUnknown::new(super::spherical_coordinates::SphericalCoordinatesType::SPHERICAL) {
            my_size += ::protobuf::rt::int32_size(8, self.coordinate_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
        }
        if !self.time.is_empty() {
            os.write_string(2, &self.time)?;
        }
        if self.static_time != false {
            os.write_bool(3, self.static_time)?;
        }
        if !self.x.is_empty() {
            os.write_string(4, &self.x)?;
        }
        if !self.y.is_empty() {
            os.write_string(5, &self.y)?;
        }
        if !self.z.is_empty() {
            os.write_string(6, &self.z)?;
        }
        if self.units != ::protobuf::EnumOrUnknown::new(data_load_path_options::DataAngularUnits::RADIANS) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.units))?;
        }
        if self.coordinate_type != ::protobuf::EnumOrUnknown::new(super::spherical_coordinates::SphericalCoordinatesType::SPHERICAL) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.coordinate_type))?;
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

    fn new() -> DataLoadPathOptions {
        DataLoadPathOptions::new()
    }

    fn clear(&mut self) {
        self.path.clear();
        self.time.clear();
        self.static_time = false;
        self.x.clear();
        self.y.clear();
        self.z.clear();
        self.units = ::protobuf::EnumOrUnknown::new(data_load_path_options::DataAngularUnits::RADIANS);
        self.coordinate_type = ::protobuf::EnumOrUnknown::new(super::spherical_coordinates::SphericalCoordinatesType::SPHERICAL);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DataLoadPathOptions {
        static instance: DataLoadPathOptions = DataLoadPathOptions {
            path: ::std::string::String::new(),
            time: ::std::string::String::new(),
            static_time: false,
            x: ::std::string::String::new(),
            y: ::std::string::String::new(),
            z: ::std::string::String::new(),
            units: ::protobuf::EnumOrUnknown::from_i32(0),
            coordinate_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DataLoadPathOptions {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DataLoadPathOptions").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DataLoadPathOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataLoadPathOptions {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `DataLoadPathOptions`
pub mod data_load_path_options {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.DataLoadPathOptions.DataAngularUnits)
    pub enum DataAngularUnits {
        // @@protoc_insertion_point(enum_value:gz.msgs.DataLoadPathOptions.DataAngularUnits.RADIANS)
        RADIANS = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.DataLoadPathOptions.DataAngularUnits.DEGREES)
        DEGREES = 1,
    }

    impl ::protobuf::Enum for DataAngularUnits {
        const NAME: &'static str = "DataAngularUnits";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<DataAngularUnits> {
            match value {
                0 => ::std::option::Option::Some(DataAngularUnits::RADIANS),
                1 => ::std::option::Option::Some(DataAngularUnits::DEGREES),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<DataAngularUnits> {
            match str {
                "RADIANS" => ::std::option::Option::Some(DataAngularUnits::RADIANS),
                "DEGREES" => ::std::option::Option::Some(DataAngularUnits::DEGREES),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [DataAngularUnits] = &[
            DataAngularUnits::RADIANS,
            DataAngularUnits::DEGREES,
        ];
    }

    impl ::protobuf::EnumFull for DataAngularUnits {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("DataLoadPathOptions.DataAngularUnits").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for DataAngularUnits {
        fn default() -> Self {
            DataAngularUnits::RADIANS
        }
    }

    impl DataAngularUnits {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DataAngularUnits>("DataLoadPathOptions.DataAngularUnits")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fgz/msgs/data_load_options.proto\x12\x07gz.msgs\x1a#gz/msgs/spheric\
    al_coordinates.proto\"\xc7\x02\n\x13DataLoadPathOptions\x12\x12\n\x04pat\
    h\x18\x01\x20\x01(\tR\x04path\x12\x12\n\x04time\x18\x02\x20\x01(\tR\x04t\
    ime\x12\x1f\n\x0bstatic_time\x18\x03\x20\x01(\x08R\nstaticTime\x12\x0c\n\
    \x01x\x18\x04\x20\x01(\tR\x01x\x12\x0c\n\x01y\x18\x05\x20\x01(\tR\x01y\
    \x12\x0c\n\x01z\x18\x06\x20\x01(\tR\x01z\x12C\n\x05units\x18\x07\x20\x01\
    (\x0e2-.gz.msgs.DataLoadPathOptions.DataAngularUnitsR\x05units\x12J\n\
    \x0fcoordinate_type\x18\x08\x20\x01(\x0e2!.gz.msgs.SphericalCoordinatesT\
    ypeR\x0ecoordinateType\",\n\x10DataAngularUnits\x12\x0b\n\x07RADIANS\x10\
    \0\x12\x0b\n\x07DEGREES\x10\x01B\"\n\x0bcom.gz.msgsB\x13DataLoadPathOpti\
    onsb\x06proto3\
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
            deps.push(super::spherical_coordinates::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DataLoadPathOptions::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(data_load_path_options::DataAngularUnits::generated_enum_descriptor_data());
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
