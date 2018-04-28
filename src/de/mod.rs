//! Deserialization

use std::io::Cursor;

use serde::{self, Deserialize};
use serde::de::Visitor;
use serde::de::value::Error;

use ::internal::gob::Message;
use ::internal::types::{TypeId, Types, WireType};

use ::internal::de::FieldValueDeserializer;
use ::internal::de::ValueDeserializer;

pub struct Deserializer<'de> {
    msg: Message<Cursor<&'de [u8]>>
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Deserializer<'de> {
        Deserializer {
            msg: Message::new(Cursor::new(input))
        }
    }
}

impl<'de> serde::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        let mut defs = Types::new();

        loop {
            let _len = self.msg.read_bytes_len()?;
            let type_id = self.msg.read_int()?;

            if type_id >= 0 {
                let de = ValueDeserializer::new(TypeId(type_id), &defs, &mut self.msg);
                return serde::de::Deserializer::deserialize_any(de, visitor);
            }

            let wire_type = {
                let de = FieldValueDeserializer::new(TypeId::WIRE_TYPE, &defs, &mut self.msg);
                WireType::deserialize(de)
            }?;

            if -type_id != wire_type.common().id.0 {
                return Err(serde::de::Error::custom(format!("type id mismatch")));
            }

            defs.insert(wire_type);
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        let int = i64::deserialize(self)?;
        if let Some(c) = ::std::char::from_u32(int as u32) {
            visitor.visit_char(c)
        } else {
            Err(serde::de::Error::custom(format!("invalid char code {}", int)))
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
