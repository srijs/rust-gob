use std::io::Write;

use serde::de::value::Error;
use serde::ser::{self, Impossible};
use serde::Serialize;

use internal::gob::{Message, Writer};
use internal::types::TypeId;
use internal::utils::Bow;

use schema::Schema;

mod serialize_struct;
pub(crate) use self::serialize_struct::SerializeStructValue;
mod serialize_seq;
pub(crate) use self::serialize_seq::SerializeSeqValue;
mod serialize_tuple;
pub(crate) use self::serialize_tuple::SerializeTupleValue;
mod serialize_map;
pub(crate) use self::serialize_map::SerializeMapValue;
mod serialize_variant;
pub(crate) use self::serialize_variant::{SerializeStructVariantValue, SerializeVariantValue};
mod serialize_wire_types;
pub(crate) use self::serialize_wire_types::SerializeWireTypes;

pub(crate) struct SerializationOk<'t> {
    pub ctx: SerializationCtx<'t>,
    pub is_empty: bool,
}

pub(crate) struct SerializationCtx<'t> {
    pub schema: Bow<'t, Schema>,
    pub value: Message<Vec<u8>>,
}

impl<'t> SerializationCtx<'t> {
    pub(crate) fn new() -> Self {
        SerializationCtx::with_schema(Bow::Owned(Schema::new()))
    }

    pub(crate) fn with_schema(schema: Bow<'t, Schema>) -> Self {
        SerializationCtx {
            schema,
            value: Message::new(Vec::new()),
        }
    }

    pub(crate) fn flush<W: Write>(
        &mut self,
        type_id: TypeId,
        mut writer: Writer<W>,
    ) -> Result<(), Error> {
        self.schema.write_pending(writer.borrow_mut())?;
        writer.write_section(type_id.0, self.value.get_ref())?;
        Ok(())
    }
}

pub(crate) struct FieldValueSerializer<'t> {
    pub ctx: SerializationCtx<'t>,
    pub type_id: TypeId,
}

impl<'t> FieldValueSerializer<'t> {
    fn check_type(&self, got: TypeId) -> Result<(), Error> {
        if self.type_id != got {
            Err(ser::Error::custom(format!(
                "type id mismatch: got {}, expected {}",
                got.0, self.type_id.0
            )))
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
    type SerializeMap = SerializeMapValue<'t>;
    type SerializeStruct = SerializeStructValue<'t>;
    type SerializeStructVariant = SerializeStructVariantValue<'t>;

    #[inline]
    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::BOOL)?;
        self.ctx.value.write_bool(v)?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: v == true,
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
            is_empty: v == 0,
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
            is_empty: v == 0,
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
            is_empty: false,
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
            is_empty: v.len() == 0,
        })
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.check_type(TypeId::BYTES)?;
        self.ctx.value.write_bytes(v)?;
        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: v.len() == 0,
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let ser = SerializeVariantValue::new(self.ctx, self.type_id, variant_index)?;
        ser.serialize_newtype(value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        SerializeSeqValue::new(self.ctx, len, self.type_id)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        SerializeTupleValue::homogeneous(self.ctx, self.type_id)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("tuple variants not implemented yet"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        SerializeMapValue::new(self.ctx, len, self.type_id)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        SerializeStructValue::new(self.ctx, self.type_id)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let ser = SerializeVariantValue::new(self.ctx, self.type_id, variant_index)?;
        ser.serialize_struct()
    }
}
