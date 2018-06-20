use serde::ser::{self, Serialize};

use error::Error;
use internal::ser::{SerializationCtx, SerializeMapValue};
use internal::types::TypeId;
use internal::utils::Bow;
use schema::Schema;

use super::output::Output;

pub struct SerializeMap<'t, O> {
    inner: SerializeMapValue<Bow<'t, Schema>>,
    out: O,
}

impl<'t, O: Output> SerializeMap<'t, O> {
    pub(crate) fn new(
        len: Option<usize>,
        type_id: TypeId,
        ctx: SerializationCtx<Bow<'t, Schema>>,
        out: O,
    ) -> Result<Self, Error> {
        Ok(SerializeMap {
            inner: SerializeMapValue::new(ctx, len, type_id)?,
            out,
        })
    }
}

impl<'t, O: Output> ser::SerializeMap for SerializeMap<'t, O> {
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
