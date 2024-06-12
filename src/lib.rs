#![forbid(unsafe_code)]
// Use these checks when closer to complete. They're a bit too strict for early development.
// #![deny(future_incompatible, missing_docs, rust_2018_idioms, unused, warnings)]
#[doc = include_str!("../README.md")]

pub mod klv_packet;
pub mod klv_value;
pub mod klv;
pub mod tag;

#[derive(Debug)]
pub enum Errors {
    NoKLVPacket,
    ValueOutOfBounds,
    UnsupportedTag(usize),
    InvalidChecksum,
}