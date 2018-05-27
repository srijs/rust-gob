//! Serialization

use std::io::Write;

use serde::Serialize;
use serde::de::value::Error;
use serde::ser::{self, Impossible};
use serde_schema::SchemaSerialize;

use internal::gob::Stream;
use internal::ser::{FieldValueSerializer, SerializationCtx, SerializeVariantValue};
use internal::utils::Bow;

pub use schema::{Schema, TypeId};

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
pub struct Serializer<'t, W> {
    ctx: SerializationCtx<Bow<'t, Schema>>,
    type_id: TypeId,
    out: Stream<W>,
}

impl<'t, W> Serializer<'t, W> {
    /// Create a new serializer for a value of the specified type,
    /// with the provided output sink.
    pub fn new(id: TypeId, out: W) -> Serializer<'t, W> {
        let ctx = SerializationCtx::with_schema(Bow::Owned(Schema::new()));
        Serializer::with_context(id, ctx, Stream::new(out))
    }

    /// Create a new serializer for a value of the specified type,
    /// with the provided schema and output sink.
    pub fn with_schema(id: TypeId, schema: &'t mut Schema, out: W) -> Serializer<'t, W> {
        let ctx = SerializationCtx::with_schema(Bow::Borrowed(schema));
        Serializer::with_context(id, ctx, Stream::new(out))
    }

    fn with_context(id: TypeId, ctx: SerializationCtx<Bow<'t, Schema>>, out: Stream<W>) -> Self {
        Serializer {
            ctx,
            type_id: id,
            out,
        }
    }
}

/// Serializes a stream of values.
pub struct StreamSerializer<W> {
    schema: Schema,
    out: Stream<W>,
}

impl<W> StreamSerializer<W> {
    /// Create a new stream serializer with the provided output sink.
    pub fn new(out: W) -> StreamSerializer<W> {
        let schema = Schema::new();
        StreamSerializer {
            schema,
            out: Stream::new(out),
        }
    }

    /// Serialize a value onto the stream.
    pub fn serialize<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: SchemaSerialize,
        W: Write,
    {
        value.schema_serialize(self)
    }

    pub fn get_ref(&self) -> &W {
        self.out.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.out.get_mut()
    }

    pub fn into_inner(self) -> W {
        self.out.into_inner()
    }
}

impl<'t, W: Write> ::serde_schema::SchemaSerializer for &'t mut StreamSerializer<W> {
    type Ok = ();
    type Error = Error;
    type TypeId = TypeId;
    type Schema = Schema;
    type Serializer = Serializer<'t, &'t mut W>;

    fn schema_mut(&mut self) -> &mut Self::Schema {
        &mut self.schema
    }

    fn serializer(self, id: TypeId) -> Result<Self::Serializer, Self::Error> {
        let ctx = SerializationCtx::with_schema(Bow::Borrowed(&mut self.schema));
        Ok(Serializer::with_context(id, ctx, self.out.borrow_mut()))
    }
}

impl<'t, W: Write> ser::Serializer for Serializer<'t, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeSeq<'t, W>;
    type SerializeTuple = SerializeTuple<'t, W>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = SerializeMap<'t, W>;
    type SerializeStruct = SerializeStruct<'t, W>;
    type SerializeStructVariant = SerializeStructVariant<'t, W>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_bool(v)?
        };
        ok.ctx.flush(self.type_id, self.out)
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
        self.ctx.value.write_uint(0)?;
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_i64(v)?
        };
        ok.ctx.flush(self.type_id, self.out)
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
        self.ctx.value.write_uint(0)?;
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_u64(v)?
        };
        ok.ctx.flush(self.type_id, self.out)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_f64(v)?
        };
        ok.ctx.flush(self.type_id, self.out)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_str(v)?
        };
        ok.ctx.flush(self.type_id, self.out)
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_bytes(v)?
        };
        ok.ctx.flush(self.type_id, self.out)
    }

    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_none()?
        };
        ok.ctx.flush(self.type_id, self.out)
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
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut ok = {
            let ser = FieldValueSerializer {
                ctx: self.ctx,
                type_id: self.type_id,
            };
            ser.serialize_newtype_variant(name, variant_index, variant, value)?
        };
        ok.ctx.flush(self.type_id, self.out)
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.ctx.value.write_uint(0)?;
        SerializeSeq::new(len, self.type_id, self.ctx, self.out)
    }

    fn serialize_tuple(mut self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.ctx.value.write_uint(0)?;
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
        self.ctx.value.write_uint(0)?;
        SerializeMap::new(len, self.type_id, self.ctx, self.out)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct::new(self.type_id, self.ctx, self.out)?)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let inner =
            SerializeVariantValue::new(self.ctx, self.type_id, variant_index)?.serialize_struct()?;
        SerializeStructVariant::new(inner, self.out)
    }
}
