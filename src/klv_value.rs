//! Information from this page was gathered from page 32 of the MISB ST 0601.19 document that was
//! published 2023-March-02.
use std::sync::Arc;

use bitvec::{field::BitField, order::Msb0, view::BitView};

use crate::{klv_packet::KlvPacket, tag::Tag, Errors};

/// The value types that are supported to be stored in a UAS Datalink KLV packet.
/// The first value is always the tag number. The second value is the value.
#[derive(Clone, Debug)]
pub enum KlvValue {
    /// Variable length, 2's complement signed integer
    /// 
    /// Storing this as an i64 for now but this may need to be some form of BigInt or whatever that
    /// is in Rust. Pretty sure that's what they refer to them as in Python.
    Int(i64),
    /// 8-bit, 2's complement signed integer
    Int8(i8),
    /// 16-bit, 2's complement signed integer
    Int16(i16),
    /// 32-bit, 2's complement signed integer
    Int32(i32),
    /// Variable length unsigned integer
    /// 
    /// Storing this as an u64 for now but this may need to be some form of BigInt or whatever that
    /// is in Rust. Pretty sure that's what they refer to them as in Python.
    Uint(u64),
    /// 8-bit, unsigned integer – i.e., single byte
    Uint8(u8),
    /// 16-bit unsigned short
    Uint16(u16),
    /// 32-bit unsigned integer
    Uint32(u32),
    /// 64-bit unsigned long
    Uint64(u64),
    /// Mapping using the IMAPB method (see MISB ST 1201 [12])
    /// 
    /// This map seems to map to a float in the conversions. It's a variable length float from what
    /// I can tell in the documentation.
    /// 
    /// NOTE: This is not supported yet.
    IMAPB(f64),
    /// One or more bytes which represent a binary value
    /// 
    /// Typically used for bit-flags.
    Byte(u8),
    /// Defined length pack
    /// 
    /// TODO: Look into what this enum's variant should be
    /// NOTE: This is not supported yet.
    DLP,
    /// Variable length pack
    /// 
    /// TODO: Look into what this enum's variant should be
    /// NOTE: This is not supported yet.
    VLP,
    /// Floating length pack
    /// 
    /// TODO: Look into what this enum's variant should be
    /// NOTE: This is not supported yet.
    FLP,
    /// Local Set
    Set(Vec<KlvPacket>),
    /// String of characters following the utf8 standard
    UTF8(Arc<str>)
}

impl KlvValue {
    pub fn from_bytes(tag: Tag, bytes: &Box<[u8]>) -> Result<KlvValue, Errors> {
        let value = match tag {
            Tag::Checksum |
            Tag::PlatformHeadingAngle => Self::uint16(bytes),
            Tag::PrecisionTimeStamp => Self::uint64(bytes),
            Tag::MissionID => Self::utf8(bytes),
            _ => return Err(Errors::UnsupportedTag(tag as usize))
        };

        Ok(value)
    }

    fn uint16(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Uint16(bytes.view_bits::<Msb0>().load_be())
    }

    fn uint64(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Uint64(bytes.view_bits::<Msb0>().load_be())
    }

    fn utf8(bytes: &Box<[u8]>) -> KlvValue {
        return KlvValue::UTF8(std::str::from_utf8(bytes).expect("Cannot create UTF8 string from bytes").into());
    }
}