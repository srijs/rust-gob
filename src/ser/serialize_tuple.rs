use serde::ser::{self, Serialize};

use error::Error;
use internal::ser::{SerializationCtx, SerializeTupleValue};
use internal::types::TypeId;
use internal::utils::Bow;
use schema::Schema;

use super::output::Output;

pub struct SerializeTuple<'t, O> {
    inner: SerializeTupleValue<Bow<'t, Schema>>,
    out: O,
}

impl<'t, O: Output> SerializeTuple<'t, O> {
    pub(crate) fn homogeneous(
        type_id: TypeId,
        ctx: SerializationCtx<Bow<'t, Schema>>,
        out: O,
    ) -> Result<Self, Error> {
        Ok(SerializeTuple {
            inner: SerializeTupleValue::homogeneous(ctx, type_id)?,
            out,
        })
    }
}

impl<'t, O: Output> ser::SerializeTuple for SerializeTuple<'t, O> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let type_id = self.inner.type_id();
        let mut ok = self.inner.end()?;
        ok.ctx.flush(type_id, self.out)
    }
}
