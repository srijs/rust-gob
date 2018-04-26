use std::collections::HashMap;
use std::io::{Cursor, Write};

use serde::Serialize;
use serde::ser::{self, Impossible};
use serde::de::value::Error;

use ::gob::Message;
use ::types::{WireType, TypeId, TypeDefs};

struct SerializationCtx {
    pub type_defs: TypeDefs,
    pub value: Message<Vec<u8>>
}

impl SerializationCtx {
    pub(crate) fn new() -> SerializationCtx {
        SerializationCtx {
            type_defs: TypeDefs::new(),
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

    pub(crate) fn finish<W: Write>(self, type_id: TypeId, mut out: W) -> Result<(), Error> {
        let mut wire_type_ctx = SerializationCtx::new();
        for wire_type in self.type_defs.custom() {
            {
                let ser = FieldValueSerializer { ctx: &mut wire_type_ctx };
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

pub struct Serializer<W> {
    ctx: SerializationCtx,
    out: W
}

impl<W: Write> Serializer<W> {
    pub fn new(out: W) -> Serializer<W> {
        Serializer { ctx: SerializationCtx::new(), out }
    }
}

impl<W: Write> ser::Serializer for Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        {
            let ser = FieldValueSerializer { ctx: &mut self.ctx };
            ser.serialize_bool(v)?;
        }
        self.ctx.finish(TypeId::BOOL, self.out)
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
        {
            let ser = FieldValueSerializer { ctx: &mut self.ctx };
            ser.serialize_i64(v)?;
        }
        self.ctx.finish(TypeId::INT, self.out)
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
        {
            let ser = FieldValueSerializer { ctx: &mut self.ctx };
            ser.serialize_u64(v)?;
        }
        self.ctx.finish(TypeId::UINT, self.out)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        {
            let ser = FieldValueSerializer { ctx: &mut self.ctx };
            ser.serialize_f64(v)?;
        }
        self.ctx.finish(TypeId::FLOAT, self.out)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        {
            let ser = FieldValueSerializer { ctx: &mut self.ctx };
            ser.serialize_str(v)?;
        }
        self.ctx.finish(TypeId::STRING, self.out)
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;
        {
            let ser = FieldValueSerializer { ctx: &mut self.ctx };
            ser.serialize_bytes(v)?;
        }
        self.ctx.finish(TypeId::BYTES, self.out)
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
        Err(ser::Error::custom("not implemented yet"))
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

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }
}

pub(crate) struct FieldValueSerializer<'t> {
    ctx: &'t mut SerializationCtx
}

impl<'t> ser::Serializer for FieldValueSerializer<'t> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_bool(v)?;
        Ok(())
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

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_int(v)?;
        Ok(())
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

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_float(v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_bytes(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_bytes(v)?;
        Ok(())
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
        Err(ser::Error::custom("not implemented yet"))
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

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("not implemented yet"))
    }
}
