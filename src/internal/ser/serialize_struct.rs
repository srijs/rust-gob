use std::borrow::Cow;
use std::io::Write;

use serde::ser::{self, Serialize};
use serde::de::value::Error;

use ::internal::utils::Bow;
use ::internal::types::{TypeId, WireType, CommonType, StructType, FieldType};

use super::{SerializationOk, SerializationCtx, FieldValueSerializer};

pub(crate) struct SerializeStructValue<'c, 't> where 't: 'c {
    ctx: Bow<'c, SerializationCtx<'t>>,
    type_id: TypeId,
    fields: Vec<FieldType>,
    current_field_idx: usize,
    last_serialized_field_idx: i64
}

impl<'c, 't> SerializeStructValue<'c, 't> {
    pub(crate) fn new(ctx: Bow<'c, SerializationCtx<'t>>, type_id: TypeId,) -> Result<Self, Error> {
        let fields;
        if let Some(&WireType::Struct(ref struct_type)) = ctx.schema.types.lookup(type_id) {
            fields = struct_type.fields.to_vec();
        } else {
            return Err(ser::Error::custom("schema mismatch, not a struct"));
        }
        Ok(SerializeStructValue {
            ctx,
            type_id,
            fields,
            current_field_idx: 0,
            last_serialized_field_idx: -1
        })
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<'c, 't> ser::SerializeStruct for SerializeStructValue<'c, 't> {
    type Ok = SerializationOk<'c, 't>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        let pre_pos = self.ctx.value.get_ref().len();
        let field_delta = self.current_field_idx as i64 - self.last_serialized_field_idx;
        self.ctx.value.write_uint(field_delta as u64)?;
        let is_empty = {
            let de = FieldValueSerializer {
                ctx: Bow::Borrowed(&mut self.ctx),
                type_id: self.fields[self.current_field_idx].id
            };
            value.serialize(de)?.is_empty
        };

        if !is_empty {
            self.last_serialized_field_idx = self.current_field_idx as i64;
        } else {
            // reset the buffer to the previous position
            self.ctx.value.get_mut().truncate(pre_pos);
        }

        self.current_field_idx += 1;

        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.current_field_idx += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0)?;

        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: false
        })
    }
}
