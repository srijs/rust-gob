use std::io::Write;

use serde::ser::{self, Serialize};

use error::Error;
use internal::gob::Stream;
use internal::ser::{SerializationCtx, SerializeStructValue};
use internal::types::TypeId;
use internal::utils::Bow;
use schema::Schema;

pub struct SerializeStruct<'t, W> {
    inner: SerializeStructValue<Bow<'t, Schema>>,
    out: Stream<W>,
}

impl<'t, W: Write> SerializeStruct<'t, W> {
    pub(crate) fn new(
        type_id: TypeId,
        ctx: SerializationCtx<Bow<'t, Schema>>,
        out: Stream<W>,
    ) -> Result<Self, Error> {
        Ok(SerializeStruct {
            inner: SerializeStructValue::new(ctx, type_id)?,
            out,
        })
    }
}

impl<'t, W: Write> ser::SerializeStruct for SerializeStruct<'t, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_field(key, value)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.inner.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let type_id = self.inner.type_id();
        let mut ok = self.inner.end()?;
        ok.ctx.flush(type_id, self.out)
    }
}
