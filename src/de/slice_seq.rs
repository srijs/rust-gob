use std::io::Cursor;

use serde::de::{DeserializeSeed, SeqAccess};
use serde::de::value::Error;

use ::gob::Message;
use ::types::{SliceType, TypeDefs};
use super::ValueDeserializer;

pub(crate) struct SliceSeqAccess<'t, 'de> where 'de: 't {
    def: &'t SliceType,
    defs: &'t TypeDefs,
    remaining_count: u64,
    msg: &'t mut Message<Cursor<&'de [u8]>>
}

impl<'t, 'de> SliceSeqAccess<'t, 'de> {
    pub fn new(def: &'t SliceType, defs: &'t TypeDefs, msg: &'t mut Message<Cursor<&'de [u8]>>) -> Result<SliceSeqAccess<'t, 'de>, Error> {
        let remaining_count = msg.read_uint()?;

        Ok(SliceSeqAccess { def, defs, remaining_count, msg })
    }
}

impl<'f, 'de> SeqAccess<'de> for SliceSeqAccess<'f, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: DeserializeSeed<'de>
    {
        if self.remaining_count == 0 {
            return Ok(None);
        }
        self.remaining_count -= 1;
        let de = ValueDeserializer::new(self.def.elem, self.defs, &mut self.msg);
        seed.deserialize(de).map(Some)
    }
}
