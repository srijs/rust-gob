use std::io::Cursor;

use bytes::Buf;
use serde::de::{IgnoredAny, Visitor};
use serde::{self, Deserialize};

use error::Error;
use internal::gob::Message;
use internal::types::{TypeId, Types, WireType};

use super::complex_value::ComplexValueDeserializer;
use super::map_value::MapValueDeserializer;
use super::seq_value::SeqValueDeserializer;
use super::struct_value::StructValueDeserializer;

pub(crate) struct FieldValueDeserializer<'t, 'de>
where
    'de: 't,
{
    type_id: TypeId,
    defs: &'t Types,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> FieldValueDeserializer<'t, 'de> {
    pub fn new(
        type_id: TypeId,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> FieldValueDeserializer<'t, 'de> {
        FieldValueDeserializer { type_id, defs, msg }
    }

    fn deserialize_byte_slice(&mut self) -> Result<&'de [u8], Error> {
        let len = self.msg.read_bytes_len()?;
        let pos = self.msg.get_ref().position() as usize;
        self.msg.get_mut().advance(len);
        let bytes = &self.msg.get_ref().get_ref()[pos..pos + len];
        Ok(bytes)
    }

    fn deserialize_str_slice(&mut self) -> Result<&'de str, Error> {
        let bytes = self.deserialize_byte_slice()?;
        ::std::str::from_utf8(bytes).map_err(|err| serde::de::Error::custom(err))
    }
}

macro_rules! primitive {
    ($fname:tt, $tname:tt, $visit:tt, $id:tt, $parse:expr) => {
        fn $fname<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            if self.type_id == TypeId::$id {
                visitor.$visit($parse(self)? as $tname)
            } else {
                Err(serde::de::Error::custom(format!("expected {}", stringify!($tname))))
            }
        }
    }
}

impl<'t, 'de> serde::Deserializer<'de> for FieldValueDeserializer<'t, 'de> {
    type Error = Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.type_id {
            TypeId::BOOL => visitor.visit_bool(self.msg.read_bool()?),
            TypeId::INT => visitor.visit_i64(self.msg.read_int()?),
            TypeId::UINT => visitor.visit_u64(self.msg.read_uint()?),
            TypeId::FLOAT => visitor.visit_f64(self.msg.read_float()?),
            TypeId::BYTES => visitor.visit_borrowed_bytes(self.deserialize_byte_slice()?),
            TypeId::STRING => visitor.visit_borrowed_str(self.deserialize_str_slice()?),
            TypeId::COMPLEX => ComplexValueDeserializer::new(self.msg).deserialize_any(visitor),
            _ => {
                if let Some(wire_type) = self.defs.lookup(self.type_id) {
                    match wire_type {
                        &WireType::Struct(ref struct_type) => {
                            let de = StructValueDeserializer::new(struct_type, self.defs, self.msg);
                            de.deserialize_any(visitor)
                        }
                        &WireType::Slice(ref slice_type) => {
                            let de = SeqValueDeserializer::new(
                                None,
                                slice_type.elem,
                                self.defs,
                                self.msg,
                            );
                            de.deserialize_any(visitor)
                        }
                        &WireType::Array(ref array_type) => {
                            let de = SeqValueDeserializer::new(
                                Some(array_type.len as usize),
                                array_type.elem,
                                self.defs,
                                self.msg,
                            );
                            de.deserialize_any(visitor)
                        }
                        &WireType::Map(ref map_type) => {
                            let de = MapValueDeserializer::new(map_type, self.defs, self.msg);
                            de.deserialize_any(visitor)
                        }
                    }
                } else {
                    Err(serde::de::Error::custom(format!(
                        "unknown type id {:?}",
                        self.type_id
                    )))
                }
            }
        }
    }

    primitive!(deserialize_bool, bool, visit_bool, BOOL, |d: Self| d.msg
        .read_bool());

    primitive!(deserialize_i8, i8, visit_i8, INT, |d: Self| d.msg
        .read_int());
    primitive!(deserialize_i16, i16, visit_i16, INT, |d: Self| d.msg
        .read_int());
    primitive!(deserialize_i32, i32, visit_i32, INT, |d: Self| d.msg
        .read_int());
    primitive!(deserialize_i64, i64, visit_i64, INT, |d: Self| d.msg
        .read_int());

    primitive!(deserialize_u8, u8, visit_u8, UINT, |d: Self| d.msg
        .read_uint());
    primitive!(deserialize_u16, u16, visit_u16, UINT, |d: Self| d.msg
        .read_uint());
    primitive!(deserialize_u32, u32, visit_u32, UINT, |d: Self| d.msg
        .read_uint());
    primitive!(deserialize_u64, u64, visit_u64, UINT, |d: Self| d.msg
        .read_uint());

    primitive!(deserialize_f32, f32, visit_f32, FLOAT, |d: Self| d.msg
        .read_float());
    primitive!(deserialize_f64, f64, visit_f64, FLOAT, |d: Self| d.msg
        .read_float());

    fn deserialize_str<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.type_id == TypeId::STRING {
            visitor.visit_borrowed_str(self.deserialize_str_slice()?)
        } else {
            Err(serde::de::Error::custom("expected str"))
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.type_id == TypeId::BYTES {
            visitor.visit_borrowed_bytes(self.deserialize_byte_slice()?)
        } else {
            Err(serde::de::Error::custom("expected bytes"))
        }
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_bytes(visitor)
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
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(&WireType::Struct(ref struct_type)) = self.defs.lookup(self.type_id) {
            let de = StructValueDeserializer::new(struct_type, self.defs, self.msg);
            de.deserialize_enum(name, variants, visitor)
        } else {
            Err(serde::de::Error::custom("not an enum type"))
        }
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(&WireType::Struct(ref struct_type)) = self.defs.lookup(self.type_id) {
            let de = StructValueDeserializer::new(struct_type, self.defs, self.msg);
            de.deserialize_struct(name, fields, visitor)
        } else {
            Err(serde::de::Error::custom("not a struct type"))
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
        option unit_struct newtype_struct seq tuple
        tuple_struct map identifier ignored_any
    }
}
