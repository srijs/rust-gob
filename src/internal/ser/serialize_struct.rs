use std::borrow::Borrow;

use serde::ser::{self, Serialize};
use serde_schema::types::{StructField, Type};

use error::Error;
use internal::types::TypeId;
use schema::Schema;

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeStructValue<S> {
    ctx: Option<SerializationCtx<S>>,
    type_id: TypeId,
    fields: Vec<StructField<TypeId>>,
    current_field_idx: usize,
    last_serialized_field_idx: i64,
}

impl<S: Borrow<Schema>> SerializeStructValue<S> {
    pub(crate) fn new(ctx: SerializationCtx<S>, type_id: TypeId) -> Result<Self, Error> {
        let fields;
        if let Some(&Type::Struct(ref struct_type)) = ctx.schema.borrow().lookup(type_id) {
            fields = struct_type.fields().to_vec();
        } else {
            return Err(ser::Error::custom("schema mismatch, not a struct"));
        }
        Ok(SerializeStructValue::from_parts(ctx, type_id, fields))
    }

    pub(crate) fn from_parts(
        ctx: SerializationCtx<S>,
        type_id: TypeId,
        fields: Vec<StructField<TypeId>>,
    ) -> Self {
        SerializeStructValue {
            ctx: Some(ctx),
            type_id,
            fields,
            current_field_idx: 0,
            last_serialized_field_idx: -1,
        }
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<S: Borrow<Schema>> ser::SerializeStruct for SerializeStructValue<S> {
    type Ok = SerializationOk<S>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ctx = self.ctx.take().unwrap();
        let pre_pos = ctx.value.get_ref().len();
        let field_delta = self.current_field_idx as i64 - self.last_serialized_field_idx;
        ctx.value.write_uint(field_delta as u64)?;
        let mut ok = {
            let de = FieldValueSerializer {
                ctx,
                type_id: *self.fields[self.current_field_idx].field_type(),
            };
            value.serialize(de)?
        };

        if !ok.is_empty {
            self.last_serialized_field_idx = self.current_field_idx as i64;
        } else {
            // reset the buffer to the previous position
            ok.ctx.value.get_mut().truncate(pre_pos);
        }

        self.current_field_idx += 1;
        self.ctx = Some(ok.ctx);

        Ok(())
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        self.current_field_idx += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut ctx = self.ctx.take().unwrap();
        ctx.value.write_uint(0)?;

        Ok(SerializationOk {
            ctx,
            is_empty: false,
        })
    }
}
