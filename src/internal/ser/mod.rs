use std::collections::HashMap;
use std::io::{Cursor, Write};

use serde::Serialize;
use serde::ser::{self, Impossible};
use serde::de::value::Error;

use ::internal::utils::Bow;
use ::internal::gob::Message;
use ::internal::types::{WireType, TypeId, Types};

use ::Schema;

mod serialize_struct;
pub(crate) use self::serialize_struct::SerializeStructValue;
mod serialize_seq;
pub(crate) use self::serialize_seq::SerializeSeqValue;

pub(crate) struct SerializationOk<'c, 't> where 't: 'c {
    pub ctx: Bow<'c, SerializationCtx<'t>>,
    pub is_empty: bool
}

pub(crate) struct SerializationCtx<'t> {
    pub schema: Bow<'t, Schema>,
    pub value: Message<Vec<u8>>
}

impl<'t> SerializationCtx<'t> {
    pub(crate) fn new() -> SerializationCtx<'t> {
        SerializationCtx {
            schema: Bow::Owned(Schema::new()),
            value: Message::new(Vec::new())
        }
    }

    fn write<W: Write>(mut out: W, buf: &[u8]) -> Result<(), Error> {
        out.write_all(buf)
            .map_err(|err| ::serde::ser::Error::custom(err))
    }

    fn write_section<W: Write>(mut out: W, type_id: i64, buf: &[u8]) -> Result<(), Error> {
        let mut type_id_msg = Message::new(Cursor::new([0u8; 9]));
        type_id_msg.write_int(type_id)?;
        let type_id_pos = type_id_msg.get_ref().position() as usize;
        let mut len_msg = Message::new(Cursor::new([0u8; 9]));
        len_msg.write_uint((buf.len() + type_id_pos) as u64)?;
        let len_pos = len_msg.get_ref().position() as usize;
        SerializationCtx::write(&mut out, &len_msg.get_ref().get_ref()[..len_pos])?;
        SerializationCtx::write(&mut out, &type_id_msg.get_ref().get_ref()[..type_id_pos])?;
        SerializationCtx::write(out, buf)
    }

    pub(crate) fn finish<W: Write>(&mut self, type_id: TypeId, mut out: W) -> Result<(), Error> {
        let mut wire_type_ctx = SerializationCtx::new();
        for wire_type in self.schema.types.custom() {
            {
                let ser = FieldValueSerializer {
                    ctx: Bow::Borrowed(&mut wire_type_ctx),
                    type_id: TypeId::WIRE_TYPE
                };
                wire_type.serialize(ser)?;
            }
            SerializationCtx::write_section(&mut out, -wire_type.common().id.0,
                wire_type_ctx.value.get_ref())?;
            wire_type_ctx.value.get_mut().clear();
        }
        SerializationCtx::write_section(out, type_id.0,
            self.value.get_ref())
    }
}

pub(crate) struct FieldValueSerializer<'c, 't> where 't: 'c {
    pub ctx: Bow<'c, SerializationCtx<'t>>,
    pub type_id: TypeId
}

impl<'c, 't> FieldValueSerializer<'c, 't> {
    fn check_type(&self, got: TypeId) -> Result<(), Error> {
        if self.type_id != got {
            Err(ser::Error::custom(format!("type id mismatch: got {}, expected {}",
                got.0, self.type_id.0)))
        } else {
            Ok(())
        }
    }
}

impl<'c, 't> ser::Serializer for FieldValueSerializer<'c, 't> {
    type Ok = SerializationOk<'c, 't>;
    type Error = Error;

    type SerializeSeq = SerializeSeqValue<'c, 't>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = SerializeStructValue<'c, 't>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::BOOL)?;
        self.ctx.value.write_bool(v)?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: v == true
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::INT)?;
        self.ctx.value.write_int(v)?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: v == 0
        })
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::UINT)?;
        self.ctx.value.write_uint(v)?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: v == 0
        })
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::FLOAT)?;
        self.ctx.value.write_float(v)?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: false
        })
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::STRING)?;
        self.ctx.value.write_bytes(v.as_bytes())?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: v.len() == 0
        })
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::BYTES)?;
        self.ctx.value.write_bytes(v)?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: v.len() == 0
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        SerializeSeqValue::new(self.ctx, len, self.type_id)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        SerializeStructValue::new(self.ctx, self.type_id)
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }
}
