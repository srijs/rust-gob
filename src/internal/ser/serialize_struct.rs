use std::borrow::Borrow;

use owning_ref::OwningRef;
use serde::ser::{self, Serialize};
use serde_schema::types::{StructField, Type};

use error::Error;
use internal::types::TypeId;
use schema::{Schema, SchemaType};

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeStructValue<S> {
    ctx: SerializationCtx<S>,
    fields: OwningRef<SchemaType, [StructField<TypeId>]>,
    current_field_idx: usize,
    last_serialized_field_idx: i64,
}

impl<S: Borrow<Schema>> SerializeStructValue<S> {
    pub(crate) fn new(ctx: SerializationCtx<S>, type_id: TypeId) -> Result<Self, Error> {
        let fields;
        if let Some(schema_type) = ctx.schema.borrow().lookup(type_id) {
            fields = OwningRef::new(schema_type).try_map::<_, _, Error>(|typ| {
                if let &Type::Struct(ref struct_type) = typ {
                    Ok(struct_type.fields())
                } else {
                    Err(ser::Error::custom("schema mismatch, not a struct"))
                }
            })?;
        } else {
            return Err(ser::Error::custom("type not found"));
        }
        Ok(SerializeStructValue::from_parts(ctx, fields))
    }

    pub(crate) fn from_parts(
        ctx: SerializationCtx<S>,
        fields: OwningRef<SchemaType, [StructField<TypeId>]>,
    ) -> Self {
        SerializeStructValue {
            ctx,
            fields,
            current_field_idx: 0,
            last_serialized_field_idx: -1,
        }
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
        let pre_pos = self.ctx.value.get_ref().len();
        let field_delta = self.current_field_idx as i64 - self.last_serialized_field_idx;
        self.ctx.value.write_uint(field_delta as u64);

        let type_id = *self.fields[self.current_field_idx].field_type();
        let is_empty = self.ctx.with_borrow(|ctx| {
            let de = FieldValueSerializer { ctx, type_id };
            value.serialize(de)
        })?;

        if !is_empty {
            self.last_serialized_field_idx = self.current_field_idx as i64;
        } else {
            // reset the buffer to the previous position
            self.ctx.value.get_mut().truncate(pre_pos);
        }

        self.current_field_idx += 1;

        Ok(())
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        self.current_field_idx += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.ctx.value.write_uint(0);

        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: false,
        })
    }
}
