extern crate klv_uas;

use std::env;
use klv_uas::tag::Tag;
use ts_analyzer::reader::TSReader;
use std::fs::File;
use klv_uas::klv_packet::KlvPacket;

fn main() {
    env_logger::init();
    let filename = env::var("TEST_FILE").expect("Environment variable not set");

    let f = File::open(filename.clone()).expect("Couldn't open file");
    let mut reader = TSReader::new(f).expect("Transport Stream file contains no SYNC bytes.");

    reader.add_tracked_pid(258);

    let klv: KlvPacket;
    loop {
        // Get a payload from the reader. The `unchecked` in the method name means that if an error
        // is hit then `Some(payload)` is returned rather than `Ok(Some(payload))` in order to reduce
        // `.unwrap()` (or other) calls.
        let payload = reader.next_payload_unchecked()
                       // Assume that a payload was found in the file and was successfully parsed.
                       .expect("No valid payload found");

        // Try to parse a UAS LS KLV packet from the payload that was found. This will likely only
        // work if you have the `search` feature enabled as the UAS LS KLV record does not start at
        // the first byte of the payload.
        klv = match KlvPacket::from_bytes(payload) {
            Ok(klv) => klv,
            Err(e) => {
                println!("Error {:?}", e);
                continue
            },
        };

        break
    }

    println!("Timestamp of KLV packet: {:?}", klv.get(Tag::PrecisionTimeStamp).unwrap());
}