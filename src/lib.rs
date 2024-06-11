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