use std::borrow::Borrow;

use serde::ser::{self, Serialize};
use serde_schema::types::Type;

use error::Error;
use internal::types::TypeId;
use schema::Schema;

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeMapValue<S> {
    needs_init: bool,
    ctx: Option<SerializationCtx<S>>,
    type_id: TypeId,
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
        let (len, key, value) = match ctx.schema.borrow().lookup(type_id) {
            Some(&Type::Map(ref map_type)) => {
                if let Some(len) = ser_len {
                    (len, *map_type.key_type(), *map_type.value_type())
                } else {
                    return Err(ser::Error::custom(
                        "maps without known length not supported",
                    ));
                }
            }
            _ => {
                return Err(ser::Error::custom("schema mismatch, not a map"));
            }
        };

        Ok(SerializeMapValue {
            needs_init: true,
            ctx: Some(ctx),
            type_id,
            len,
            key,
            value,
        })
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<S: Borrow<Schema>> ser::SerializeMap for SerializeMapValue<S> {
    type Ok = SerializationOk<S>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ctx = self.ctx.take().unwrap();
        if self.needs_init {
            ctx.value.write_uint(self.len as u64);
            self.needs_init = false;
        }
        let de = FieldValueSerializer {
            ctx,
            type_id: self.key,
        };
        let ok = key.serialize(de)?;
        self.ctx = Some(ok.ctx);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let ctx = self.ctx.take().unwrap();
        let de = FieldValueSerializer {
            ctx,
            type_id: self.value,
        };
        let ok = value.serialize(de)?;
        self.ctx = Some(ok.ctx);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut ctx = self.ctx.take().unwrap();
        let is_empty = self.len == 0;

        if is_empty {
            ctx.value.write_uint(0);
        }

        Ok(SerializationOk { ctx, is_empty })
    }
}
