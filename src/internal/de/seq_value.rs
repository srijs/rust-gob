use std::io::Cursor;

use serde::de::value::Error;
use serde::de::{self, DeserializeSeed, Deserializer, Visitor};

use super::FieldValueDeserializer;
use internal::gob::Message;
use internal::types::{TypeId, Types};

struct SeqAccess<'t, 'de>
where
    'de: 't,
{
    element: TypeId,
    defs: &'t Types,
    remaining_count: u64,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> SeqAccess<'t, 'de> {
    fn new(
        len: Option<usize>,
        element: TypeId,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> Result<SeqAccess<'t, 'de>, Error> {
        let remaining_count = msg.read_uint()?;

        if let Some(len) = len {
            if remaining_count != len as u64 {
                return Err(de::Error::custom(format!(
                    "sequence len mismatch (expected {}, got {})",
                    len, remaining_count
                )));
            }
        }

        Ok(SeqAccess {
            element,
            defs,
            remaining_count,
            msg,
        })
    }
}

impl<'f, 'de> de::SeqAccess<'de> for SeqAccess<'f, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remaining_count == 0 {
            return Ok(None);
        }
        self.remaining_count -= 1;
        let de = FieldValueDeserializer::new(self.element, self.defs, &mut self.msg);
        seed.deserialize(de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining_count as usize)
    }
}

pub(crate) struct SeqValueDeserializer<'t, 'de>
where
    'de: 't,
{
    len: Option<usize>,
    element: TypeId,
    defs: &'t Types,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> SeqValueDeserializer<'t, 'de> {
    #[inline]
    pub(crate) fn new(
        len: Option<usize>,
        element: TypeId,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> SeqValueDeserializer<'t, 'de> {
        SeqValueDeserializer {
            len,
            element,
            defs,
            msg,
        }
    }
}

impl<'t, 'de> Deserializer<'de> for SeqValueDeserializer<'t, 'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self.len, self.element, self.defs, self.msg)?)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
