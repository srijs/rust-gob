use std::io::Write;

use serde::ser::{self, Serialize};
use serde::de::value::Error;

use ::internal::ser::{SerializationCtx, SerializeStructValue};
use ::internal::types::TypeId;
use ::internal::gob::Writer;

pub struct SerializeStruct<'t, W> {
    inner: SerializeStructValue<'t>,
    out: Writer<W>
}

impl<'t, W: Write> SerializeStruct<'t, W> {
    pub(crate) fn new(type_id: TypeId, ctx: SerializationCtx<'t>, out: Writer<W>) -> Result<Self, Error> {
        Ok(SerializeStruct {
            inner: SerializeStructValue::new(ctx, type_id)?,
            out
        })
    }
}

impl<'t, W: Write> ser::SerializeStruct for SerializeStruct<'t, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let type_id = self.inner.type_id();
        let mut ok = self.inner.end()?;
        ok.ctx.flush(type_id, self.out)
    }
}
