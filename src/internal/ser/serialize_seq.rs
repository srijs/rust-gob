use std::borrow::Borrow;

use serde::de::value::Error;
use serde::ser::{self, Serialize};
use serde_schema::types::Type;

use internal::types::TypeId;
use schema::Schema;

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeSeqValue<S> {
    needs_init: bool,
    ctx: Option<SerializationCtx<S>>,
    type_id: TypeId,
    len: usize,
    elem: TypeId,
}

impl<S: Borrow<Schema>> SerializeSeqValue<S> {
    pub(crate) fn new(
        ctx: SerializationCtx<S>,
        ser_len: Option<usize>,
        type_id: TypeId,
    ) -> Result<Self, Error> {
        let (len, elem) = match ctx.schema.borrow().lookup(type_id) {
            Some(&Type::Seq(ref seq_type)) => {
                if let Some(len) = seq_type.len().or(ser_len) {
                    (len, *seq_type.element_type())
                } else {
                    return Err(ser::Error::custom(
                        "sequences without known length not supported",
                    ));
                }
            }
            _ => {
                return Err(ser::Error::custom("schema mismatch, not a sequence"));
            }
        };

        Ok(SerializeSeqValue {
            needs_init: true,
            ctx: Some(ctx),
            type_id,
            len,
            elem,
        })
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<S: Borrow<Schema>> ser::SerializeSeq for SerializeSeqValue<S> {
    type Ok = SerializationOk<S>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ctx = self.ctx.take().unwrap();
        if self.needs_init {
            ctx.value.write_uint(self.len as u64)?;
            self.needs_init = false;
        }
        let de = FieldValueSerializer {
            ctx,
            type_id: self.elem,
        };
        let ok = value.serialize(de)?;
        self.ctx = Some(ok.ctx);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut ctx = self.ctx.take().unwrap();
        let is_empty = self.len == 0;

        if is_empty {
            ctx.value.write_uint(0)?;
        }

        Ok(SerializationOk { ctx, is_empty })
    }
}
