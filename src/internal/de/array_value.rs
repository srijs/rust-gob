use std::io::Cursor;

use serde;
use serde::de::{Deserializer, DeserializeSeed, SeqAccess, Visitor};
use serde::de::value::Error;

use ::internal::gob::Message;
use ::internal::types::{ArrayType, Types};
use super::FieldValueDeserializer;

struct ArraySeqAccess<'t, 'de> where 'de: 't {
    def: &'t ArrayType,
    defs: &'t Types,
    remaining_count: u64,
    msg: &'t mut Message<Cursor<&'de [u8]>>
}

impl<'t, 'de> ArraySeqAccess<'t, 'de> {
    fn new(def: &'t ArrayType, defs: &'t Types, msg: &'t mut Message<Cursor<&'de [u8]>>) -> Result<ArraySeqAccess<'t, 'de>, Error> {
        let remaining_count = msg.read_uint()?;

        if remaining_count != def.len as u64 {
            return Err(serde::de::Error::custom(format!("array len mismatch (expected {}, got {})", def.len, remaining_count)))
        }

        Ok(ArraySeqAccess { def, defs, remaining_count, msg })
    }
}

impl<'f, 'de> SeqAccess<'de> for ArraySeqAccess<'f, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: DeserializeSeed<'de>
    {
        if self.remaining_count == 0 {
            return Ok(None);
        }
        self.remaining_count -= 1;
        let de = FieldValueDeserializer::new(self.def.elem, self.defs, &mut self.msg);
        seed.deserialize(de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining_count as usize)
    }
}

pub(crate) struct ArrayValueDeserializer<'t, 'de> where 'de: 't {
    def: &'t ArrayType,
    defs: &'t Types,
    msg: &'t mut Message<Cursor<&'de [u8]>>
}

impl<'t, 'de> ArrayValueDeserializer<'t, 'de> {
    #[inline]
    pub(crate) fn new(def: &'t ArrayType, defs: &'t Types, msg: &'t mut Message<Cursor<&'de [u8]>>) -> ArrayValueDeserializer<'t, 'de> {
        ArrayValueDeserializer { def, defs, msg }
    }
}

impl<'t, 'de> Deserializer<'de> for ArrayValueDeserializer<'t, 'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        visitor.visit_seq(ArraySeqAccess::new(self.def, self.defs, self.msg)?)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
