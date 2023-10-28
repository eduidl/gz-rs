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

//! Generated file from `ignition/msgs/joint_trajectory_point.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.JointTrajectoryPoint)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JointTrajectoryPoint {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.JointTrajectoryPoint.positions)
    pub positions: ::std::vec::Vec<f64>,
    // @@protoc_insertion_point(field:ignition.msgs.JointTrajectoryPoint.velocities)
    pub velocities: ::std::vec::Vec<f64>,
    // @@protoc_insertion_point(field:ignition.msgs.JointTrajectoryPoint.accelerations)
    pub accelerations: ::std::vec::Vec<f64>,
    // @@protoc_insertion_point(field:ignition.msgs.JointTrajectoryPoint.effort)
    pub effort: ::std::vec::Vec<f64>,
    // @@protoc_insertion_point(field:ignition.msgs.JointTrajectoryPoint.time_from_start)
    pub time_from_start: ::protobuf::MessageField<super::duration::Duration>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.JointTrajectoryPoint.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JointTrajectoryPoint {
    fn default() -> &'a JointTrajectoryPoint {
        <JointTrajectoryPoint as ::protobuf::Message>::default_instance()
    }
}

impl JointTrajectoryPoint {
    pub fn new() -> JointTrajectoryPoint {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "positions",
            |m: &JointTrajectoryPoint| { &m.positions },
            |m: &mut JointTrajectoryPoint| { &mut m.positions },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "velocities",
            |m: &JointTrajectoryPoint| { &m.velocities },
            |m: &mut JointTrajectoryPoint| { &mut m.velocities },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "accelerations",
            |m: &JointTrajectoryPoint| { &m.accelerations },
            |m: &mut JointTrajectoryPoint| { &mut m.accelerations },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "effort",
            |m: &JointTrajectoryPoint| { &m.effort },
            |m: &mut JointTrajectoryPoint| { &mut m.effort },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::duration::Duration>(
            "time_from_start",
            |m: &JointTrajectoryPoint| { &m.time_from_start },
            |m: &mut JointTrajectoryPoint| { &mut m.time_from_start },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JointTrajectoryPoint>(
            "JointTrajectoryPoint",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JointTrajectoryPoint {
    const NAME: &'static str = "JointTrajectoryPoint";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_double_into(&mut self.positions)?;
                },
                9 => {
                    self.positions.push(is.read_double()?);
                },
                18 => {
                    is.read_repeated_packed_double_into(&mut self.velocities)?;
                },
                17 => {
                    self.velocities.push(is.read_double()?);
                },
                26 => {
                    is.read_repeated_packed_double_into(&mut self.accelerations)?;
                },
                25 => {
                    self.accelerations.push(is.read_double()?);
                },
                34 => {
                    is.read_repeated_packed_double_into(&mut self.effort)?;
                },
                33 => {
                    self.effort.push(is.read_double()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.time_from_start)?;
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
        my_size += 9 * self.positions.len() as u64;
        my_size += 9 * self.velocities.len() as u64;
        my_size += 9 * self.accelerations.len() as u64;
        my_size += 9 * self.effort.len() as u64;
        if let Some(v) = self.time_from_start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.positions {
            os.write_double(1, *v)?;
        };
        for v in &self.velocities {
            os.write_double(2, *v)?;
        };
        for v in &self.accelerations {
            os.write_double(3, *v)?;
        };
        for v in &self.effort {
            os.write_double(4, *v)?;
        };
        if let Some(v) = self.time_from_start.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> JointTrajectoryPoint {
        JointTrajectoryPoint::new()
    }

    fn clear(&mut self) {
        self.positions.clear();
        self.velocities.clear();
        self.accelerations.clear();
        self.effort.clear();
        self.time_from_start.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JointTrajectoryPoint {
        static instance: JointTrajectoryPoint = JointTrajectoryPoint {
            positions: ::std::vec::Vec::new(),
            velocities: ::std::vec::Vec::new(),
            accelerations: ::std::vec::Vec::new(),
            effort: ::std::vec::Vec::new(),
            time_from_start: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JointTrajectoryPoint {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JointTrajectoryPoint").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JointTrajectoryPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JointTrajectoryPoint {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*ignition/msgs/joint_trajectory_point.proto\x12\rignition.msgs\x1a\x1c\
    ignition/msgs/duration.proto\"\xd3\x01\n\x14JointTrajectoryPoint\x12\x1c\
    \n\tpositions\x18\x01\x20\x03(\x01R\tpositions\x12\x1e\n\nvelocities\x18\
    \x02\x20\x03(\x01R\nvelocities\x12$\n\raccelerations\x18\x03\x20\x03(\
    \x01R\raccelerations\x12\x16\n\x06effort\x18\x04\x20\x03(\x01R\x06effort\
    \x12?\n\x0ftime_from_start\x18\x05\x20\x01(\x0b2\x17.ignition.msgs.Durat\
    ionR\rtimeFromStartB/\n\x11com.ignition.msgsB\x1aJointTrajectoryPointPro\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::duration::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JointTrajectoryPoint::generated_message_descriptor_data());
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
