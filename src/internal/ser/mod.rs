use std::io::{Cursor, Write};

use serde::Serialize;
use serde::ser::{self, Impossible};
use serde::de::value::Error;

use ::internal::utils::Bow;
use ::internal::gob::{Message, Writer};
use ::internal::types::TypeId;

use ::schema::Schema;

mod serialize_struct;
pub(crate) use self::serialize_struct::SerializeStructValue;
mod serialize_seq;
pub(crate) use self::serialize_seq::SerializeSeqValue;
mod serialize_tuple;
pub(crate) use self::serialize_tuple::SerializeTupleValue;

pub(crate) struct SerializationOk<'t> {
    pub ctx: SerializationCtx<'t>,
    pub is_empty: bool
}

pub(crate) struct SerializationCtx<'t> {
    pub schema: Bow<'t, Schema>,
    pub value: Message<Vec<u8>>
}

impl<'t> SerializationCtx<'t> {
    pub(crate) fn new() -> Self {
        SerializationCtx::with_schema(Bow::Owned(Schema::new()))
    }

    pub(crate) fn with_schema(schema: Bow<'t, Schema>) -> Self {
        SerializationCtx {
            schema,
            value: Message::new(Vec::new())
        }
    }

    pub(crate) fn flush<W: Write>(&mut self, type_id: TypeId, mut writer: Writer<W>) -> Result<(), Error> {
        let mut wire_type_ctx = SerializationCtx::new();
        let mut last_sent_type_id = self.schema.last_sent_type_id;
        for wire_type in self.schema.types.custom(last_sent_type_id) {
            {
                let ser = FieldValueSerializer {
                    ctx: wire_type_ctx,
                    type_id: TypeId::WIRE_TYPE
                };
                let ok = wire_type.serialize(ser)?;
                wire_type_ctx = ok.ctx;
            }
            writer.write_section(-wire_type.common().id.0,
                wire_type_ctx.value.get_ref())?;
            wire_type_ctx.value.get_mut().clear();
            last_sent_type_id = Some(wire_type.common().id);
        }
        self.schema.last_sent_type_id = last_sent_type_id;
        writer.write_section(type_id.0,
            self.value.get_ref())?;
        Ok(())
    }
}

pub(crate) struct FieldValueSerializer<'t> {
    pub ctx: SerializationCtx<'t>,
    pub type_id: TypeId
}

impl<'t> FieldValueSerializer<'t> {
    fn check_type(&self, got: TypeId) -> Result<(), Error> {
        if self.type_id != got {
            Err(ser::Error::custom(format!("type id mismatch: got {}, expected {}",
                got.0, self.type_id.0)))
        } else {
            Ok(())
        }
    }
}

impl<'t> ser::Serializer for FieldValueSerializer<'t> {
    type Ok = SerializationOk<'t>;
    type Error = Error;

    type SerializeSeq = SerializeSeqValue<'t>;
    type SerializeTuple = SerializeTupleValue<'t>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = SerializeStructValue<'t>;
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

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        SerializeTupleValue::homogeneous(self.ctx, self.type_id)
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
