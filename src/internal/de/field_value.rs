use std::io::Cursor;

use bytes::Buf;
use serde::{self, Deserialize};
use serde::de::Visitor;
use serde::de::value::Error;

use ::internal::gob::Message;
use ::internal::types::{TypeId, Types, WireType};

use super::struct_value::StructValueDeserializer;
use super::seq_value::SeqValueDeserializer;
use super::map_value::MapValueDeserializer;
use super::complex_value::ComplexValueDeserializer;

pub(crate) struct FieldValueDeserializer<'t, 'de> where 'de: 't {
    type_id: TypeId,
    defs: &'t Types,
    msg: &'t mut Message<Cursor<&'de [u8]>>
}

impl<'t, 'de> FieldValueDeserializer<'t, 'de> {
    pub fn new(type_id: TypeId, defs: &'t Types, msg: &'t mut Message<Cursor<&'de [u8]>>) -> FieldValueDeserializer<'t, 'de> {
        FieldValueDeserializer {
            type_id, defs, msg
        }
    }

    fn deserialize_byte_slice(&mut self) -> Result<&'de [u8], Error> {
        let len = self.msg.read_bytes_len()?;
        let pos = self.msg.get_ref().position() as usize;
        self.msg.get_mut().advance(len);
        let bytes = &self.msg.get_ref().get_ref()[pos..pos+len];
        Ok(bytes)
    }

    fn deserialize_str_slice(&mut self) -> Result<&'de str, Error> {
        let bytes = self.deserialize_byte_slice()?;
        ::std::str::from_utf8(bytes)
            .map_err(|err| serde::de::Error::custom(err))
    }
}

impl<'t, 'de> serde::Deserializer<'de> for FieldValueDeserializer<'t, 'de> {
    type Error = Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match self.type_id {
            TypeId::BOOL => visitor.visit_bool(self.msg.read_bool()?),
            TypeId::INT => visitor.visit_i64(self.msg.read_int()?),
            TypeId::UINT => visitor.visit_u64(self.msg.read_uint()?),
            TypeId::FLOAT => visitor.visit_f64(self.msg.read_float()?),
            TypeId::BYTES => visitor.visit_borrowed_bytes(self.deserialize_byte_slice()?),
            TypeId::STRING => visitor.visit_borrowed_str(self.deserialize_str_slice()?),
            TypeId::COMPLEX => {
                ComplexValueDeserializer::new(self.msg).deserialize_any(visitor)
            },
            _ => {
                if let Some(wire_type) = self.defs.lookup(self.type_id) {
                    match wire_type {
                        &WireType::Struct(ref struct_type) => {
                            let de = StructValueDeserializer::new(struct_type, self.defs, self.msg);
                            de.deserialize_any(visitor)
                        },
                        &WireType::Slice(ref slice_type) => {
                            let de = SeqValueDeserializer::new(None, slice_type.elem, self.defs, self.msg);
                            de.deserialize_any(visitor)
                        },
                        &WireType::Array(ref array_type) => {
                            let de = SeqValueDeserializer::new(Some(array_type.len as usize), array_type.elem, self.defs, self.msg);
                            de.deserialize_any(visitor)
                        },
                        &WireType::Map(ref map_type) => {
                            let de = MapValueDeserializer::new(map_type, self.defs, self.msg);
                            de.deserialize_any(visitor)
                        }
                    }
                } else {
                    Err(serde::de::Error::custom(format!("unknown type id {:?}", self.type_id)))
                }
            }
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

    #[inline]
    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        if let Some(&WireType::Struct(ref struct_type)) = self.defs.lookup(self.type_id) {
            let de = StructValueDeserializer::new(struct_type, self.defs, self.msg);
            de.deserialize_enum(name, variants, visitor)
        } else {
            Err(serde::de::Error::custom("not an enum type"))
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
