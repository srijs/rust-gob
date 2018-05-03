use std::io::Cursor;

use serde;
use serde::de::value::Error;
use serde::de::{Deserializer, Visitor};

use internal::gob::Message;
use internal::types::{TypeId, Types, WireType};

use super::field_value::FieldValueDeserializer;
use super::struct_value::StructValueDeserializer;

pub(crate) struct ValueDeserializer<'t, 'de>
where
    'de: 't,
{
    type_id: TypeId,
    defs: &'t Types,
    msg: &'t mut Message<Cursor<&'de [u8]>>,
}

impl<'t, 'de> ValueDeserializer<'t, 'de> {
    pub fn new(
        type_id: TypeId,
        defs: &'t Types,
        msg: &'t mut Message<Cursor<&'de [u8]>>,
    ) -> ValueDeserializer<'t, 'de> {
        ValueDeserializer { type_id, defs, msg }
    }
}

impl<'t, 'de> Deserializer<'de> for ValueDeserializer<'t, 'de> {
    type Error = Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(&WireType::Struct(ref struct_type)) = self.defs.lookup(self.type_id) {
            let de = StructValueDeserializer::new(struct_type, &self.defs, &mut self.msg);
            return de.deserialize_any(visitor);
        }

        if self.msg.read_uint()? != 0 {
            return Err(serde::de::Error::custom(format!(
                "neither a singleton nor a struct value"
            )));
        }

        let de = FieldValueDeserializer::new(self.type_id, &self.defs, &mut self.msg);
        return de.deserialize_any(visitor);
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
        if let Some(&WireType::Struct(ref struct_type)) = self.defs.lookup(self.type_id) {
            let de = StructValueDeserializer::new(struct_type, &self.defs, &mut self.msg);
            return de.deserialize_enum(name, variants, visitor);
        }

        if self.msg.read_uint()? != 0 {
            return Err(serde::de::Error::custom(format!(
                "neither a singleton nor a struct value"
            )));
        }

        let de = FieldValueDeserializer::new(self.type_id, &self.defs, &mut self.msg);
        return de.deserialize_enum(name, variants, visitor);
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
