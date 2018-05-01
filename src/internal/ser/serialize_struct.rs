use serde::ser::{self, Serialize};
use serde::de::value::Error;
use serde_schema::{Type, StructField};

use ::internal::types::{TypeId, WireType, FieldType};

use super::{SerializationOk, SerializationCtx, FieldValueSerializer};

pub(crate) struct SerializeStructValue<'t> {
    ctx: SerializationCtx<'t>,
    type_id: TypeId,
    fields: Vec<StructField<TypeId>>,
    current_field_idx: usize,
    last_serialized_field_idx: i64
}

impl<'t> SerializeStructValue<'t> {
    pub(crate) fn new(ctx: SerializationCtx<'t>, type_id: TypeId) -> Result<Self, Error> {
        let struct_fields;
        if let Some(&Type::Struct { ref name, ref fields }) = ctx.schema.lookup(type_id) {
            struct_fields = fields.to_vec();
        } else {
            return Err(ser::Error::custom("schema mismatch, not a struct"));
        }
        Ok(SerializeStructValue {
            ctx,
            type_id,
            fields: struct_fields,
            current_field_idx: 0,
            last_serialized_field_idx: -1
        })
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<'t> ser::SerializeStruct for SerializeStructValue<'t> {
    type Ok = SerializationOk<'t>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        let pre_pos = self.ctx.value.get_ref().len();
        let field_delta = self.current_field_idx as i64 - self.last_serialized_field_idx;
        self.ctx.value.write_uint(field_delta as u64)?;
        let ctx = ::std::mem::replace(&mut self.ctx, SerializationCtx::new());
        let ok = {
            let de = FieldValueSerializer {
                ctx,
                type_id: self.fields[self.current_field_idx].id
            };
            value.serialize(de)?
        };
        self.ctx = ok.ctx;

        if !ok.is_empty {
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
