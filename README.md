# klv-uas

[![Crates.io Total Downloads](https://img.shields.io/crates/d/klv-uas)](https://crates.io/crates/klv-uas)
[![docs.rs](https://img.shields.io/docsrs/klv-uas)](https://docs.rs/klv-uas)
[![Crates.io Version](https://img.shields.io/crates/v/klv-uas)](https://crates.io/crates/klv-uas/versions)
[![GitHub Repo stars](https://img.shields.io/github/stars/GrimOutlook/klv-uas)](https://github.com/GrimOutlook/klv-uas)
[![Crates.io License](https://img.shields.io/crates/l/klv-uas)](../LICENSE)

A library for extracting KLV data from transport stream packet payloads. This library is not
indented to be used for injecting KLV data into video streams.

## Example

```rust
extern crate klv_uas;

use std::env;
use ts_analyzer::reader::TSReader;
use std::fs::File;
use std::io::BufReader;
use klv_uas::klv_packet::KlvPacket;

fn main() {
    env_logger::init();
    let filename = env::var("TEST_FILE").expect("Environment variable not set");

    let f = File::open(filename.clone()).expect("Couldn't open file");
    let mut reader = TSReader::new(f).expect("Transport Stream file contains no SYNC bytes.");

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

    println!("Timestamp of KLV packet: {}", klv.precision_time_stamp());
}
```

### Goals

- [ ] Support parsing all value types from KLV fields.
  - [x] int
  - [x] int8
  - [x] int16
  - [x] int32
  - [x] uint
  - [x] uint8
  - [x] uint16
  - [x] uint32
  - [x] uint64
  - [ ] IMAPB
  - [ ] Byte
  - [ ] DLP
  - [ ] VLP
  - [ ] FLP
  - [ ] Set
  - [x] UTF8

### Testing

`TEST_FILE="$HOME/Truck.ts" cargo run --features search --example klv_timestamp`

---

## Reference Material

- A sample TS stream with KLV data can be found [here](https://www.arcgis.com/home/item.html?id=55ec6f32d5e342fcbfba376ca2cc409a).
- The standards for KLV metadata can be found [here](https://nsgreg.nga.mil/misb.jsp). Find `MISB ST 0107.X` and click `FILE`. The current link is [here](https://kubic-nsg-standards-nsgreg-nsgreg-files-6lxvt.s3.us-east-1.amazonaws.com/doc/Document/ST0107.5.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVXR7TTKDX37WLG6Z%2F20240530%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240530T191903Z&X-Amz-Expires=7200&X-Amz-SignedHeaders=host&response-cache-control=7200&response-content-disposition=inline&response-content-type=application%2Fpdf&X-Amz-Signature=6f5bea7707638df7b9bd51389eca587021b89c22a851be32113f66acf42bcdfc) but is likely to change.
- The standards for the UAS Datalink Local Set can be found [here](https://nsgreg.nga.mil/misb.jsp). Find `MISB ST 0601.X` and click `FILE`. The current link is [here](https://kubic-nsg-standards-nsgreg-nsgreg-files-6lxvt.s3.us-east-1.amazonaws.com/doc/Document/ST0601.19.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVXR7TTKDX37WLG6Z%2F20240530%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240530T191903Z&X-Amz-Expires=7200&X-Amz-SignedHeaders=host&response-cache-control=7200&response-content-disposition=inline&response-content-type=application%2Fpdf&X-Amz-Signature=6d80dcb5bae2542423382f17ec4c2ba23366c1378ccc33abf55ccf39dac7b1f0) but is likely to change.
