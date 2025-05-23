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

//! Generated file from `gz/msgs/imagegeom.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.ImageGeom)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ImageGeom {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.ImageGeom.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.ImageGeom.uri)
    pub uri: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.ImageGeom.scale)
    pub scale: f64,
    // @@protoc_insertion_point(field:gz.msgs.ImageGeom.threshold)
    pub threshold: i32,
    // @@protoc_insertion_point(field:gz.msgs.ImageGeom.height)
    pub height: f64,
    // @@protoc_insertion_point(field:gz.msgs.ImageGeom.granularity)
    pub granularity: i32,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.ImageGeom.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ImageGeom {
    fn default() -> &'a ImageGeom {
        <ImageGeom as ::protobuf::Message>::default_instance()
    }
}

impl ImageGeom {
    pub fn new() -> ImageGeom {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &ImageGeom| { &m.header },
            |m: &mut ImageGeom| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uri",
            |m: &ImageGeom| { &m.uri },
            |m: &mut ImageGeom| { &mut m.uri },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scale",
            |m: &ImageGeom| { &m.scale },
            |m: &mut ImageGeom| { &mut m.scale },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "threshold",
            |m: &ImageGeom| { &m.threshold },
            |m: &mut ImageGeom| { &mut m.threshold },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "height",
            |m: &ImageGeom| { &m.height },
            |m: &mut ImageGeom| { &mut m.height },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "granularity",
            |m: &ImageGeom| { &m.granularity },
            |m: &mut ImageGeom| { &mut m.granularity },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ImageGeom>(
            "ImageGeom",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ImageGeom {
    const NAME: &'static str = "ImageGeom";

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
                    self.uri = is.read_string()?;
                },
                25 => {
                    self.scale = is.read_double()?;
                },
                32 => {
                    self.threshold = is.read_int32()?;
                },
                41 => {
                    self.height = is.read_double()?;
                },
                48 => {
                    self.granularity = is.read_int32()?;
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
        if !self.uri.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.uri);
        }
        if self.scale != 0. {
            my_size += 1 + 8;
        }
        if self.threshold != 0 {
            my_size += ::protobuf::rt::int32_size(4, self.threshold);
        }
        if self.height != 0. {
            my_size += 1 + 8;
        }
        if self.granularity != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.granularity);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.uri.is_empty() {
            os.write_string(2, &self.uri)?;
        }
        if self.scale != 0. {
            os.write_double(3, self.scale)?;
        }
        if self.threshold != 0 {
            os.write_int32(4, self.threshold)?;
        }
        if self.height != 0. {
            os.write_double(5, self.height)?;
        }
        if self.granularity != 0 {
            os.write_int32(6, self.granularity)?;
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

    fn new() -> ImageGeom {
        ImageGeom::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.uri.clear();
        self.scale = 0.;
        self.threshold = 0;
        self.height = 0.;
        self.granularity = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ImageGeom {
        static instance: ImageGeom = ImageGeom {
            header: ::protobuf::MessageField::none(),
            uri: ::std::string::String::new(),
            scale: 0.,
            threshold: 0,
            height: 0.,
            granularity: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ImageGeom {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ImageGeom").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ImageGeom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageGeom {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17gz/msgs/imagegeom.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.proto\
    \"\xb4\x01\n\tImageGeom\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.ms\
    gs.HeaderR\x06header\x12\x10\n\x03uri\x18\x02\x20\x01(\tR\x03uri\x12\x14\
    \n\x05scale\x18\x03\x20\x01(\x01R\x05scale\x12\x1c\n\tthreshold\x18\x04\
    \x20\x01(\x05R\tthreshold\x12\x16\n\x06height\x18\x05\x20\x01(\x01R\x06h\
    eight\x12\x20\n\x0bgranularity\x18\x06\x20\x01(\x05R\x0bgranularityB\x1e\
    \n\x0bcom.gz.msgsB\x0fImageGeomProtosb\x06proto3\
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
            messages.push(ImageGeom::generated_message_descriptor_data());
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
