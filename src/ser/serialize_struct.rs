use std::borrow::Cow;
use std::io::Write;

use serde::ser::{self, Serialize};
use serde::de::value::Error;

use ::internal::utils::Bow;
use ::internal::ser::{SerializationCtx, FieldValueSerializer};
use ::internal::ser::SerializeStructValue;
use ::internal::types::{TypeId, WireType, CommonType, StructType, FieldType};

pub struct SerializeStruct<'t, W> {
    inner: SerializeStructValue<'t, 't>,
    out: W
}

impl<'t, W: Write> SerializeStruct<'t, W> {
    pub(crate) fn new(type_id: TypeId, ctx: SerializationCtx<'t>, out: W) -> Result<Self, Error> {
        Ok(SerializeStruct {
            inner: SerializeStructValue::new(Bow::Owned(ctx), type_id)?,
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
        ok.ctx.finish(type_id, self.out)
    }
}
