#[cfg(feature = "log")]
#[cfg(not(test))]
use log::{debug, trace};

#[cfg(feature = "log")]
#[cfg(test)]
use std::{println as trace, println as warn, println as debug}; 

use crate::{klv_value::KlvValue, tag::Tag, Errors};

#[derive(Clone, Debug)]
pub struct Klv {
    tag: Tag,
    value: KlvValue,
}

impl Klv {
    pub fn new(tag_id: usize, raw_value: Box<[u8]>) -> Result<Klv, Errors> {
        // Convert the tag ID into the tag variant it corresponds to
        let tag = Tag::from(tag_id);

        #[cfg(feature="log")]
        trace!("Parsing value from tag [{:?}]", tag);

        // Return early if we know this tag is not supported
        if tag == Tag::Unknown || tag == Tag::Deprecated {
            return Err(Errors::UnsupportedTag(tag_id as usize))
        }

        let value = KlvValue::from_bytes(tag, &raw_value)?;
        
        Ok(Klv { tag, value })
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn value(&self) -> &KlvValue {
        &self.value
    }
}