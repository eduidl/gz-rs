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

//! Generated file from `gz/msgs/annotated_oriented_3d_box_v.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.AnnotatedOriented3DBox_V)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AnnotatedOriented3DBox_V {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.AnnotatedOriented3DBox_V.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.AnnotatedOriented3DBox_V.annotated_box)
    pub annotated_box: ::std::vec::Vec<super::annotated_oriented_3d_box::AnnotatedOriented3DBox>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.AnnotatedOriented3DBox_V.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AnnotatedOriented3DBox_V {
    fn default() -> &'a AnnotatedOriented3DBox_V {
        <AnnotatedOriented3DBox_V as ::protobuf::Message>::default_instance()
    }
}

impl AnnotatedOriented3DBox_V {
    pub fn new() -> AnnotatedOriented3DBox_V {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &AnnotatedOriented3DBox_V| { &m.header },
            |m: &mut AnnotatedOriented3DBox_V| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "annotated_box",
            |m: &AnnotatedOriented3DBox_V| { &m.annotated_box },
            |m: &mut AnnotatedOriented3DBox_V| { &mut m.annotated_box },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AnnotatedOriented3DBox_V>(
            "AnnotatedOriented3DBox_V",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AnnotatedOriented3DBox_V {
    const NAME: &'static str = "AnnotatedOriented3DBox_V";

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
                    self.annotated_box.push(is.read_message()?);
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
        for value in &self.annotated_box {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        for v in &self.annotated_box {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> AnnotatedOriented3DBox_V {
        AnnotatedOriented3DBox_V::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.annotated_box.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AnnotatedOriented3DBox_V {
        static instance: AnnotatedOriented3DBox_V = AnnotatedOriented3DBox_V {
            header: ::protobuf::MessageField::none(),
            annotated_box: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AnnotatedOriented3DBox_V {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AnnotatedOriented3DBox_V").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AnnotatedOriented3DBox_V {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AnnotatedOriented3DBox_V {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)gz/msgs/annotated_oriented_3d_box_v.proto\x12\x07gz.msgs\x1a\x14gz/ms\
    gs/header.proto\x1a'gz/msgs/annotated_oriented_3d_box.proto\"\x89\x01\n\
    \x18AnnotatedOriented3DBox_V\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.\
    gz.msgs.HeaderR\x06header\x12D\n\rannotated_box\x18\x02\x20\x03(\x0b2\
    \x1f.gz.msgs.AnnotatedOriented3DBoxR\x0cannotatedBoxB-\n\x0bcom.gz.msgsB\
    \x1eAnnotatedOriented3DBox_VProtosb\x06proto3\
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
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::annotated_oriented_3d_box::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AnnotatedOriented3DBox_V::generated_message_descriptor_data());
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
