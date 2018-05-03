use serde::de::value::Error;
use serde::ser::{self, Serialize};
use serde_schema::types::Type;

use internal::types::TypeId;

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeSeqValue<'t> {
    needs_init: bool,
    ctx: SerializationCtx<'t>,
    type_id: TypeId,
    len: usize,
    elem: TypeId,
}

impl<'t> SerializeSeqValue<'t> {
    pub(crate) fn new(
        ctx: SerializationCtx<'t>,
        ser_len: Option<usize>,
        type_id: TypeId,
    ) -> Result<Self, Error> {
        let (len, elem) = match ctx.schema.lookup(type_id) {
            Some(&Type::Seq {
                len: seq_len,
                element,
            }) => {
                if let Some(len) = seq_len.or(ser_len) {
                    (len, element)
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
            ctx,
            type_id,
            len,
            elem,
        })
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<'t> ser::SerializeSeq for SerializeSeqValue<'t> {
    type Ok = SerializationOk<'t>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.needs_init {
            self.ctx.value.write_uint(self.len as u64)?;
            self.needs_init = false;
        }
        let ctx = ::std::mem::replace(&mut self.ctx, SerializationCtx::new());
        let de = FieldValueSerializer {
            ctx,
            type_id: self.elem,
        };
        let ok = value.serialize(de)?;
        self.ctx = ok.ctx;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let is_empty = self.len == 0;

        if is_empty {
            self.ctx.value.write_uint(0)?;
        }

        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: is_empty,
        })
    }
}
