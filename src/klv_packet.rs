use std::io::{Cursor, Read};

#[cfg(feature = "search")]
use memmem::{Searcher, TwoWaySearcher};
use strum::EnumCount;

use crate::{klv::Klv, tag::Tag, Errors};

pub const UAS_LOCAL_SET_UNIVERSAL_LABEL: [u8; 16] = [
    0x06, 0x0E, 0x2B, 0x34,
    0x02, 0x0B, 0x01, 0x01,
    0x0E, 0x01, 0x03, 0x01,
    0x01, 0x00, 0x00, 0x00,
];

#[derive(Clone, Debug)]
pub struct KlvPacket {
    fields: Vec<Klv>,
}

impl KlvPacket {
    /// Parse the bytes into a usable KLV packet
    pub fn from_bytes(bytes: Box<[u8]>) -> Result<KlvPacket, Errors> {
        let start_index: usize;
        
        #[cfg(feature = "search")]
        {
            let search = TwoWaySearcher::new(UAS_LOCAL_SET_UNIVERSAL_LABEL);
            start_index = match search.search_in(&bytes) {
                Some(idx) => idx,
                None => return Err(Errors::NoKLVPacket),
            };
        }
        #[cfg(not(feature = "search"))]
        {
            // Check if the first bytes are exactly the magic number from the UAS LS Label.
            let test_bytes = &bytes[0..];
            if ! test_bytes.iter().eq(UAS_LOCAL_SET_UNIVERSAL_LABEL.iter()) {
                return Err(Errors::NoKLVPacket)
            }

            start_index = 0;
        }

        // Create a cursor for the bytes so we can keep track of what has been read without a bunch
        // of magic numbers.
        let mut buffer = Cursor::new(bytes);
        buffer.set_position((start_index + UAS_LOCAL_SET_UNIVERSAL_LABEL.len()) as u64);

        // Get the length of the UAS Datalink Packet Value field.
        let klv_length = Self::get_length(&buffer);

        // Get the number of Tag variants that are currently supported.
        let max_tag_id = Tag::COUNT;

        let mut fields = Vec::new();

        while buffer.position() < (klv_length + UAS_LOCAL_SET_UNIVERSAL_LABEL.len()) as u64 {
            let tag = Self::get_tag(&buffer);
            if tag > max_tag_id {
                continue
            }

            let length = Self::get_length(&buffer);
            let value = Self::get_value(&mut buffer, tag, length)?;
            fields.push(value);
        }

        Ok(KlvPacket { fields })
    }
    
    pub fn get_id(&self, tag: usize) -> Option<Klv> {
        self.fields.iter().find(|field_tag| field_tag.tag() as usize == tag).cloned()
    }

    pub fn get(&self, tag: Tag) -> Option<Klv> {
        self.get_id(tag as usize)
    }

    /// Return the checksum of this UAS LS KLV packet
    pub fn checksum(&self) -> Klv {
        self.get(Tag::Checksum).expect("KLV packets must have a checksum")
    }
    
    /// Return the precision time stamp of the UAS LS KLV packet
    pub fn precision_time_stamp(&self) -> Klv {
        self.get(Tag::PrecisionTimeStamp).expect("KLV packets must have a precision time stamp")
    }

    /// Get the next tag from these bytes.
    /// 
    /// The first byte in the `bytes` slice should be the start of the next tag.
    fn get_tag(buf: &Cursor<Box<[u8]>>) -> usize {
        todo!()
    }

    /// Get the length of a field. This handled both non-BER and BER length values.
    fn get_length(buf: &Cursor<Box<[u8]>>) -> usize {
        todo!()

    }

    /// Get the BER value from the bytes given
    /// 
    /// The first byte in the `bytes` slice should be the start of the BER sequence.
    fn get_ber_value(buf: &Cursor<Box<[u8]>>) -> usize {

        // For each byte check to see if the byte starts with a 1 in the MSB position.

        // If there is a 1 then we need to parse this byte and then read the next.

        // If there is a 0 then this is the last byte.

        todo!()
    }

    fn get_value(buf: &mut Cursor<Box<[u8]>>, tag: usize, length: usize) -> Result<Klv, Errors> {
        let mut value_buf = vec![0; length];
        buf.read_exact(&mut value_buf).expect("Couldn't read all value data necessary from buffer");
        Klv::new(tag, value_buf.into())
    }
}


#[cfg(test)]
mod tests {
    use super::KlvPacket;

    // fn packet1() -> Box<[u8]> {

    // }

    // #[test]
    // fn from_bytes() {
    //     let packet = KlvPacket::from_bytes(packet1()).unwrap();
    //     assert_eq!(packet.checksum(), Some(0), "Checksum is incorrect");
    //     assert_eq!(packet.precision_time_stamp(), 0, "Precision Time Stamp is incorrect");
    // }
}