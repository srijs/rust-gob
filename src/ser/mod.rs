//! Serialization

use std::io::Write;

use serde::ser::{self, Impossible};
use serde::Serialize;
use serde_schema::SchemaSerialize;

use internal::ser::{FieldValueSerializer, SerializationCtx, SerializeVariantValue};
use internal::utils::Bow;

use error::Error;
pub use schema::{Schema, TypeId};

mod output;
pub use self::output::{Output, OutputBuffer, OutputPart, OutputWrite};

mod serialize_struct;
pub use self::serialize_struct::SerializeStruct;
mod serialize_seq;
pub use self::serialize_seq::SerializeSeq;
mod serialize_tuple;
pub use self::serialize_tuple::SerializeTuple;
mod serialize_map;
pub use self::serialize_map::SerializeMap;
mod serialize_struct_variant;
pub use self::serialize_struct_variant::SerializeStructVariant;

/// Serializes a single value.
pub struct Serializer<'t, O> {
    ctx: SerializationCtx<Bow<'t, Schema>>,
    type_id: TypeId,
    out: O,
}

/// Serializes a stream of values.
pub struct StreamSerializer<O> {
    schema: Schema,
    out: O,
}

impl StreamSerializer<OutputBuffer> {
    /// Create a new stream serializer that writes into a buffer.
    pub fn new_with_buffer() -> Self {
        let buffer = OutputBuffer::new();
        StreamSerializer::new(buffer)
    }
}

impl<W: Write> StreamSerializer<OutputWrite<W>> {
    /// Create a new stream serializer with the provided `Write` output.
    pub fn new_with_write(w: W) -> Self {
        StreamSerializer::new(OutputWrite::new(w))
    }
}

impl<O> StreamSerializer<O> {
    fn new(out: O) -> StreamSerializer<O> {
        let schema = Schema::new();
        StreamSerializer { schema, out }
    }

    pub fn schema_mut(&mut self) -> &mut Schema {
        &mut self.schema
    }

    pub fn serializer<'a>(&'a mut self, id: TypeId) -> Result<Serializer<'a, &'a mut O>, Error> {
        let ctx = SerializationCtx::with_schema(Bow::Borrowed(&mut self.schema));
        Ok(Serializer {
            type_id: id,
            ctx,
            out: &mut self.out,
        })
    }

    /// Serialize a value onto the stream.
    pub fn serialize<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: SchemaSerialize,
        O: Output,
    {
        let type_id = T::schema_register(&mut self.schema)?;
        self.serialize_with_type_id(type_id, value)
    }

    pub fn serialize_with_type_id<T>(&mut self, type_id: TypeId, value: &T) -> Result<(), Error>
    where
        T: Serialize,
        O: Output,
    {
        value.serialize(self.serializer(type_id)?)
    }

    pub fn get_ref(&self) -> &O {
        &self.out
    }

    pub fn get_mut(&mut self) -> &mut O {
        &mut self.out
    }

    pub fn into_inner(self) -> O {
        self.out
    }
}

impl<'t, O: Output> ser::Serializer for Serializer<'t, O> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeSeq<'t, O>;
    type SerializeTuple = SerializeTuple<'t, O>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = SerializeMap<'t, O>;
    type SerializeStruct = SerializeStruct<'t, O>;
    type SerializeStructVariant = SerializeStructVariant<'t, O>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_bool(v)?
        };
        ok.ctx.flush(self.out)
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
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_i64(v)?
        };
        ok.ctx.flush(self.out)
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
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_u64(v)?
        };
        ok.ctx.flush(self.out)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_f64(v)?
        };
        ok.ctx.flush(self.out)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_str(v)?
        };
        ok.ctx.flush(self.out)
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_bytes(v)?
        };
        ok.ctx.flush(self.out)
    }

    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_none()?
        };
        ok.ctx.flush(self.out)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
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
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.ctx.value.write_int(self.type_id.0);
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_newtype_variant(name, variant_index, variant, value)?
        };
        ok.ctx.flush(self.out)
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        SerializeSeq::new(len, self.type_id, self.ctx, self.out)
    }

    fn serialize_tuple(mut self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        SerializeTuple::homogeneous(self.type_id, self.ctx, self.out)
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
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        self.ctx.value.write_uint(0);
        SerializeMap::new(len, self.type_id, self.ctx, self.out)
    }

    fn serialize_struct(
        mut self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        Ok(SerializeStruct::new(self.type_id, self.ctx, self.out)?)
    }

    fn serialize_struct_variant(
        mut self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.ctx.value.write_int(self.type_id.0);
        let inner =
            SerializeVariantValue::new(self.ctx, self.type_id, variant_index)?.serialize_struct()?;
        SerializeStructVariant::new(inner, self.out)
    }
}
