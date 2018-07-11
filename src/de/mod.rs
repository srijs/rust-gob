//! Deserialization

use std::io::{Cursor, Read};

use bytes::Buf;
use serde::de::{IgnoredAny, Visitor};
use serde::{self, Deserialize};

use error::Error;
use internal::gob::{Message, Stream};
use internal::types::{TypeId, Types, WireType};
use internal::utils::{Bow, Buffer};

use internal::de::FieldValueDeserializer;
use internal::de::ValueDeserializer;

pub struct StreamDeserializer<R> {
    defs: Types,
    stream: Stream<R>,
    buffer: Buffer,
    prev_len: usize,
}

impl<R> StreamDeserializer<R> {
    pub fn new(read: R) -> Self {
        StreamDeserializer {
            defs: Types::new(),
            stream: Stream::new(read),
            buffer: Buffer::new(),
            prev_len: 0,
        }
    }

    pub fn deserialize<'de, T>(&'de mut self) -> Result<Option<T>, Error>
    where
        R: Read,
        T: Deserialize<'de>,
    {
        if let Some(deserializer) = self.deserializer()? {
            Ok(Some(T::deserialize(deserializer)?))
        } else {
            Ok(None)
        }
    }

    pub fn deserializer<'de>(&'de mut self) -> Result<Option<Deserializer<'de>>, Error>
    where
        R: Read,
    {
        if self.prev_len > 0 {
            self.buffer.advance(self.prev_len);
            self.prev_len = 0;
        }
        loop {
            let header = match self.stream.read_section(&mut self.buffer)? {
                Some(header) => header,
                None => return Ok(None),
            };

            if header.type_id >= 0 {
                let slice = &self.buffer.bytes()[header.payload_range.clone()];
                let msg = Message::new(Cursor::new(slice));
                self.prev_len = header.payload_range.end;
                return Ok(Some(Deserializer {
                    defs: Bow::Borrowed(&mut self.defs),
                    msg: msg,
                    type_id: Some(TypeId(header.type_id)),
                }));
            }

            let wire_type = {
                let slice = &self.buffer.bytes()[header.payload_range.clone()];
                let mut msg = Message::new(Cursor::new(slice));
                let de = FieldValueDeserializer::new(TypeId::WIRE_TYPE, &self.defs, &mut msg);
                WireType::deserialize(de)
            }?;

            if -header.type_id != wire_type.common().id.0 {
                return Err(Error::deserialize("type id mismatch"));
            }

            self.defs.insert(wire_type);
            self.buffer.advance(header.payload_range.end);
        }
    }

    pub fn get_ref(&self) -> &R {
        self.stream.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut R {
        self.stream.get_mut()
    }

    pub fn into_inner(self) -> R {
        self.stream.into_inner()
    }
}

pub struct Deserializer<'de> {
    defs: Bow<'de, Types>,
    msg: Message<Cursor<&'de [u8]>>,
    type_id: Option<TypeId>,
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Deserializer<'de> {
        Deserializer {
            defs: Bow::Owned(Types::new()),
            msg: Message::new(Cursor::new(input)),
            type_id: None,
        }
    }

    fn value_deserializer<'t>(&'t mut self) -> Result<ValueDeserializer<'t, 'de>, Error> {
        if let Some(type_id) = self.type_id {
            return Ok(ValueDeserializer::new(type_id, &self.defs, &mut self.msg));
        }

        loop {
            let _len = self.msg.read_bytes_len()?;
            let type_id = self.msg.read_int()?;

            if type_id >= 0 {
                return Ok(ValueDeserializer::new(
                    TypeId(type_id),
                    &self.defs,
                    &mut self.msg,
                ));
            }

            let wire_type = {
                let de = FieldValueDeserializer::new(TypeId::WIRE_TYPE, &self.defs, &mut self.msg);
                WireType::deserialize(de)
            }?;

            if -type_id != wire_type.common().id.0 {
                return Err(serde::de::Error::custom(format!("type id mismatch")));
            }

            self.defs.insert(wire_type);
        }
    }
}

impl<'de> serde::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.value_deserializer()?.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.value_deserializer()?
            .deserialize_enum(name, variants, visitor)
    }

    fn deserialize_struct<V>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.value_deserializer()?
            .deserialize_struct(name, fields, visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let int = i64::deserialize(self)?;
        if let Some(c) = ::std::char::from_u32(int as u32) {
            visitor.visit_char(c)
        } else {
            Err(serde::de::Error::custom(format!(
                "invalid char code {}",
                int
            )))
        }
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_ignored_any(IgnoredAny)?;
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 str string bytes
        byte_buf option unit_struct newtype_struct seq tuple
        tuple_struct map identifier ignored_any
    }
}
