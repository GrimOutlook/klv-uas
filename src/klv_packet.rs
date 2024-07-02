use std::{io::{Cursor, Read}, sync::Arc};
use bitvec::{field::BitField, order::Msb0, view::BitView};

use crate::{klv::Klv, klv_value::KlvValue, tag::Tag, Errors};

#[cfg(feature = "search")]
use memmem::{Searcher, TwoWaySearcher};

#[cfg(feature = "log")]
use log::{debug, trace};

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
    /// Get the next tag from these bytes. This handles both non-BER and BER length values.
    /// 
    /// The first byte in the `bytes` slice should be the start of the next tag.
    fn get_tag(buf: &mut Cursor<Box<[u8]>>) -> usize {
        Self::get_ber_value(buf)
    }

    /// Get the length of a field. This handles both non-BER and BER length values.
    fn get_length(buf: &mut Cursor<Box<[u8]>>) -> usize {
        Self::get_ber_value(buf)
    }

    /// Get the BER value from the bytes given
    /// 
    /// The first byte in the `bytes` slice should be the start of the BER sequence.
    fn get_ber_value(buf: &mut Cursor<Box<[u8]>>) -> usize {
        // Buffer for reading bytes
        let mut new_byte: [u8; 1] = [0];
        // Get the first byte so we can evaluate if we need more
        buf.read(&mut new_byte).expect("Can't read from bytes");
        
        // If the first bit is a 1 then this is a long-form BER.
        let bits = new_byte.view_bits::<Msb0>();
        let msb = bits.get(0).expect("Cannot get the first bit from the byte array");
        
        if msb == true {
            let Some(remainder) =  bits.get(1..bits.len()) else {
                panic!("Cannot get bits after first for BER byte")
            };
            let long_length = remainder.load_be();
            // Read all of the length bytes determined from the first length byte
            let mut len_bytes = vec![0; long_length];
            buf.read(&mut len_bytes).expect("Can't read from bytes");

            return len_bytes.view_bits::<Msb0>().load_be();
        }

        return bits.load_be();
    }

    fn get_value(buf: &mut Cursor<Box<[u8]>>, tag: usize, length: usize) -> Result<Klv, Errors> {
        let mut value_buf = vec![0; length];
        buf.read_exact(&mut value_buf).expect("Couldn't read all value data necessary from buffer");
        Klv::new(tag, value_buf.into())
    }

    // This function calculated the checksum of the packet buffer passed in. This should be the
    // entire packet, starting with the UAS LS Key and ending with the calculated checksum.
    fn calculate_checksum(buf: &[u8]) -> u16 {
        let mut bcc: u16 = 0;
        buf[..buf.len() - 2].iter().enumerate().for_each(|(idx, byte)| bcc = bcc.wrapping_add((*byte as u16) << (8 * ((idx + 1) % 2)) as u16));
        return bcc;
    }

    /// Parse the bytes into a usable KLV packet
    pub fn from_bytes(bytes: Box<[u8]>) -> Result<KlvPacket, Errors> {
        let start_index: usize;
        
        #[cfg(feature = "search")]
        {
            let search = TwoWaySearcher::new(&UAS_LOCAL_SET_UNIVERSAL_LABEL);
            start_index = match search.search_in(&bytes) {
                Some(idx) => idx,
                None => return Err(Errors::NoKLVPacket),
            };
        }
        #[cfg(not(feature = "search"))]
        {
            // Check if the first bytes are exactly the magic number from the UAS LS Label.
            let test_bytes = &bytes[0..UAS_LOCAL_SET_UNIVERSAL_LABEL.len()];
            if ! test_bytes.iter().eq(UAS_LOCAL_SET_UNIVERSAL_LABEL.iter()) {
                return Err(Errors::NoKLVPacket)
            }

            start_index = 0;
        }

        #[cfg(feature="log")]
        trace!("Parsing KLV packet: {:02X?}", bytes);

        // This is the position that the length bytes for the entire packet start at.
        let length_position = start_index + UAS_LOCAL_SET_UNIVERSAL_LABEL.len();

        // Create a cursor for the bytes so we can keep track of what has been read without a bunch
        // of magic numbers.
        let mut buffer = Cursor::new(bytes.clone());
        buffer.set_position(length_position as u64);

        // Get the length of the UAS Datalink Packet Value field.
        let klv_length = Self::get_length(&mut buffer);
        #[cfg(feature="log")]
        trace!("Length of packet [{}]", klv_length);

        // Need to keep track of how many bytes make up the length field as this is used when
        // grabbing the bytes that are used to calculate the checksum
        let length_length = buffer.position() as usize - length_position;

        // Index in the data where KLV data ends
        let klv_packet_end = length_position + length_length + klv_length;

        // Get the number of Tag variants that are currently supported.
        let max_tag_id = Tag::COUNT;

        let mut fields = Vec::new();

        while buffer.position() < klv_packet_end as u64 {
            let tag = Self::get_tag(&mut buffer);
            // If the tag is larger than the known max tag ID then we know it's not supported
            if tag > max_tag_id {
                return Err(Errors::UnsupportedTag(tag))
            }

            let length = Self::get_length(&mut buffer);

            // Continue on to the next field if the length of this one is 0
            if length == 0 {
                #[cfg(feature="log")]
                debug!("Length of tag [{}] is 0", tag);
                continue
            }

            let value = Self::get_value(&mut buffer, tag, length)?;
            
            #[cfg(feature="log")]
            trace!("Added tag to KLV packet: [{}]", Into::<&'static str>::into(value.tag()));
            fields.push(value);
        }

        println!("KLV packet has fields: {:?}", fields.iter().map(|i| i.tag().into()).collect::<Vec<&str>>());

        let packet = KlvPacket { fields };

        let packet_checksum = packet.checksum();

        let checksum_bytes_length = klv_packet_end;
        let calculated_checksum = Self::calculate_checksum(bytes.get(start_index..checksum_bytes_length).unwrap());

        if packet_checksum != calculated_checksum {
            #[cfg(feature="log")]
            debug!("Checksum for packet [{}] vs calculated checksum [{}]", packet_checksum, calculated_checksum);
            return Err(Errors::InvalidChecksum)
        }

        Ok(packet)
    }
    
    pub fn get_id(&self, tag: usize) -> Option<Klv> {
        self.fields.iter().find(|field_tag| tag == field_tag.tag().into()).cloned()
    }

    pub fn get(&self, tag: Tag) -> Option<Klv> {
        self.get_id(tag.into())
    }

    /// Return the checksum of this UAS LS KLV packet
    pub fn checksum(&self) -> u16 {
        match self.get(Tag::Checksum).expect("KLV packets must have a checksum").value() {
            KlvValue::Uint16(value) => *value,
            _ => panic!("This packet does not have a checksum and that error was not caught. This should be unreachable")
        }
    }
    
    /// Return the precision time stamp of the UAS LS KLV packet
    pub fn precision_time_stamp(&self) -> u64 {
        match self.get(Tag::PrecisionTimeStamp).expect("KLV packets must have a precision time stamp").value() {
            KlvValue::Uint64(value) => *value,
            _ => panic!("This packet does not have a checksum and that error was not caught. This should be unreachable")
        }
    }
    
    /// Return the precision time stamp of the UAS LS KLV packet
    pub fn mission_id(&self) -> Option<Arc<str>> {
        match self.get(Tag::MissionID)?.value() {
            KlvValue::Utf8(value) => Some(value.clone()),
            _ => panic!("This packet does not have a mission ID and that error was not caught. This should be unreachable")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, sync::Arc, vec};

    use itertools::chain;
    use test_case::test_case;

    use super::{KlvPacket, UAS_LOCAL_SET_UNIVERSAL_LABEL};

    fn packet_from_value(test_value: Vec<u8>) -> Vec<u8> {
        let precision_timestamp_bytes = vec![0x02, 0x08, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
        let checksum_header = vec![0x01, 0x02];
        // Add 4 for the checksum that will be added at the end.
        let length: Vec<u8> = vec![(precision_timestamp_bytes.len() + test_value.len() + 4).try_into().expect("Length does not fit in 1 byte")];

        let packet_minus_checksum: Box<[u8]> = chain!(Vec::from(UAS_LOCAL_SET_UNIVERSAL_LABEL), length, precision_timestamp_bytes, test_value, checksum_header).collect();
        let checksum = KlvPacket::calculate_checksum(&packet_minus_checksum);
        let packet = chain!(Vec::from(packet_minus_checksum), Vec::from(checksum.to_be_bytes())).collect();
        return packet;
    }

    fn packet_1() -> Vec<u8> {
        return packet_from_value(vec![0x03, 0x02, b'I', b'D']) // Mission ID is 'ID'.
    }

    #[test_case(packet_1, 46955, Some("ID".into()))]
    fn from_bytes(packet: fn () -> Vec<u8>, checksum: u16, mission_id: Option<Arc<str>>) {
        let bytes = packet();
        let packet = KlvPacket::from_bytes(bytes.into()).unwrap();
        assert_eq!(packet.checksum(), checksum, "Checksum is incorrect");
        assert_eq!(packet.precision_time_stamp(), 4822678189205111, "Precision Time Stamp is incorrect");
        assert_eq!(packet.mission_id(), mission_id)
    }

    #[test_case(Box::new([0x71, 0xF1, 0x00]), 113; "Short form")]
    #[test_case(Box::new([0x81, 0xF1, 0x00]), 241; "Long-Form: One byte")]
    #[test_case(Box::new([0x83, 0xF1, 0xFF, 0xF1]), 15859697; "Long-Form Three bytes")]
    fn get_ber_value(bytes: Box<[u8]>, correct_length: usize) {
        let mut test_bytes = Cursor::new(bytes.clone());
        let length = KlvPacket::get_ber_value(&mut test_bytes);
        assert_eq!(length, correct_length, "Failed to BER")
    }
}