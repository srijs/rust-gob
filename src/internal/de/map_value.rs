use std::io::Cursor;

use serde::de::{DeserializeSeed, Deserializer, MapAccess, Visitor};

use super::FieldValueDeserializer;
use error::Error;
use internal::gob::Message;
use internal::types::{MapType, Types};

struct MapMapAccess<'t, 'de>
where
    'de: 't,
{
    def: &'t MapType,
    defs: &'t Types,
    remaining_count: u64,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> MapMapAccess<'t, 'de> {
    fn new(
        def: &'t MapType,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> Result<MapMapAccess<'t, 'de>, Error> {
        let remaining_count = msg.read_uint()?;

        Ok(MapMapAccess {
            def,
            defs,
            remaining_count,
            msg,
        })
    }
}

impl<'f, 'de> MapAccess<'de> for MapMapAccess<'f, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.remaining_count == 0 {
            return Ok(None);
        }
        self.remaining_count -= 1;
        let de = FieldValueDeserializer::new(self.def.key, self.defs, &mut self.msg);
        seed.deserialize(de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let de = FieldValueDeserializer::new(self.def.elem, self.defs, &mut self.msg);
        seed.deserialize(de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining_count as usize)
    }
}

pub(crate) struct MapValueDeserializer<'t, 'de>
where
    'de: 't,
{
    def: &'t MapType,
    defs: &'t Types,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> MapValueDeserializer<'t, 'de> {
    #[inline]
    pub(crate) fn new(
        def: &'t MapType,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> MapValueDeserializer<'t, 'de> {
        MapValueDeserializer { def, defs, msg }
    }
}

impl<'t, 'de> Deserializer<'de> for MapValueDeserializer<'t, 'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MapMapAccess::new(self.def, self.defs, self.msg)?)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
