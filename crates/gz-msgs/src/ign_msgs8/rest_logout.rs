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

//! Generated file from `ignition/msgs/rest_logout.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.RestLogout)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RestLogout {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.RestLogout.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.RestLogout.id)
    pub id: u32,
    // @@protoc_insertion_point(field:ignition.msgs.RestLogout.url)
    pub url: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.RestLogout.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RestLogout {
    fn default() -> &'a RestLogout {
        <RestLogout as ::protobuf::Message>::default_instance()
    }
}

impl RestLogout {
    pub fn new() -> RestLogout {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &RestLogout| { &m.header },
            |m: &mut RestLogout| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &RestLogout| { &m.id },
            |m: &mut RestLogout| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "url",
            |m: &RestLogout| { &m.url },
            |m: &mut RestLogout| { &mut m.url },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RestLogout>(
            "RestLogout",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RestLogout {
    const NAME: &'static str = "RestLogout";

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
                    self.url = is.read_string()?;
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
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.url);
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
        if !self.url.is_empty() {
            os.write_string(3, &self.url)?;
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

    fn new() -> RestLogout {
        RestLogout::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.id = 0;
        self.url.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RestLogout {
        static instance: RestLogout = RestLogout {
            header: ::protobuf::MessageField::none(),
            id: 0,
            url: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RestLogout {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RestLogout").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RestLogout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RestLogout {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fignition/msgs/rest_logout.proto\x12\rignition.msgs\x1a\x1aignition\
    /msgs/header.proto\"]\n\nRestLogout\x12-\n\x06header\x18\x01\x20\x01(\
    \x0b2\x15.ignition.msgs.HeaderR\x06header\x12\x0e\n\x02id\x18\x02\x20\
    \x01(\rR\x02id\x12\x10\n\x03url\x18\x03\x20\x01(\tR\x03urlB%\n\x11com.ig\
    nition.msgsB\x10RestLogoutProtosb\x06proto3\
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
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RestLogout::generated_message_descriptor_data());
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
