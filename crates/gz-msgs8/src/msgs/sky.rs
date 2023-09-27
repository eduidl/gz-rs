// This file is generated by rust-protobuf 3.2.0. Do not edit
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

//! Generated file from `ignition/msgs/sky.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Sky)
pub struct Sky {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Sky.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.time)
    pub time: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.sunrise)
    pub sunrise: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.sunset)
    pub sunset: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.wind_speed)
    pub wind_speed: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.wind_direction)
    pub wind_direction: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.cloud_ambient)
    pub cloud_ambient: ::protobuf::MessageField<super::color::Color>,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.humidity)
    pub humidity: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Sky.mean_cloud_size)
    pub mean_cloud_size: f64,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Sky.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Sky {
    fn default() -> &'a Sky {
        <Sky as ::protobuf::Message>::default_instance()
    }
}

impl Sky {
    pub fn new() -> Sky {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Sky| { &m.header },
            |m: &mut Sky| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time",
            |m: &Sky| { &m.time },
            |m: &mut Sky| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sunrise",
            |m: &Sky| { &m.sunrise },
            |m: &mut Sky| { &mut m.sunrise },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sunset",
            |m: &Sky| { &m.sunset },
            |m: &mut Sky| { &mut m.sunset },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wind_speed",
            |m: &Sky| { &m.wind_speed },
            |m: &mut Sky| { &mut m.wind_speed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wind_direction",
            |m: &Sky| { &m.wind_direction },
            |m: &mut Sky| { &mut m.wind_direction },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::color::Color>(
            "cloud_ambient",
            |m: &Sky| { &m.cloud_ambient },
            |m: &mut Sky| { &mut m.cloud_ambient },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "humidity",
            |m: &Sky| { &m.humidity },
            |m: &mut Sky| { &mut m.humidity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mean_cloud_size",
            |m: &Sky| { &m.mean_cloud_size },
            |m: &mut Sky| { &mut m.mean_cloud_size },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Sky>(
            "Sky",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Sky {
    const NAME: &'static str = "Sky";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                17 => {
                    self.time = is.read_double()?;
                },
                25 => {
                    self.sunrise = is.read_double()?;
                },
                33 => {
                    self.sunset = is.read_double()?;
                },
                41 => {
                    self.wind_speed = is.read_double()?;
                },
                49 => {
                    self.wind_direction = is.read_double()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cloud_ambient)?;
                },
                65 => {
                    self.humidity = is.read_double()?;
                },
                73 => {
                    self.mean_cloud_size = is.read_double()?;
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
        if self.time != 0. {
            my_size += 1 + 8;
        }
        if self.sunrise != 0. {
            my_size += 1 + 8;
        }
        if self.sunset != 0. {
            my_size += 1 + 8;
        }
        if self.wind_speed != 0. {
            my_size += 1 + 8;
        }
        if self.wind_direction != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.cloud_ambient.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.humidity != 0. {
            my_size += 1 + 8;
        }
        if self.mean_cloud_size != 0. {
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
        if self.time != 0. {
            os.write_double(2, self.time)?;
        }
        if self.sunrise != 0. {
            os.write_double(3, self.sunrise)?;
        }
        if self.sunset != 0. {
            os.write_double(4, self.sunset)?;
        }
        if self.wind_speed != 0. {
            os.write_double(5, self.wind_speed)?;
        }
        if self.wind_direction != 0. {
            os.write_double(6, self.wind_direction)?;
        }
        if let Some(v) = self.cloud_ambient.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.humidity != 0. {
            os.write_double(8, self.humidity)?;
        }
        if self.mean_cloud_size != 0. {
            os.write_double(9, self.mean_cloud_size)?;
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

    fn new() -> Sky {
        Sky::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.time = 0.;
        self.sunrise = 0.;
        self.sunset = 0.;
        self.wind_speed = 0.;
        self.wind_direction = 0.;
        self.cloud_ambient.clear();
        self.humidity = 0.;
        self.mean_cloud_size = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Sky {
        static instance: Sky = Sky {
            header: ::protobuf::MessageField::none(),
            time: 0.,
            sunrise: 0.,
            sunset: 0.,
            wind_speed: 0.,
            wind_direction: 0.,
            cloud_ambient: ::protobuf::MessageField::none(),
            humidity: 0.,
            mean_cloud_size: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Sky {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Sky").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Sky {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Sky {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ignition/msgs/sky.proto\x12\rignition.msgs\x1a\x19ignition/msgs/co\
    lor.proto\x1a\x1aignition/msgs/header.proto\"\xbf\x02\n\x03Sky\x12-\n\
    \x06header\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x12\
    \x12\n\x04time\x18\x02\x20\x01(\x01R\x04time\x12\x18\n\x07sunrise\x18\
    \x03\x20\x01(\x01R\x07sunrise\x12\x16\n\x06sunset\x18\x04\x20\x01(\x01R\
    \x06sunset\x12\x1d\n\nwind_speed\x18\x05\x20\x01(\x01R\twindSpeed\x12%\n\
    \x0ewind_direction\x18\x06\x20\x01(\x01R\rwindDirection\x129\n\rcloud_am\
    bient\x18\x07\x20\x01(\x0b2\x14.ignition.msgs.ColorR\x0ccloudAmbient\x12\
    \x1a\n\x08humidity\x18\x08\x20\x01(\x01R\x08humidity\x12&\n\x0fmean_clou\
    d_size\x18\t\x20\x01(\x01R\rmeanCloudSizeB\x1e\n\x11com.ignition.msgsB\t\
    SkyProtosb\x06proto3\
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
            deps.push(super::color::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Sky::generated_message_descriptor_data());
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