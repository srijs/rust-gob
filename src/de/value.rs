use std::io::Cursor;

use serde;
use serde::de::Visitor;
use serde::de::value::Error;

use ::gob::Message;
use ::types::{TypeId, TypeDefs, WireType};

use super::struct_map::StructMapAccess;
use super::slice_seq::SliceSeqAccess;

pub(crate) struct ValueDeserializer<'t, 'de> where 'de: 't {
    type_id: TypeId,
    defs: &'t TypeDefs,
    msg: &'t mut Message<Cursor<&'de [u8]>>
}

impl<'t, 'de> ValueDeserializer<'t, 'de> {
    pub fn new(type_id: TypeId, defs: &'t TypeDefs, msg: &'t mut Message<Cursor<&'de [u8]>>) -> ValueDeserializer<'t, 'de> {
        ValueDeserializer {
            type_id, defs, msg
        }
    }

    fn deserialize_byte_slice(&mut self) -> Result<&'de [u8], Error> {
        let len = self.msg.read_bytes_len()?;
        let pos = self.msg.get_ref().position() as usize;
        let bytes = &self.msg.get_ref().get_ref()[pos..pos+len];
        Ok(bytes)
    }

    fn deserialize_str_slice(&mut self) -> Result<&'de str, Error> {
        let bytes = self.deserialize_byte_slice()?;
        ::std::str::from_utf8(bytes)
            .map_err(|err| serde::de::Error::custom(err))
    }
}

impl<'t, 'de> serde::Deserializer<'de> for ValueDeserializer<'t, 'de> {
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
            _ => {
                if let Some(wire_type) = self.defs.lookup(self.type_id) {
                    match wire_type {
                        &WireType::Struct(ref struct_type) => {
                            let access = StructMapAccess::new(struct_type, self.defs, &mut self.msg);
                            visitor.visit_map(access)
                        },
                        &WireType::Slice(ref slice_type) => {
                            let access = SliceSeqAccess::new(slice_type, self.defs, &mut self.msg)?;
                            visitor.visit_seq(access)
                        }
                        _ => unimplemented!()
                    }
                } else {
                    Err(serde::de::Error::custom(format!("unknown type id {:?}", self.type_id)))
                }
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
