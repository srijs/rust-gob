use std::io::Write;

use serde::de::value::Error;
use serde::ser::{self, Serialize};

use internal::utils::Bow;
use internal::gob::Stream;
use internal::ser::{SerializationCtx, SerializeMapValue};
use internal::types::TypeId;
use schema::Schema;

pub struct SerializeMap<'t, W> {
    inner: SerializeMapValue<Bow<'t, Schema>>,
    out: Stream<W>,
}

impl<'t, W: Write> SerializeMap<'t, W> {
    pub(crate) fn new(
        len: Option<usize>,
        type_id: TypeId,
        ctx: SerializationCtx<Bow<'t, Schema>>,
        out: Stream<W>,
    ) -> Result<Self, Error> {
        Ok(SerializeMap {
            inner: SerializeMapValue::new(ctx, len, type_id)?,
            out,
        })
    }
}

impl<'t, W: Write> ser::SerializeMap for SerializeMap<'t, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_key(key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let type_id = self.inner.type_id();
        let mut ok = self.inner.end()?;
        ok.ctx.flush(type_id, self.out)
    }
}
