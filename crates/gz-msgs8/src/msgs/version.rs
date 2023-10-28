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

//! Generated file from `ignition/msgs/version.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Version)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Version {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Version.major)
    pub major: i32,
    // @@protoc_insertion_point(field:ignition.msgs.Version.minor)
    pub minor: i32,
    // @@protoc_insertion_point(field:ignition.msgs.Version.patch)
    pub patch: i32,
    // @@protoc_insertion_point(field:ignition.msgs.Version.prerelease)
    pub prerelease: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Version.build)
    pub build: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Version.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Version {
    fn default() -> &'a Version {
        <Version as ::protobuf::Message>::default_instance()
    }
}

impl Version {
    pub fn new() -> Version {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "major",
            |m: &Version| { &m.major },
            |m: &mut Version| { &mut m.major },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "minor",
            |m: &Version| { &m.minor },
            |m: &mut Version| { &mut m.minor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "patch",
            |m: &Version| { &m.patch },
            |m: &mut Version| { &mut m.patch },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prerelease",
            |m: &Version| { &m.prerelease },
            |m: &mut Version| { &mut m.prerelease },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "build",
            |m: &Version| { &m.build },
            |m: &mut Version| { &mut m.build },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Version>(
            "Version",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Version {
    const NAME: &'static str = "Version";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.major = is.read_int32()?;
                },
                16 => {
                    self.minor = is.read_int32()?;
                },
                24 => {
                    self.patch = is.read_int32()?;
                },
                34 => {
                    self.prerelease = is.read_string()?;
                },
                42 => {
                    self.build = is.read_string()?;
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
        if self.major != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.major);
        }
        if self.minor != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.minor);
        }
        if self.patch != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.patch);
        }
        if !self.prerelease.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.prerelease);
        }
        if !self.build.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.build);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.major != 0 {
            os.write_int32(1, self.major)?;
        }
        if self.minor != 0 {
            os.write_int32(2, self.minor)?;
        }
        if self.patch != 0 {
            os.write_int32(3, self.patch)?;
        }
        if !self.prerelease.is_empty() {
            os.write_string(4, &self.prerelease)?;
        }
        if !self.build.is_empty() {
            os.write_string(5, &self.build)?;
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

    fn new() -> Version {
        Version::new()
    }

    fn clear(&mut self) {
        self.major = 0;
        self.minor = 0;
        self.patch = 0;
        self.prerelease.clear();
        self.build.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Version {
        static instance: Version = Version {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: ::std::string::String::new(),
            build: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Version {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Version").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Version {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Version {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bignition/msgs/version.proto\x12\rignition.msgs\"\x81\x01\n\x07Vers\
    ion\x12\x14\n\x05major\x18\x01\x20\x01(\x05R\x05major\x12\x14\n\x05minor\
    \x18\x02\x20\x01(\x05R\x05minor\x12\x14\n\x05patch\x18\x03\x20\x01(\x05R\
    \x05patch\x12\x1e\n\nprerelease\x18\x04\x20\x01(\tR\nprerelease\x12\x14\
    \n\x05build\x18\x05\x20\x01(\tR\x05buildB\x1c\n\x11com.ignition.msgsB\
    \x07Versionb\x06proto3\
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
            messages.push(Version::generated_message_descriptor_data());
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
