//! Information from this page was gathered from page 32 of the MISB ST 0601.19 document that was
//! published 2023-March-02.
use std::sync::Arc;

use bitvec::{field::BitField, order::Msb0, view::BitView};
use strum_macros::EnumDiscriminants;

use crate::{klv_packet::KlvPacket, tag::Tag, Errors};

#[cfg(feature = "log")]
use log::warn;

/// The value types that are supported to be stored in a UAS Datalink KLV packet.
/// The first value is always the tag number. The second value is the value.
#[derive(Clone, Debug, EnumDiscriminants)]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(KlvValueType))]
pub enum KlvValue {
    /// This KLV tag is unknown.
    Unknown,
    /// This KLV tag has been deprecated.
    Deprecated,
    /// This KLV value type has yet to be implemented.
    Unimplemented,
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
    Byte(Box<[u8]>),
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
    Utf8(Arc<str>)
}

impl KlvValue {
    pub fn from_bytes(tag: Tag, bytes: &Box<[u8]>) -> Result<KlvValue, Errors> {
        let t = tag.tag_type();
        let value = match t {
            KlvValueType::Int       => Self::int(bytes),
            KlvValueType::Int8      => Self::int8(bytes),
            KlvValueType::Int16     => Self::int16(bytes),
            KlvValueType::Int32     => Self::int32(bytes),
            KlvValueType::Uint      => Self::uint(bytes),
            KlvValueType::Uint8     => Self::uint8(bytes),
            KlvValueType::Uint16    => Self::uint16(bytes),
            KlvValueType::Uint32    => Self::uint32(bytes),
            KlvValueType::Uint64    => Self::uint64(bytes),
            KlvValueType::Utf8      => Self::utf8(bytes),
            KlvValueType::IMAPB     => Self::imapb(bytes),
            KlvValueType::Set       => Self::set(tag, bytes),
            KlvValueType::Byte      => Self::byte(bytes),
            KlvValueType::DLP       => Self::dlp(bytes),
            KlvValueType::VLP       => Self::vlp(bytes),
            KlvValueType::FLP       => Self::flp(bytes),
            _ => return Err(Errors::UnsupportedTag(tag.into()))
        };

        Ok(value)
    }

    fn int(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Int(bytes.view_bits::<Msb0>().load_be())
    }

    fn int8(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Int8(bytes.view_bits::<Msb0>().load_be())
    }

    fn int16(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Int16(bytes.view_bits::<Msb0>().load_be())
    }

    fn int32(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Int32(bytes.view_bits::<Msb0>().load_be())
    }

    fn uint(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Uint(bytes.view_bits::<Msb0>().load_be())
    }

    fn uint8(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Uint8(bytes.view_bits::<Msb0>().load_be())
    }

    fn uint16(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Uint16(bytes.view_bits::<Msb0>().load_be())
    }

    fn uint32(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Uint32(bytes.view_bits::<Msb0>().load_be())
    }

    fn uint64(bytes: &Box<[u8]>) -> KlvValue {
        KlvValue::Uint64(bytes.view_bits::<Msb0>().load_be())
    }

    fn utf8(bytes: &Box<[u8]>) -> KlvValue {
        return KlvValue::Utf8(std::str::from_utf8(bytes)
                .expect(&format!("Cannot create UTF8 string from bytes {:02X?}", bytes)).into());
    }

    fn imapb(bytes: &Box<[u8]>) -> KlvValue {
        Self::klv_unimplemented("IMAPB")
    }

    fn set(tag: Tag, bytes: &Box<[u8]>) -> KlvValue {
        Self::klv_unimplemented("Set")
    }

    fn byte(bytes: &Box<[u8]>) -> KlvValue {
        Self::klv_unimplemented("Byte")
    }

    fn dlp(bytes: &Box<[u8]>) -> KlvValue {
        Self::klv_unimplemented("DLP")
    }

    fn vlp(bytes: &Box<[u8]>) -> KlvValue {
        Self::klv_unimplemented("VLP")
    }

    fn flp(bytes: &Box<[u8]>) -> KlvValue {
        Self::klv_unimplemented("FLP")
    }

    fn klv_unimplemented(tag_type: &str) -> KlvValue {
        #[cfg(not(feature = "ignore_incomplete"))]
        {
            todo!("Implement converting KLV bytes to {}", tag_type);
        }
        #[cfg(feature = "ignore_incomplete")]
        {
            warn!("Converting KLV bytes to {} is not yet supported", tag_type);
            KlvValue::Unimplemented
        }
    }
}