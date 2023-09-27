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

//! Generated file from `gz/msgs/clock.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Clock)
pub struct Clock {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Clock.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Clock.system)
    pub system: ::protobuf::MessageField<super::time::Time>,
    // @@protoc_insertion_point(field:gz.msgs.Clock.real)
    pub real: ::protobuf::MessageField<super::time::Time>,
    // @@protoc_insertion_point(field:gz.msgs.Clock.sim)
    pub sim: ::protobuf::MessageField<super::time::Time>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Clock.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Clock {
    fn default() -> &'a Clock {
        <Clock as ::protobuf::Message>::default_instance()
    }
}

impl Clock {
    pub fn new() -> Clock {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Clock| { &m.header },
            |m: &mut Clock| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::time::Time>(
            "system",
            |m: &Clock| { &m.system },
            |m: &mut Clock| { &mut m.system },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::time::Time>(
            "real",
            |m: &Clock| { &m.real },
            |m: &mut Clock| { &mut m.real },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::time::Time>(
            "sim",
            |m: &Clock| { &m.sim },
            |m: &mut Clock| { &mut m.sim },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Clock>(
            "Clock",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Clock {
    const NAME: &'static str = "Clock";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.system)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.real)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.sim)?;
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
        if let Some(v) = self.system.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.real.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.sim.as_ref() {
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
        if let Some(v) = self.system.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.real.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.sim.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> Clock {
        Clock::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.system.clear();
        self.real.clear();
        self.sim.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Clock {
        static instance: Clock = Clock {
            header: ::protobuf::MessageField::none(),
            system: ::protobuf::MessageField::none(),
            real: ::protobuf::MessageField::none(),
            sim: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Clock {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Clock").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Clock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Clock {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13gz/msgs/clock.proto\x12\x07gz.msgs\x1a\x12gz/msgs/time.proto\x1a\
    \x14gz/msgs/header.proto\"\x9b\x01\n\x05Clock\x12'\n\x06header\x18\x01\
    \x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12%\n\x06system\x18\x02\
    \x20\x01(\x0b2\r.gz.msgs.TimeR\x06system\x12!\n\x04real\x18\x03\x20\x01(\
    \x0b2\r.gz.msgs.TimeR\x04real\x12\x1f\n\x03sim\x18\x04\x20\x01(\x0b2\r.g\
    z.msgs.TimeR\x03simB\x1a\n\x0bcom.gz.msgsB\x0bClockProtosb\x06proto3\
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
            deps.push(super::time::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Clock::generated_message_descriptor_data());
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