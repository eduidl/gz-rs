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

//! Generated file from `gz/msgs/gui_camera.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.GUICamera)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GUICamera {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.GUICamera.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.GUICamera.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.GUICamera.view_controller)
    pub view_controller: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.GUICamera.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:gz.msgs.GUICamera.track)
    pub track: ::protobuf::MessageField<super::track_visual::TrackVisual>,
    // @@protoc_insertion_point(field:gz.msgs.GUICamera.projection_type)
    pub projection_type: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.GUICamera.duration)
    pub duration: f64,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.GUICamera.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GUICamera {
    fn default() -> &'a GUICamera {
        <GUICamera as ::protobuf::Message>::default_instance()
    }
}

impl GUICamera {
    pub fn new() -> GUICamera {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &GUICamera| { &m.header },
            |m: &mut GUICamera| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &GUICamera| { &m.name },
            |m: &mut GUICamera| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "view_controller",
            |m: &GUICamera| { &m.view_controller },
            |m: &mut GUICamera| { &mut m.view_controller },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &GUICamera| { &m.pose },
            |m: &mut GUICamera| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::track_visual::TrackVisual>(
            "track",
            |m: &GUICamera| { &m.track },
            |m: &mut GUICamera| { &mut m.track },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "projection_type",
            |m: &GUICamera| { &m.projection_type },
            |m: &mut GUICamera| { &mut m.projection_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "duration",
            |m: &GUICamera| { &m.duration },
            |m: &mut GUICamera| { &mut m.duration },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GUICamera>(
            "GUICamera",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GUICamera {
    const NAME: &'static str = "GUICamera";

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
                26 => {
                    self.view_controller = is.read_string()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.track)?;
                },
                50 => {
                    self.projection_type = is.read_string()?;
                },
                57 => {
                    self.duration = is.read_double()?;
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
        if !self.view_controller.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.view_controller);
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.track.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.projection_type.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.projection_type);
        }
        if self.duration != 0. {
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.view_controller.is_empty() {
            os.write_string(3, &self.view_controller)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.track.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if !self.projection_type.is_empty() {
            os.write_string(6, &self.projection_type)?;
        }
        if self.duration != 0. {
            os.write_double(7, self.duration)?;
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

    fn new() -> GUICamera {
        GUICamera::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.name.clear();
        self.view_controller.clear();
        self.pose.clear();
        self.track.clear();
        self.projection_type.clear();
        self.duration = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GUICamera {
        static instance: GUICamera = GUICamera {
            header: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            view_controller: ::std::string::String::new(),
            pose: ::protobuf::MessageField::none(),
            track: ::protobuf::MessageField::none(),
            projection_type: ::std::string::String::new(),
            duration: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GUICamera {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GUICamera").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GUICamera {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GUICamera {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18gz/msgs/gui_camera.proto\x12\x07gz.msgs\x1a\x12gz/msgs/pose.proto\
    \x1a\x1agz/msgs/track_visual.proto\x1a\x14gz/msgs/header.proto\"\x85\x02\
    \n\tGUICamera\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\
    \x06header\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12'\n\x0fview_\
    controller\x18\x03\x20\x01(\tR\x0eviewController\x12!\n\x04pose\x18\x04\
    \x20\x01(\x0b2\r.gz.msgs.PoseR\x04pose\x12*\n\x05track\x18\x05\x20\x01(\
    \x0b2\x14.gz.msgs.TrackVisualR\x05track\x12'\n\x0fprojection_type\x18\
    \x06\x20\x01(\tR\x0eprojectionType\x12\x1a\n\x08duration\x18\x07\x20\x01\
    (\x01R\x08durationB\x1e\n\x0bcom.gz.msgsB\x0fGUICameraProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::track_visual::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GUICamera::generated_message_descriptor_data());
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
