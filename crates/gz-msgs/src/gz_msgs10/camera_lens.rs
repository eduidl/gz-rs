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

//! Generated file from `gz/msgs/camera_lens.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.CameraLens)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CameraLens {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.type)
    pub type_: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.c1)
    pub c1: f64,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.c2)
    pub c2: f64,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.c3)
    pub c3: f64,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.f)
    pub f: f64,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.fun)
    pub fun: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.scale_to_hfov)
    pub scale_to_hfov: bool,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.cutoff_angle)
    pub cutoff_angle: f64,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.hfov)
    pub hfov: f64,
    // @@protoc_insertion_point(field:gz.msgs.CameraLens.env_texture_size)
    pub env_texture_size: i32,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.CameraLens.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CameraLens {
    fn default() -> &'a CameraLens {
        <CameraLens as ::protobuf::Message>::default_instance()
    }
}

impl CameraLens {
    pub fn new() -> CameraLens {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &CameraLens| { &m.header },
            |m: &mut CameraLens| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &CameraLens| { &m.type_ },
            |m: &mut CameraLens| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "c1",
            |m: &CameraLens| { &m.c1 },
            |m: &mut CameraLens| { &mut m.c1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "c2",
            |m: &CameraLens| { &m.c2 },
            |m: &mut CameraLens| { &mut m.c2 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "c3",
            |m: &CameraLens| { &m.c3 },
            |m: &mut CameraLens| { &mut m.c3 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "f",
            |m: &CameraLens| { &m.f },
            |m: &mut CameraLens| { &mut m.f },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fun",
            |m: &CameraLens| { &m.fun },
            |m: &mut CameraLens| { &mut m.fun },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scale_to_hfov",
            |m: &CameraLens| { &m.scale_to_hfov },
            |m: &mut CameraLens| { &mut m.scale_to_hfov },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cutoff_angle",
            |m: &CameraLens| { &m.cutoff_angle },
            |m: &mut CameraLens| { &mut m.cutoff_angle },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hfov",
            |m: &CameraLens| { &m.hfov },
            |m: &mut CameraLens| { &mut m.hfov },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "env_texture_size",
            |m: &CameraLens| { &m.env_texture_size },
            |m: &mut CameraLens| { &mut m.env_texture_size },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CameraLens>(
            "CameraLens",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CameraLens {
    const NAME: &'static str = "CameraLens";

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
                    self.type_ = is.read_string()?;
                },
                25 => {
                    self.c1 = is.read_double()?;
                },
                33 => {
                    self.c2 = is.read_double()?;
                },
                41 => {
                    self.c3 = is.read_double()?;
                },
                49 => {
                    self.f = is.read_double()?;
                },
                58 => {
                    self.fun = is.read_string()?;
                },
                64 => {
                    self.scale_to_hfov = is.read_bool()?;
                },
                73 => {
                    self.cutoff_angle = is.read_double()?;
                },
                81 => {
                    self.hfov = is.read_double()?;
                },
                88 => {
                    self.env_texture_size = is.read_int32()?;
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
        if !self.type_.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.type_);
        }
        if self.c1 != 0. {
            my_size += 1 + 8;
        }
        if self.c2 != 0. {
            my_size += 1 + 8;
        }
        if self.c3 != 0. {
            my_size += 1 + 8;
        }
        if self.f != 0. {
            my_size += 1 + 8;
        }
        if !self.fun.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.fun);
        }
        if self.scale_to_hfov != false {
            my_size += 1 + 1;
        }
        if self.cutoff_angle != 0. {
            my_size += 1 + 8;
        }
        if self.hfov != 0. {
            my_size += 1 + 8;
        }
        if self.env_texture_size != 0 {
            my_size += ::protobuf::rt::int32_size(11, self.env_texture_size);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.type_.is_empty() {
            os.write_string(2, &self.type_)?;
        }
        if self.c1 != 0. {
            os.write_double(3, self.c1)?;
        }
        if self.c2 != 0. {
            os.write_double(4, self.c2)?;
        }
        if self.c3 != 0. {
            os.write_double(5, self.c3)?;
        }
        if self.f != 0. {
            os.write_double(6, self.f)?;
        }
        if !self.fun.is_empty() {
            os.write_string(7, &self.fun)?;
        }
        if self.scale_to_hfov != false {
            os.write_bool(8, self.scale_to_hfov)?;
        }
        if self.cutoff_angle != 0. {
            os.write_double(9, self.cutoff_angle)?;
        }
        if self.hfov != 0. {
            os.write_double(10, self.hfov)?;
        }
        if self.env_texture_size != 0 {
            os.write_int32(11, self.env_texture_size)?;
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

    fn new() -> CameraLens {
        CameraLens::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.type_.clear();
        self.c1 = 0.;
        self.c2 = 0.;
        self.c3 = 0.;
        self.f = 0.;
        self.fun.clear();
        self.scale_to_hfov = false;
        self.cutoff_angle = 0.;
        self.hfov = 0.;
        self.env_texture_size = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CameraLens {
        static instance: CameraLens = CameraLens {
            header: ::protobuf::MessageField::none(),
            type_: ::std::string::String::new(),
            c1: 0.,
            c2: 0.,
            c3: 0.,
            f: 0.,
            fun: ::std::string::String::new(),
            scale_to_hfov: false,
            cutoff_angle: 0.,
            hfov: 0.,
            env_texture_size: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CameraLens {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CameraLens").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CameraLens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CameraLens {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19gz/msgs/camera_lens.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.pro\
    to\"\x9e\x02\n\nCameraLens\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz\
    .msgs.HeaderR\x06header\x12\x12\n\x04type\x18\x02\x20\x01(\tR\x04type\
    \x12\x0e\n\x02c1\x18\x03\x20\x01(\x01R\x02c1\x12\x0e\n\x02c2\x18\x04\x20\
    \x01(\x01R\x02c2\x12\x0e\n\x02c3\x18\x05\x20\x01(\x01R\x02c3\x12\x0c\n\
    \x01f\x18\x06\x20\x01(\x01R\x01f\x12\x10\n\x03fun\x18\x07\x20\x01(\tR\
    \x03fun\x12\"\n\rscale_to_hfov\x18\x08\x20\x01(\x08R\x0bscaleToHfov\x12!\
    \n\x0ccutoff_angle\x18\t\x20\x01(\x01R\x0bcutoffAngle\x12\x12\n\x04hfov\
    \x18\n\x20\x01(\x01R\x04hfov\x12(\n\x10env_texture_size\x18\x0b\x20\x01(\
    \x05R\x0eenvTextureSizeB\x1f\n\x0bcom.gz.msgsB\x10CameraLensProtosb\x06p\
    roto3\
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
            messages.push(CameraLens::generated_message_descriptor_data());
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
