use std::borrow::Cow;
use std::io::Write;

use serde::ser::{self, Serialize};
use serde::de::value::Error;

use ::internal::utils::Bow;
use ::internal::types::{TypeId, WireType, CommonType, SliceType};

use super::{SerializationOk, SerializationCtx, FieldValueSerializer};

pub(crate) struct SerializeSeqValue<'c, 't> where 't: 'c {
    needs_init: bool,
    ctx: Bow<'c, SerializationCtx<'t>>,
    type_id: TypeId,
    len: usize,
    elem: TypeId
}

impl<'c, 't> SerializeSeqValue<'c, 't> {
    pub(crate) fn new(ctx: Bow<'c, SerializationCtx<'t>>, len: Option<usize>, type_id: TypeId) -> Result<Self, Error> {
        let (len, id, elem) = match ctx.schema.types.lookup(type_id) {
            Some(&WireType::Slice(ref slice_type)) => {
                if let Some(len) = len {
                    (len, slice_type.common.id, slice_type.elem)
                } else {
                    return Err(ser::Error::custom("sequences without known length not supported"));
                }
            },
            Some(&WireType::Array(ref array_type)) => {
                (array_type.len as usize, array_type.common.id, array_type.elem)
            },
            _ => {
                return Err(ser::Error::custom("schema mismatch, not a sequence"));
            }
        };

        Ok(SerializeSeqValue {
            needs_init: true,
            ctx,
            type_id: id,
            len,
            elem
        })
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<'c, 't> ser::SerializeSeq for SerializeSeqValue<'c, 't> {
    type Ok = SerializationOk<'c, 't>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        if self.needs_init {
            self.ctx.value.write_uint(self.len as u64)?;
            self.needs_init = false;
        }
        let de = FieldValueSerializer {
            ctx: Bow::Borrowed(&mut self.ctx),
            type_id: self.elem
        };
        value.serialize(de)?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let is_empty = self.len == 0;

        if is_empty {
            self.ctx.value.write_uint(0)?;
        }

        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: is_empty
        })
    }
}
