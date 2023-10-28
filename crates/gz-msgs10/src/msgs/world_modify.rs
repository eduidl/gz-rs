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

//! Generated file from `gz/msgs/world_modify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.WorldModify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct WorldModify {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.WorldModify.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.WorldModify.world_name)
    pub world_name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.WorldModify.remove)
    pub remove: bool,
    // @@protoc_insertion_point(field:gz.msgs.WorldModify.create)
    pub create: bool,
    // @@protoc_insertion_point(field:gz.msgs.WorldModify.cloned)
    pub cloned: bool,
    // @@protoc_insertion_point(field:gz.msgs.WorldModify.cloned_uri)
    pub cloned_uri: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.WorldModify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WorldModify {
    fn default() -> &'a WorldModify {
        <WorldModify as ::protobuf::Message>::default_instance()
    }
}

impl WorldModify {
    pub fn new() -> WorldModify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &WorldModify| { &m.header },
            |m: &mut WorldModify| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_name",
            |m: &WorldModify| { &m.world_name },
            |m: &mut WorldModify| { &mut m.world_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "remove",
            |m: &WorldModify| { &m.remove },
            |m: &mut WorldModify| { &mut m.remove },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "create",
            |m: &WorldModify| { &m.create },
            |m: &mut WorldModify| { &mut m.create },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cloned",
            |m: &WorldModify| { &m.cloned },
            |m: &mut WorldModify| { &mut m.cloned },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cloned_uri",
            |m: &WorldModify| { &m.cloned_uri },
            |m: &mut WorldModify| { &mut m.cloned_uri },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WorldModify>(
            "WorldModify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WorldModify {
    const NAME: &'static str = "WorldModify";

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
                    self.world_name = is.read_string()?;
                },
                24 => {
                    self.remove = is.read_bool()?;
                },
                32 => {
                    self.create = is.read_bool()?;
                },
                40 => {
                    self.cloned = is.read_bool()?;
                },
                50 => {
                    self.cloned_uri = is.read_string()?;
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
        if !self.world_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.world_name);
        }
        if self.remove != false {
            my_size += 1 + 1;
        }
        if self.create != false {
            my_size += 1 + 1;
        }
        if self.cloned != false {
            my_size += 1 + 1;
        }
        if !self.cloned_uri.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.cloned_uri);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.world_name.is_empty() {
            os.write_string(2, &self.world_name)?;
        }
        if self.remove != false {
            os.write_bool(3, self.remove)?;
        }
        if self.create != false {
            os.write_bool(4, self.create)?;
        }
        if self.cloned != false {
            os.write_bool(5, self.cloned)?;
        }
        if !self.cloned_uri.is_empty() {
            os.write_string(6, &self.cloned_uri)?;
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

    fn new() -> WorldModify {
        WorldModify::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.world_name.clear();
        self.remove = false;
        self.create = false;
        self.cloned = false;
        self.cloned_uri.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WorldModify {
        static instance: WorldModify = WorldModify {
            header: ::protobuf::MessageField::none(),
            world_name: ::std::string::String::new(),
            remove: false,
            create: false,
            cloned: false,
            cloned_uri: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WorldModify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WorldModify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WorldModify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WorldModify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1agz/msgs/world_modify.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.pr\
    oto\"\xbc\x01\n\x0bWorldModify\x12'\n\x06header\x18\x01\x20\x01(\x0b2\
    \x0f.gz.msgs.HeaderR\x06header\x12\x1d\n\nworld_name\x18\x02\x20\x01(\tR\
    \tworldName\x12\x16\n\x06remove\x18\x03\x20\x01(\x08R\x06remove\x12\x16\
    \n\x06create\x18\x04\x20\x01(\x08R\x06create\x12\x16\n\x06cloned\x18\x05\
    \x20\x01(\x08R\x06cloned\x12\x1d\n\ncloned_uri\x18\x06\x20\x01(\tR\tclon\
    edUriB\x20\n\x0bcom.gz.msgsB\x11WorldModifyProtosb\x06proto3\
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
            messages.push(WorldModify::generated_message_descriptor_data());
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
