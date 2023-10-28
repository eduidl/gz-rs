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

//! Generated file from `ignition/msgs/axis.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Axis)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Axis {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Axis.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.xyz)
    pub xyz: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.limit_lower)
    pub limit_lower: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.limit_upper)
    pub limit_upper: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.limit_effort)
    pub limit_effort: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.limit_velocity)
    pub limit_velocity: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.damping)
    pub damping: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.friction)
    pub friction: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.position)
    pub position: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.velocity)
    pub velocity: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.force)
    pub force: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.acceleration)
    pub acceleration: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Axis.xyz_expressed_in)
    pub xyz_expressed_in: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Axis.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Axis {
    fn default() -> &'a Axis {
        <Axis as ::protobuf::Message>::default_instance()
    }
}

impl Axis {
    pub fn new() -> Axis {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Axis| { &m.header },
            |m: &mut Axis| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "xyz",
            |m: &Axis| { &m.xyz },
            |m: &mut Axis| { &mut m.xyz },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "limit_lower",
            |m: &Axis| { &m.limit_lower },
            |m: &mut Axis| { &mut m.limit_lower },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "limit_upper",
            |m: &Axis| { &m.limit_upper },
            |m: &mut Axis| { &mut m.limit_upper },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "limit_effort",
            |m: &Axis| { &m.limit_effort },
            |m: &mut Axis| { &mut m.limit_effort },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "limit_velocity",
            |m: &Axis| { &m.limit_velocity },
            |m: &mut Axis| { &mut m.limit_velocity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "damping",
            |m: &Axis| { &m.damping },
            |m: &mut Axis| { &mut m.damping },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "friction",
            |m: &Axis| { &m.friction },
            |m: &mut Axis| { &mut m.friction },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "position",
            |m: &Axis| { &m.position },
            |m: &mut Axis| { &mut m.position },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "velocity",
            |m: &Axis| { &m.velocity },
            |m: &mut Axis| { &mut m.velocity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "force",
            |m: &Axis| { &m.force },
            |m: &mut Axis| { &mut m.force },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "acceleration",
            |m: &Axis| { &m.acceleration },
            |m: &mut Axis| { &mut m.acceleration },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "xyz_expressed_in",
            |m: &Axis| { &m.xyz_expressed_in },
            |m: &mut Axis| { &mut m.xyz_expressed_in },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Axis>(
            "Axis",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Axis {
    const NAME: &'static str = "Axis";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.xyz)?;
                },
                25 => {
                    self.limit_lower = is.read_double()?;
                },
                33 => {
                    self.limit_upper = is.read_double()?;
                },
                41 => {
                    self.limit_effort = is.read_double()?;
                },
                49 => {
                    self.limit_velocity = is.read_double()?;
                },
                57 => {
                    self.damping = is.read_double()?;
                },
                65 => {
                    self.friction = is.read_double()?;
                },
                81 => {
                    self.position = is.read_double()?;
                },
                89 => {
                    self.velocity = is.read_double()?;
                },
                97 => {
                    self.force = is.read_double()?;
                },
                105 => {
                    self.acceleration = is.read_double()?;
                },
                114 => {
                    self.xyz_expressed_in = is.read_string()?;
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
        if let Some(v) = self.xyz.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.limit_lower != 0. {
            my_size += 1 + 8;
        }
        if self.limit_upper != 0. {
            my_size += 1 + 8;
        }
        if self.limit_effort != 0. {
            my_size += 1 + 8;
        }
        if self.limit_velocity != 0. {
            my_size += 1 + 8;
        }
        if self.damping != 0. {
            my_size += 1 + 8;
        }
        if self.friction != 0. {
            my_size += 1 + 8;
        }
        if self.position != 0. {
            my_size += 1 + 8;
        }
        if self.velocity != 0. {
            my_size += 1 + 8;
        }
        if self.force != 0. {
            my_size += 1 + 8;
        }
        if self.acceleration != 0. {
            my_size += 1 + 8;
        }
        if !self.xyz_expressed_in.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.xyz_expressed_in);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.xyz.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.limit_lower != 0. {
            os.write_double(3, self.limit_lower)?;
        }
        if self.limit_upper != 0. {
            os.write_double(4, self.limit_upper)?;
        }
        if self.limit_effort != 0. {
            os.write_double(5, self.limit_effort)?;
        }
        if self.limit_velocity != 0. {
            os.write_double(6, self.limit_velocity)?;
        }
        if self.damping != 0. {
            os.write_double(7, self.damping)?;
        }
        if self.friction != 0. {
            os.write_double(8, self.friction)?;
        }
        if self.position != 0. {
            os.write_double(10, self.position)?;
        }
        if self.velocity != 0. {
            os.write_double(11, self.velocity)?;
        }
        if self.force != 0. {
            os.write_double(12, self.force)?;
        }
        if self.acceleration != 0. {
            os.write_double(13, self.acceleration)?;
        }
        if !self.xyz_expressed_in.is_empty() {
            os.write_string(14, &self.xyz_expressed_in)?;
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

    fn new() -> Axis {
        Axis::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.xyz.clear();
        self.limit_lower = 0.;
        self.limit_upper = 0.;
        self.limit_effort = 0.;
        self.limit_velocity = 0.;
        self.damping = 0.;
        self.friction = 0.;
        self.position = 0.;
        self.velocity = 0.;
        self.force = 0.;
        self.acceleration = 0.;
        self.xyz_expressed_in.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Axis {
        static instance: Axis = Axis {
            header: ::protobuf::MessageField::none(),
            xyz: ::protobuf::MessageField::none(),
            limit_lower: 0.,
            limit_upper: 0.,
            limit_effort: 0.,
            limit_velocity: 0.,
            damping: 0.,
            friction: 0.,
            position: 0.,
            velocity: 0.,
            force: 0.,
            acceleration: 0.,
            xyz_expressed_in: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Axis {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Axis").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Axis {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Axis {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18ignition/msgs/axis.proto\x12\rignition.msgs\x1a\x1cignition/msgs/v\
    ector3d.proto\x1a\x1aignition/msgs/header.proto\"\xbe\x03\n\x04Axis\x12-\
    \n\x06header\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\
    \x12)\n\x03xyz\x18\x02\x20\x01(\x0b2\x17.ignition.msgs.Vector3dR\x03xyz\
    \x12\x1f\n\x0blimit_lower\x18\x03\x20\x01(\x01R\nlimitLower\x12\x1f\n\
    \x0blimit_upper\x18\x04\x20\x01(\x01R\nlimitUpper\x12!\n\x0climit_effort\
    \x18\x05\x20\x01(\x01R\x0blimitEffort\x12%\n\x0elimit_velocity\x18\x06\
    \x20\x01(\x01R\rlimitVelocity\x12\x18\n\x07damping\x18\x07\x20\x01(\x01R\
    \x07damping\x12\x1a\n\x08friction\x18\x08\x20\x01(\x01R\x08friction\x12\
    \x1a\n\x08position\x18\n\x20\x01(\x01R\x08position\x12\x1a\n\x08velocity\
    \x18\x0b\x20\x01(\x01R\x08velocity\x12\x14\n\x05force\x18\x0c\x20\x01(\
    \x01R\x05force\x12\"\n\x0cacceleration\x18\r\x20\x01(\x01R\x0caccelerati\
    on\x12(\n\x10xyz_expressed_in\x18\x0e\x20\x01(\tR\x0exyzExpressedInB\x1f\
    \n\x11com.ignition.msgsB\nAxisProtosb\x06proto3\
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
            messages.push(Axis::generated_message_descriptor_data());
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
