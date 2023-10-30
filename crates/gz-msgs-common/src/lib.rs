#![doc = include_str!("../README.md")]

pub use protobuf;
use protobuf::Message;

pub trait GzMessage: Message {
    const GZ_TYPE_NAME: &'static str;
}

pub use gz_msgs_derive::{GzMessage, IgnMessage};
