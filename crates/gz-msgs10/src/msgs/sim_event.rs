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

//! Generated file from `gz/msgs/sim_event.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.SimEvent)
pub struct SimEvent {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.SimEvent.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.SimEvent.id)
    pub id: u32,
    // @@protoc_insertion_point(field:gz.msgs.SimEvent.type)
    pub type_: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.SimEvent.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.SimEvent.world_statistics)
    pub world_statistics: ::protobuf::MessageField<super::world_stats::WorldStatistics>,
    // @@protoc_insertion_point(field:gz.msgs.SimEvent.data)
    pub data: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.SimEvent.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SimEvent {
    fn default() -> &'a SimEvent {
        <SimEvent as ::protobuf::Message>::default_instance()
    }
}

impl SimEvent {
    pub fn new() -> SimEvent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &SimEvent| { &m.header },
            |m: &mut SimEvent| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &SimEvent| { &m.id },
            |m: &mut SimEvent| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &SimEvent| { &m.type_ },
            |m: &mut SimEvent| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &SimEvent| { &m.name },
            |m: &mut SimEvent| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::world_stats::WorldStatistics>(
            "world_statistics",
            |m: &SimEvent| { &m.world_statistics },
            |m: &mut SimEvent| { &mut m.world_statistics },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &SimEvent| { &m.data },
            |m: &mut SimEvent| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SimEvent>(
            "SimEvent",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SimEvent {
    const NAME: &'static str = "SimEvent";

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
                    self.id = is.read_uint32()?;
                },
                26 => {
                    self.type_ = is.read_string()?;
                },
                34 => {
                    self.name = is.read_string()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.world_statistics)?;
                },
                50 => {
                    self.data = is.read_string()?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.id);
        }
        if !self.type_.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.type_);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.name);
        }
        if let Some(v) = self.world_statistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.id != 0 {
            os.write_uint32(2, self.id)?;
        }
        if !self.type_.is_empty() {
            os.write_string(3, &self.type_)?;
        }
        if !self.name.is_empty() {
            os.write_string(4, &self.name)?;
        }
        if let Some(v) = self.world_statistics.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if !self.data.is_empty() {
            os.write_string(6, &self.data)?;
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

    fn new() -> SimEvent {
        SimEvent::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.id = 0;
        self.type_.clear();
        self.name.clear();
        self.world_statistics.clear();
        self.data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SimEvent {
        static instance: SimEvent = SimEvent {
            header: ::protobuf::MessageField::none(),
            id: 0,
            type_: ::std::string::String::new(),
            name: ::std::string::String::new(),
            world_statistics: ::protobuf::MessageField::none(),
            data: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SimEvent {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SimEvent").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SimEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SimEvent {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17gz/msgs/sim_event.proto\x12\x07gz.msgs\x1a\x19gz/msgs/world_stats.\
    proto\x1a\x14gz/msgs/header.proto\"\xc4\x01\n\x08SimEvent\x12'\n\x06head\
    er\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12\x0e\n\x02id\
    \x18\x02\x20\x01(\rR\x02id\x12\x12\n\x04type\x18\x03\x20\x01(\tR\x04type\
    \x12\x12\n\x04name\x18\x04\x20\x01(\tR\x04name\x12C\n\x10world_statistic\
    s\x18\x05\x20\x01(\x0b2\x18.gz.msgs.WorldStatisticsR\x0fworldStatistics\
    \x12\x12\n\x04data\x18\x06\x20\x01(\tR\x04dataB\x1d\n\x0bcom.gz.msgsB\
    \x0eSimEventProtosb\x06proto3\
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
            deps.push(super::world_stats::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SimEvent::generated_message_descriptor_data());
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