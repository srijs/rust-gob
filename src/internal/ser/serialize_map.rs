use std::borrow::Borrow;

use serde::ser::{self, Serialize};
use serde_schema::types::Type;

use error::Error;
use internal::types::TypeId;
use schema::Schema;

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeMapValue<S> {
    needs_init: bool,
    ctx: SerializationCtx<S>,
    len: usize,
    key: TypeId,
    value: TypeId,
}

impl<S: Borrow<Schema>> SerializeMapValue<S> {
    pub(crate) fn new(
        ctx: SerializationCtx<S>,
        ser_len: Option<usize>,
        type_id: TypeId,
    ) -> Result<Self, Error> {
        let (len, key, value) = if let Some(schema_type) = ctx.schema.borrow().lookup(type_id) {
            if let &Type::Map(ref map_type) = &*schema_type {
                if let Some(len) = ser_len {
                    (len, *map_type.key_type(), *map_type.value_type())
                } else {
                    return Err(ser::Error::custom(
                        "maps without known length not supported",
                    ));
                }
            } else {
                return Err(ser::Error::custom("schema mismatch, not a map"));
            }
        } else {
            return Err(ser::Error::custom("type not found"));
        };

        Ok(SerializeMapValue {
            needs_init: true,
            ctx,
            len,
            key,
            value,
        })
    }
}

impl<S: Borrow<Schema>> ser::SerializeMap for SerializeMapValue<S> {
    type Ok = SerializationOk<S>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.needs_init {
            self.ctx.value.write_uint(self.len as u64);
            self.needs_init = false;
        }
        let type_id = self.key;
        self.ctx.with_borrow(|ctx| {
            let de = FieldValueSerializer { ctx, type_id };
            key.serialize(de)
        })?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let type_id = self.value;
        self.ctx.with_borrow(|ctx| {
            let de = FieldValueSerializer { ctx, type_id };
            value.serialize(de)
        })?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let is_empty = self.len == 0;

        if is_empty {
            self.ctx.value.write_uint(0);
        }

        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty,
        })
    }
}
