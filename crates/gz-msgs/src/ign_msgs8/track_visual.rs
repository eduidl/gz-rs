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

//! Generated file from `ignition/msgs/track_visual.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.TrackVisual)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrackVisual {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.id)
    pub id: u32,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.inherit_orientation)
    pub inherit_orientation: bool,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.min_dist)
    pub min_dist: f64,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.max_dist)
    pub max_dist: f64,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.static)
    pub static_: bool,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.use_model_frame)
    pub use_model_frame: bool,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.xyz)
    pub xyz: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:ignition.msgs.TrackVisual.inherit_yaw)
    pub inherit_yaw: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.TrackVisual.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrackVisual {
    fn default() -> &'a TrackVisual {
        <TrackVisual as ::protobuf::Message>::default_instance()
    }
}

impl TrackVisual {
    pub fn new() -> TrackVisual {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &TrackVisual| { &m.header },
            |m: &mut TrackVisual| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &TrackVisual| { &m.name },
            |m: &mut TrackVisual| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &TrackVisual| { &m.id },
            |m: &mut TrackVisual| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "inherit_orientation",
            |m: &TrackVisual| { &m.inherit_orientation },
            |m: &mut TrackVisual| { &mut m.inherit_orientation },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "min_dist",
            |m: &TrackVisual| { &m.min_dist },
            |m: &mut TrackVisual| { &mut m.min_dist },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_dist",
            |m: &TrackVisual| { &m.max_dist },
            |m: &mut TrackVisual| { &mut m.max_dist },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "static",
            |m: &TrackVisual| { &m.static_ },
            |m: &mut TrackVisual| { &mut m.static_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "use_model_frame",
            |m: &TrackVisual| { &m.use_model_frame },
            |m: &mut TrackVisual| { &mut m.use_model_frame },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "xyz",
            |m: &TrackVisual| { &m.xyz },
            |m: &mut TrackVisual| { &mut m.xyz },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "inherit_yaw",
            |m: &TrackVisual| { &m.inherit_yaw },
            |m: &mut TrackVisual| { &mut m.inherit_yaw },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrackVisual>(
            "TrackVisual",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrackVisual {
    const NAME: &'static str = "TrackVisual";

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
                    self.name = is.read_string()?;
                },
                24 => {
                    self.id = is.read_uint32()?;
                },
                32 => {
                    self.inherit_orientation = is.read_bool()?;
                },
                41 => {
                    self.min_dist = is.read_double()?;
                },
                49 => {
                    self.max_dist = is.read_double()?;
                },
                56 => {
                    self.static_ = is.read_bool()?;
                },
                64 => {
                    self.use_model_frame = is.read_bool()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.xyz)?;
                },
                80 => {
                    self.inherit_yaw = is.read_bool()?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.id);
        }
        if self.inherit_orientation != false {
            my_size += 1 + 1;
        }
        if self.min_dist != 0. {
            my_size += 1 + 8;
        }
        if self.max_dist != 0. {
            my_size += 1 + 8;
        }
        if self.static_ != false {
            my_size += 1 + 1;
        }
        if self.use_model_frame != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.xyz.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.inherit_yaw != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.id != 0 {
            os.write_uint32(3, self.id)?;
        }
        if self.inherit_orientation != false {
            os.write_bool(4, self.inherit_orientation)?;
        }
        if self.min_dist != 0. {
            os.write_double(5, self.min_dist)?;
        }
        if self.max_dist != 0. {
            os.write_double(6, self.max_dist)?;
        }
        if self.static_ != false {
            os.write_bool(7, self.static_)?;
        }
        if self.use_model_frame != false {
            os.write_bool(8, self.use_model_frame)?;
        }
        if let Some(v) = self.xyz.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.inherit_yaw != false {
            os.write_bool(10, self.inherit_yaw)?;
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

    fn new() -> TrackVisual {
        TrackVisual::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.name.clear();
        self.id = 0;
        self.inherit_orientation = false;
        self.min_dist = 0.;
        self.max_dist = 0.;
        self.static_ = false;
        self.use_model_frame = false;
        self.xyz.clear();
        self.inherit_yaw = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrackVisual {
        static instance: TrackVisual = TrackVisual {
            header: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            id: 0,
            inherit_orientation: false,
            min_dist: 0.,
            max_dist: 0.,
            static_: false,
            use_model_frame: false,
            xyz: ::protobuf::MessageField::none(),
            inherit_yaw: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrackVisual {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrackVisual").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrackVisual {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrackVisual {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20ignition/msgs/track_visual.proto\x12\rignition.msgs\x1a\x1cignitio\
    n/msgs/vector3d.proto\x1a\x1aignition/msgs/header.proto\"\xd3\x02\n\x0bT\
    rackVisual\x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.Head\
    erR\x06header\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x0e\n\
    \x02id\x18\x03\x20\x01(\rR\x02id\x12/\n\x13inherit_orientation\x18\x04\
    \x20\x01(\x08R\x12inheritOrientation\x12\x19\n\x08min_dist\x18\x05\x20\
    \x01(\x01R\x07minDist\x12\x19\n\x08max_dist\x18\x06\x20\x01(\x01R\x07max\
    Dist\x12\x16\n\x06static\x18\x07\x20\x01(\x08R\x06static\x12&\n\x0fuse_m\
    odel_frame\x18\x08\x20\x01(\x08R\ruseModelFrame\x12)\n\x03xyz\x18\t\x20\
    \x01(\x0b2\x17.ignition.msgs.Vector3dR\x03xyz\x12\x1f\n\x0binherit_yaw\
    \x18\n\x20\x01(\x08R\ninheritYawB&\n\x11com.ignition.msgsB\x11TrackVisua\
    lProtosb\x06proto3\
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
            deps.push(super::vector3d::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrackVisual::generated_message_descriptor_data());
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
