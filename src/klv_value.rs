//! Information from this page was gathered from page 32 of the MISB ST 0601.19 document that was
//! published 2023-March-02.
use std::sync::Arc;

use crate::klv_packet::KLVPacket;

/// The value types that are supported to be stored in a UAS Datalink KLV packet
#[derive(Clone, Debug)]
pub enum Value {
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
    Set(Vec<KLVPacket>),
    /// String of characters following the utf8 standard
    UTF8(Arc<str>)
}