use std::borrow::Borrow;

use serde::ser::{self, Serialize};
use serde_schema::types::Type;

use error::Error;
use internal::types::TypeId;
use schema::Schema;

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeSeqValue<S> {
    needs_init: bool,
    ctx: SerializationCtx<S>,
    len: usize,
    elem: TypeId,
}

impl<S: Borrow<Schema>> SerializeSeqValue<S> {
    pub(crate) fn new(
        ctx: SerializationCtx<S>,
        ser_len: Option<usize>,
        type_id: TypeId,
    ) -> Result<Self, Error> {
        let (len, elem) = if let Some(schema_type) = ctx.schema.borrow().lookup(type_id) {
            if let &Type::Seq(ref seq_type) = &*schema_type {
                if let Some(len) = seq_type.len().or(ser_len) {
                    (len, *seq_type.element_type())
                } else {
                    return Err(ser::Error::custom(
                        "sequences without known length not supported",
                    ));
                }
            } else {
                return Err(ser::Error::custom("schema mismatch, not a sequence"));
            }
        } else {
            return Err(ser::Error::custom("type not found"));
        };

        Ok(SerializeSeqValue {
            needs_init: true,
            ctx,
            len,
            elem,
        })
    }
}

impl<S: Borrow<Schema>> ser::SerializeSeq for SerializeSeqValue<S> {
    type Ok = SerializationOk<S>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.needs_init {
            self.ctx.value.write_uint(self.len as u64);
            self.needs_init = false;
        }
        let type_id = self.elem;
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
