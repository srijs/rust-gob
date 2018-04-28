use std::borrow::Cow;
use std::io::Write;

use serde::ser::{self, Serialize};
use serde::de::value::Error;

use ::internal::utils::Bow;
use ::internal::ser::{SerializationCtx, FieldValueSerializer};
use ::internal::ser::SerializeSeqValue;
use ::internal::types::{TypeId, WireType, CommonType, SliceType};

pub struct SerializeSeq<'t, W> {
    inner: SerializeSeqValue<'t, 't>,
    out: W
}

impl<'t, W: Write> SerializeSeq<'t, W> {
    pub(crate) fn new(len: Option<usize>, type_id: TypeId, ctx: SerializationCtx<'t>, out: W) -> Result<Self, Error> {
        Ok(SerializeSeq {
            inner: SerializeSeqValue::new(Bow::Owned(ctx), len, type_id)?,
            out
        })
    }
}

impl<'t, W: Write> ser::SerializeSeq for SerializeSeq<'t, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let type_id = self.inner.type_id();
        let mut ok = self.inner.end()?;
        ok.ctx.finish(type_id, self.out)
    }
}
