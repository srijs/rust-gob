use std::io::Cursor;

use serde;
use serde::de::{DeserializeSeed, Deserializer, IntoDeserializer, Visitor};
use serde::de::{EnumAccess, MapAccess, VariantAccess};

use super::FieldValueDeserializer;
use error::Error;
use internal::gob::Message;
use internal::types::{FieldType, StructType, Types};

struct StructAccess<'t, 'de>
where
    'de: 't,
{
    def: &'t StructType,
    defs: &'t Types,
    field_no: i64,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> StructAccess<'t, 'de> {
    fn new(
        def: &'t StructType,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> StructAccess<'t, 'de> {
        StructAccess {
            def,
            defs,
            field_no: -1,
            msg,
        }
    }

    fn current_field(&self) -> Result<&'t FieldType, Error> {
        let field_no = self.field_no as usize;
        if field_no >= self.def.fields.len() {
            return Err(serde::de::Error::custom(format!(
                "field number overflow ({}) on type {:?}",
                field_no, self.def
            )));
        }
        Ok(&self.def.fields[field_no])
    }
}

impl<'t, 'de> MapAccess<'de> for StructAccess<'t, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let field_delta = self.msg.read_uint()?;

        if field_delta == 0 {
            return Ok(None);
        }

        self.field_no += field_delta as i64;
        let field = self.current_field()?;

        let de = <&str as IntoDeserializer<'_, Error>>::into_deserializer(&field.name);
        let value = seed.deserialize(de)?;
        Ok(Some(value))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let field = self.current_field()?;
        let de = FieldValueDeserializer::new(field.id, self.defs, &mut self.msg);
        seed.deserialize(de)
    }
}

impl<'t, 'de> EnumAccess<'de> for StructAccess<'t, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(val) = self.next_key_seed(seed)? {
            Ok((val, self))
        } else {
            Err(serde::de::Error::custom("encountered empty enum struct"))
        }
    }
}

impl<'t, 'de> VariantAccess<'de> for StructAccess<'t, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        Err(serde::de::Error::custom("unit variants not supported yet"))
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        let field = self.current_field()?;
        let val = {
            let de = FieldValueDeserializer::new(field.id, self.defs, &mut self.msg);
            seed.deserialize(de)?
        };
        let field_delta = self.msg.read_uint()?;
        if field_delta != 0 {
            Err(serde::de::Error::custom(
                "enum struct has more than one field",
            ))
        } else {
            Ok(val)
        }
    }

    fn tuple_variant<V>(mut self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let field = self.current_field()?;
        let val = {
            let de = FieldValueDeserializer::new(field.id, self.defs, &mut self.msg);
            de.deserialize_seq(visitor)?
        };
        let field_delta = self.msg.read_uint()?;
        if field_delta != 0 {
            Err(serde::de::Error::custom(
                "enum struct has more than one field",
            ))
        } else {
            Ok(val)
        }
    }

    fn struct_variant<V>(
        mut self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let field = self.current_field()?;
        let val = {
            let de = FieldValueDeserializer::new(field.id, self.defs, &mut self.msg);
            de.deserialize_seq(visitor)?
        };
        let field_delta = self.msg.read_uint()?;
        if field_delta != 0 {
            Err(serde::de::Error::custom(
                "enum struct has more than one field",
            ))
        } else {
            Ok(val)
        }
    }
}

pub(crate) struct StructValueDeserializer<'t, 'de>
where
    'de: 't,
{
    def: &'t StructType,
    defs: &'t Types,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> StructValueDeserializer<'t, 'de> {
    #[inline]
    pub(crate) fn new(
        def: &'t StructType,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> StructValueDeserializer<'t, 'de> {
        StructValueDeserializer { def, defs, msg }
    }
}

impl<'t, 'de> Deserializer<'de> for StructValueDeserializer<'t, 'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(StructAccess::new(self.def, self.defs, self.msg))
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(StructAccess::new(self.def, self.defs, self.msg))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
