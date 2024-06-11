#[cfg(feature = "search")]
use memmem::{Searcher, TwoWaySearcher};

use crate::Errors;

pub const UAS_LOCAL_SET_UNIVERSAL_LABEL: [u8; 16] = [
    0x06, 0x0E, 0x2B, 0x34,
    0x02, 0x0B, 0x01, 0x01,
    0x0E, 0x01, 0x03, 0x01,
    0x01, 0x00, 0x00, 0x00,
];

#[derive(Clone, Debug)]
pub struct KLVPacket {
    /// Checksum used to detect errors within a UAS Datalink LS packet
    checksum: u16,
    /// Timestamp for all metadata in this Local Set in microseconds
    precision_time_stamp: u64,
}

impl KLVPacket {
    /// Parse the bytes into a usable KLV packet
    pub fn from_bytes(bytes: Box<[u8]>) -> Result<KLVPacket, Errors> {
        let start_index: u64;
        
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
            let test_bytes = &bytes[0..UAS_LOCAL_SET_UNIVERSAL_LABEL.len()];
            if ! test_bytes.iter().eq(UAS_LOCAL_SET_UNIVERSAL_LABEL.iter()) {
                return Err(Errors::NoKLVPacket)
            }

            start_index = 0;
        }
        
        // Check the raw bytes to find the index where
        let klv = KLVPacket {
            checksum: 0,
            precision_time_stamp: 0,
        };
        Ok(klv)
    }

    /// Return the checksum of this UAS LS KLV packet
    pub fn checksum(&self) -> u16 { self.checksum }
    /// Return the precision time stamp of the UAS LS KLV packet
    pub fn precision_time_stamp(&self) -> u64 { self.precision_time_stamp }
}