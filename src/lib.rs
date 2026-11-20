#![forbid(unsafe_code)]
// Use these checks when closer to complete. They're a bit too strict for early
// development. #![deny(future_incompatible, missing_docs, rust_2018_idioms,
// unused, warnings)]
#[doc = include_str!("../README.md")]
pub mod klv;
pub mod klv_packet;
pub mod klv_value;
pub mod tag;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Tag `{0}` is currently unsupported")]
    UnsupportedTag(usize),
    #[error("Checksum for KLV packet is invalid")]
    InvalidChecksum,
    #[error("Tag `{tag}` has {len} value bytes, more than its type holds")]
    InvalidLength { tag: usize, len: usize },
}
