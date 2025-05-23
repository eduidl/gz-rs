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

//! Generated file from `gz/msgs/distortion.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Distortion)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Distortion {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Distortion.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Distortion.center)
    pub center: ::protobuf::MessageField<super::vector2d::Vector2d>,
    // @@protoc_insertion_point(field:gz.msgs.Distortion.k1)
    pub k1: f64,
    // @@protoc_insertion_point(field:gz.msgs.Distortion.k2)
    pub k2: f64,
    // @@protoc_insertion_point(field:gz.msgs.Distortion.k3)
    pub k3: f64,
    // @@protoc_insertion_point(field:gz.msgs.Distortion.p1)
    pub p1: f64,
    // @@protoc_insertion_point(field:gz.msgs.Distortion.p2)
    pub p2: f64,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Distortion.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Distortion {
    fn default() -> &'a Distortion {
        <Distortion as ::protobuf::Message>::default_instance()
    }
}

impl Distortion {
    pub fn new() -> Distortion {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Distortion| { &m.header },
            |m: &mut Distortion| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector2d::Vector2d>(
            "center",
            |m: &Distortion| { &m.center },
            |m: &mut Distortion| { &mut m.center },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "k1",
            |m: &Distortion| { &m.k1 },
            |m: &mut Distortion| { &mut m.k1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "k2",
            |m: &Distortion| { &m.k2 },
            |m: &mut Distortion| { &mut m.k2 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "k3",
            |m: &Distortion| { &m.k3 },
            |m: &mut Distortion| { &mut m.k3 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "p1",
            |m: &Distortion| { &m.p1 },
            |m: &mut Distortion| { &mut m.p1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "p2",
            |m: &Distortion| { &m.p2 },
            |m: &mut Distortion| { &mut m.p2 },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Distortion>(
            "Distortion",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Distortion {
    const NAME: &'static str = "Distortion";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.center)?;
                },
                25 => {
                    self.k1 = is.read_double()?;
                },
                33 => {
                    self.k2 = is.read_double()?;
                },
                41 => {
                    self.k3 = is.read_double()?;
                },
                49 => {
                    self.p1 = is.read_double()?;
                },
                57 => {
                    self.p2 = is.read_double()?;
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
        if let Some(v) = self.center.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.k1 != 0. {
            my_size += 1 + 8;
        }
        if self.k2 != 0. {
            my_size += 1 + 8;
        }
        if self.k3 != 0. {
            my_size += 1 + 8;
        }
        if self.p1 != 0. {
            my_size += 1 + 8;
        }
        if self.p2 != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.center.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.k1 != 0. {
            os.write_double(3, self.k1)?;
        }
        if self.k2 != 0. {
            os.write_double(4, self.k2)?;
        }
        if self.k3 != 0. {
            os.write_double(5, self.k3)?;
        }
        if self.p1 != 0. {
            os.write_double(6, self.p1)?;
        }
        if self.p2 != 0. {
            os.write_double(7, self.p2)?;
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

    fn new() -> Distortion {
        Distortion::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.center.clear();
        self.k1 = 0.;
        self.k2 = 0.;
        self.k3 = 0.;
        self.p1 = 0.;
        self.p2 = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Distortion {
        static instance: Distortion = Distortion {
            header: ::protobuf::MessageField::none(),
            center: ::protobuf::MessageField::none(),
            k1: 0.,
            k2: 0.,
            k3: 0.,
            p1: 0.,
            p2: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Distortion {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Distortion").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Distortion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Distortion {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18gz/msgs/distortion.proto\x12\x07gz.msgs\x1a\x16gz/msgs/vector2d.pr\
    oto\x1a\x14gz/msgs/header.proto\"\xb0\x01\n\nDistortion\x12'\n\x06header\
    \x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12)\n\x06center\
    \x18\x02\x20\x01(\x0b2\x11.gz.msgs.Vector2dR\x06center\x12\x0e\n\x02k1\
    \x18\x03\x20\x01(\x01R\x02k1\x12\x0e\n\x02k2\x18\x04\x20\x01(\x01R\x02k2\
    \x12\x0e\n\x02k3\x18\x05\x20\x01(\x01R\x02k3\x12\x0e\n\x02p1\x18\x06\x20\
    \x01(\x01R\x02p1\x12\x0e\n\x02p2\x18\x07\x20\x01(\x01R\x02p2B\x1f\n\x0bc\
    om.gz.msgsB\x10DistortionProtosb\x06proto3\
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
            deps.push(super::vector2d::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Distortion::generated_message_descriptor_data());
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
