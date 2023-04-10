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

//! Generated file from `gz/msgs/any.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Any)
pub struct Any {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Any.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Any.type)
    pub type_: ::protobuf::EnumOrUnknown<any::ValueType>,
    // message oneof groups
    pub value: ::std::option::Option<any::Value>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Any.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Any {
    fn default() -> &'a Any {
        <Any as ::protobuf::Message>::default_instance()
    }
}

impl Any {
    pub fn new() -> Any {
        ::std::default::Default::default()
    }

    // double double_value = 3;

    pub fn double_value(&self) -> f64 {
        match self.value {
            ::std::option::Option::Some(any::Value::DoubleValue(v)) => v,
            _ => 0.,
        }
    }

    pub fn clear_double_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_double_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::DoubleValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(any::Value::DoubleValue(v))
    }

    // int32 int_value = 4;

    pub fn int_value(&self) -> i32 {
        match self.value {
            ::std::option::Option::Some(any::Value::IntValue(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_int_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::IntValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(any::Value::IntValue(v))
    }

    // string string_value = 5;

    pub fn string_value(&self) -> &str {
        match self.value {
            ::std::option::Option::Some(any::Value::StringValue(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_string_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_string_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::StringValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(any::Value::StringValue(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(any::Value::StringValue(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(any::Value::StringValue(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(any::Value::StringValue(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        if self.has_string_value() {
            match self.value.take() {
                ::std::option::Option::Some(any::Value::StringValue(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // bool bool_value = 6;

    pub fn bool_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::BoolValue(v)) => v,
            _ => false,
        }
    }

    pub fn clear_bool_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::BoolValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(any::Value::BoolValue(v))
    }

    // .gz.msgs.Vector3d vector3d_value = 7;

    pub fn vector3d_value(&self) -> &super::vector3d::Vector3d {
        match self.value {
            ::std::option::Option::Some(any::Value::Vector3dValue(ref v)) => v,
            _ => <super::vector3d::Vector3d as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_vector3d_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_vector3d_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::Vector3dValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vector3d_value(&mut self, v: super::vector3d::Vector3d) {
        self.value = ::std::option::Option::Some(any::Value::Vector3dValue(v))
    }

    // Mutable pointer to the field.
    pub fn mut_vector3d_value(&mut self) -> &mut super::vector3d::Vector3d {
        if let ::std::option::Option::Some(any::Value::Vector3dValue(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(any::Value::Vector3dValue(super::vector3d::Vector3d::new()));
        }
        match self.value {
            ::std::option::Option::Some(any::Value::Vector3dValue(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vector3d_value(&mut self) -> super::vector3d::Vector3d {
        if self.has_vector3d_value() {
            match self.value.take() {
                ::std::option::Option::Some(any::Value::Vector3dValue(v)) => v,
                _ => panic!(),
            }
        } else {
            super::vector3d::Vector3d::new()
        }
    }

    // .gz.msgs.Color color_value = 8;

    pub fn color_value(&self) -> &super::color::Color {
        match self.value {
            ::std::option::Option::Some(any::Value::ColorValue(ref v)) => v,
            _ => <super::color::Color as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_color_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_color_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::ColorValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_color_value(&mut self, v: super::color::Color) {
        self.value = ::std::option::Option::Some(any::Value::ColorValue(v))
    }

    // Mutable pointer to the field.
    pub fn mut_color_value(&mut self) -> &mut super::color::Color {
        if let ::std::option::Option::Some(any::Value::ColorValue(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(any::Value::ColorValue(super::color::Color::new()));
        }
        match self.value {
            ::std::option::Option::Some(any::Value::ColorValue(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_color_value(&mut self) -> super::color::Color {
        if self.has_color_value() {
            match self.value.take() {
                ::std::option::Option::Some(any::Value::ColorValue(v)) => v,
                _ => panic!(),
            }
        } else {
            super::color::Color::new()
        }
    }

    // .gz.msgs.Pose pose3d_value = 9;

    pub fn pose3d_value(&self) -> &super::pose::Pose {
        match self.value {
            ::std::option::Option::Some(any::Value::Pose3dValue(ref v)) => v,
            _ => <super::pose::Pose as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_pose3d_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_pose3d_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::Pose3dValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pose3d_value(&mut self, v: super::pose::Pose) {
        self.value = ::std::option::Option::Some(any::Value::Pose3dValue(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pose3d_value(&mut self) -> &mut super::pose::Pose {
        if let ::std::option::Option::Some(any::Value::Pose3dValue(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(any::Value::Pose3dValue(super::pose::Pose::new()));
        }
        match self.value {
            ::std::option::Option::Some(any::Value::Pose3dValue(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pose3d_value(&mut self) -> super::pose::Pose {
        if self.has_pose3d_value() {
            match self.value.take() {
                ::std::option::Option::Some(any::Value::Pose3dValue(v)) => v,
                _ => panic!(),
            }
        } else {
            super::pose::Pose::new()
        }
    }

    // .gz.msgs.Quaternion quaternion_value = 10;

    pub fn quaternion_value(&self) -> &super::quaternion::Quaternion {
        match self.value {
            ::std::option::Option::Some(any::Value::QuaternionValue(ref v)) => v,
            _ => <super::quaternion::Quaternion as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_quaternion_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_quaternion_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::QuaternionValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quaternion_value(&mut self, v: super::quaternion::Quaternion) {
        self.value = ::std::option::Option::Some(any::Value::QuaternionValue(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quaternion_value(&mut self) -> &mut super::quaternion::Quaternion {
        if let ::std::option::Option::Some(any::Value::QuaternionValue(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(any::Value::QuaternionValue(super::quaternion::Quaternion::new()));
        }
        match self.value {
            ::std::option::Option::Some(any::Value::QuaternionValue(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quaternion_value(&mut self) -> super::quaternion::Quaternion {
        if self.has_quaternion_value() {
            match self.value.take() {
                ::std::option::Option::Some(any::Value::QuaternionValue(v)) => v,
                _ => panic!(),
            }
        } else {
            super::quaternion::Quaternion::new()
        }
    }

    // .gz.msgs.Time time_value = 11;

    pub fn time_value(&self) -> &super::time::Time {
        match self.value {
            ::std::option::Option::Some(any::Value::TimeValue(ref v)) => v,
            _ => <super::time::Time as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_time_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_time_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(any::Value::TimeValue(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_time_value(&mut self, v: super::time::Time) {
        self.value = ::std::option::Option::Some(any::Value::TimeValue(v))
    }

    // Mutable pointer to the field.
    pub fn mut_time_value(&mut self) -> &mut super::time::Time {
        if let ::std::option::Option::Some(any::Value::TimeValue(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(any::Value::TimeValue(super::time::Time::new()));
        }
        match self.value {
            ::std::option::Option::Some(any::Value::TimeValue(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_time_value(&mut self) -> super::time::Time {
        if self.has_time_value() {
            match self.value.take() {
                ::std::option::Option::Some(any::Value::TimeValue(v)) => v,
                _ => panic!(),
            }
        } else {
            super::time::Time::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Any| { &m.header },
            |m: &mut Any| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Any| { &m.type_ },
            |m: &mut Any| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "double_value",
            Any::has_double_value,
            Any::double_value,
            Any::set_double_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "int_value",
            Any::has_int_value,
            Any::int_value,
            Any::set_int_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "string_value",
            Any::has_string_value,
            Any::string_value,
            Any::set_string_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "bool_value",
            Any::has_bool_value,
            Any::bool_value,
            Any::set_bool_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::vector3d::Vector3d>(
            "vector3d_value",
            Any::has_vector3d_value,
            Any::vector3d_value,
            Any::mut_vector3d_value,
            Any::set_vector3d_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::color::Color>(
            "color_value",
            Any::has_color_value,
            Any::color_value,
            Any::mut_color_value,
            Any::set_color_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::pose::Pose>(
            "pose3d_value",
            Any::has_pose3d_value,
            Any::pose3d_value,
            Any::mut_pose3d_value,
            Any::set_pose3d_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::quaternion::Quaternion>(
            "quaternion_value",
            Any::has_quaternion_value,
            Any::quaternion_value,
            Any::mut_quaternion_value,
            Any::set_quaternion_value,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::time::Time>(
            "time_value",
            Any::has_time_value,
            Any::time_value,
            Any::mut_time_value,
            Any::set_time_value,
        ));
        oneofs.push(any::Value::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Any>(
            "Any",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Any {
    const NAME: &'static str = "Any";

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
                25 => {
                    self.value = ::std::option::Option::Some(any::Value::DoubleValue(is.read_double()?));
                },
                32 => {
                    self.value = ::std::option::Option::Some(any::Value::IntValue(is.read_int32()?));
                },
                42 => {
                    self.value = ::std::option::Option::Some(any::Value::StringValue(is.read_string()?));
                },
                48 => {
                    self.value = ::std::option::Option::Some(any::Value::BoolValue(is.read_bool()?));
                },
                58 => {
                    self.value = ::std::option::Option::Some(any::Value::Vector3dValue(is.read_message()?));
                },
                66 => {
                    self.value = ::std::option::Option::Some(any::Value::ColorValue(is.read_message()?));
                },
                74 => {
                    self.value = ::std::option::Option::Some(any::Value::Pose3dValue(is.read_message()?));
                },
                82 => {
                    self.value = ::std::option::Option::Some(any::Value::QuaternionValue(is.read_message()?));
                },
                90 => {
                    self.value = ::std::option::Option::Some(any::Value::TimeValue(is.read_message()?));
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(any::ValueType::NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.type_.value());
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &any::Value::DoubleValue(v) => {
                    my_size += 1 + 8;
                },
                &any::Value::IntValue(v) => {
                    my_size += ::protobuf::rt::int32_size(4, v);
                },
                &any::Value::StringValue(ref v) => {
                    my_size += ::protobuf::rt::string_size(5, &v);
                },
                &any::Value::BoolValue(v) => {
                    my_size += 1 + 1;
                },
                &any::Value::Vector3dValue(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &any::Value::ColorValue(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &any::Value::Pose3dValue(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &any::Value::QuaternionValue(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &any::Value::TimeValue(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(any::ValueType::NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &any::Value::DoubleValue(v) => {
                    os.write_double(3, v)?;
                },
                &any::Value::IntValue(v) => {
                    os.write_int32(4, v)?;
                },
                &any::Value::StringValue(ref v) => {
                    os.write_string(5, v)?;
                },
                &any::Value::BoolValue(v) => {
                    os.write_bool(6, v)?;
                },
                &any::Value::Vector3dValue(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &any::Value::ColorValue(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &any::Value::Pose3dValue(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &any::Value::QuaternionValue(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &any::Value::TimeValue(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
                },
            };
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

    fn new() -> Any {
        Any::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(any::ValueType::NONE);
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Any {
        static instance: Any = Any {
            header: ::protobuf::MessageField::none(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            value: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Any {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Any").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Any {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Any {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Any`
pub mod any {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:gz.msgs.Any.value)
    pub enum Value {
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.double_value)
        DoubleValue(f64),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.int_value)
        IntValue(i32),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.string_value)
        StringValue(::std::string::String),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.bool_value)
        BoolValue(bool),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.vector3d_value)
        Vector3dValue(super::super::vector3d::Vector3d),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.color_value)
        ColorValue(super::super::color::Color),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.pose3d_value)
        Pose3dValue(super::super::pose::Pose),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.quaternion_value)
        QuaternionValue(super::super::quaternion::Quaternion),
        // @@protoc_insertion_point(oneof_field:gz.msgs.Any.time_value)
        TimeValue(super::super::time::Time),
    }

    impl ::protobuf::Oneof for Value {
    }

    impl ::protobuf::OneofFull for Value {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::Any as ::protobuf::MessageFull>::descriptor().oneof_by_name("value").unwrap()).clone()
        }
    }

    impl Value {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Value>("value")
        }
    }
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.Any.ValueType)
    pub enum ValueType {
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.NONE)
        NONE = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.DOUBLE)
        DOUBLE = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.INT32)
        INT32 = 2,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.STRING)
        STRING = 3,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.BOOLEAN)
        BOOLEAN = 4,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.VECTOR3D)
        VECTOR3D = 5,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.COLOR)
        COLOR = 6,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.POSE3D)
        POSE3D = 7,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.QUATERNIOND)
        QUATERNIOND = 8,
        // @@protoc_insertion_point(enum_value:gz.msgs.Any.ValueType.TIME)
        TIME = 9,
    }

    impl ::protobuf::Enum for ValueType {
        const NAME: &'static str = "ValueType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<ValueType> {
            match value {
                0 => ::std::option::Option::Some(ValueType::NONE),
                1 => ::std::option::Option::Some(ValueType::DOUBLE),
                2 => ::std::option::Option::Some(ValueType::INT32),
                3 => ::std::option::Option::Some(ValueType::STRING),
                4 => ::std::option::Option::Some(ValueType::BOOLEAN),
                5 => ::std::option::Option::Some(ValueType::VECTOR3D),
                6 => ::std::option::Option::Some(ValueType::COLOR),
                7 => ::std::option::Option::Some(ValueType::POSE3D),
                8 => ::std::option::Option::Some(ValueType::QUATERNIOND),
                9 => ::std::option::Option::Some(ValueType::TIME),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [ValueType] = &[
            ValueType::NONE,
            ValueType::DOUBLE,
            ValueType::INT32,
            ValueType::STRING,
            ValueType::BOOLEAN,
            ValueType::VECTOR3D,
            ValueType::COLOR,
            ValueType::POSE3D,
            ValueType::QUATERNIOND,
            ValueType::TIME,
        ];
    }

    impl ::protobuf::EnumFull for ValueType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Any.ValueType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for ValueType {
        fn default() -> Self {
            ValueType::NONE
        }
    }

    impl ValueType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ValueType>("Any.ValueType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11gz/msgs/any.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.proto\x1a\
    \x13gz/msgs/color.proto\x1a\x12gz/msgs/pose.proto\x1a\x18gz/msgs/quatern\
    ion.proto\x1a\x12gz/msgs/time.proto\x1a\x16gz/msgs/vector3d.proto\"\x8a\
    \x05\n\x03Any\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\
    \x06header\x12*\n\x04type\x18\x02\x20\x01(\x0e2\x16.gz.msgs.Any.ValueTyp\
    eR\x04type\x12#\n\x0cdouble_value\x18\x03\x20\x01(\x01H\0R\x0bdoubleValu\
    e\x12\x1d\n\tint_value\x18\x04\x20\x01(\x05H\0R\x08intValue\x12#\n\x0cst\
    ring_value\x18\x05\x20\x01(\tH\0R\x0bstringValue\x12\x1f\n\nbool_value\
    \x18\x06\x20\x01(\x08H\0R\tboolValue\x12:\n\x0evector3d_value\x18\x07\
    \x20\x01(\x0b2\x11.gz.msgs.Vector3dH\0R\rvector3dValue\x121\n\x0bcolor_v\
    alue\x18\x08\x20\x01(\x0b2\x0e.gz.msgs.ColorH\0R\ncolorValue\x122\n\x0cp\
    ose3d_value\x18\t\x20\x01(\x0b2\r.gz.msgs.PoseH\0R\x0bpose3dValue\x12@\n\
    \x10quaternion_value\x18\n\x20\x01(\x0b2\x13.gz.msgs.QuaternionH\0R\x0fq\
    uaternionValue\x12.\n\ntime_value\x18\x0b\x20\x01(\x0b2\r.gz.msgs.TimeH\
    \0R\ttimeValue\"\x85\x01\n\tValueType\x12\x08\n\x04NONE\x10\0\x12\n\n\
    \x06DOUBLE\x10\x01\x12\t\n\x05INT32\x10\x02\x12\n\n\x06STRING\x10\x03\
    \x12\x0b\n\x07BOOLEAN\x10\x04\x12\x0c\n\x08VECTOR3D\x10\x05\x12\t\n\x05C\
    OLOR\x10\x06\x12\n\n\x06POSE3D\x10\x07\x12\x0f\n\x0bQUATERNIOND\x10\x08\
    \x12\x08\n\x04TIME\x10\tB\x07\n\x05valueB\x1a\n\x0bcom.gz.msgsB\x0bEmpty\
    Protosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::color::file_descriptor().clone());
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::quaternion::file_descriptor().clone());
            deps.push(super::time::file_descriptor().clone());
            deps.push(super::vector3d::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Any::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(any::ValueType::generated_enum_descriptor_data());
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
