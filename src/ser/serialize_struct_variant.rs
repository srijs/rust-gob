use std::io::Write;

use serde::de::value::Error;
use serde::ser::{self, Serialize};

use internal::gob::Stream;
use internal::ser::SerializeStructVariantValue;
use internal::utils::Bow;
use schema::Schema;

pub struct SerializeStructVariant<'t, W> {
    inner: SerializeStructVariantValue<Bow<'t, Schema>>,
    out: Stream<W>,
}

impl<'t, W: Write> SerializeStructVariant<'t, W> {
    pub(crate) fn new(
        inner: SerializeStructVariantValue<Bow<'t, Schema>>,
        out: Stream<W>,
    ) -> Result<Self, Error> {
        Ok(SerializeStructVariant { inner, out })
    }
}

impl<'t, W: Write> ser::SerializeStructVariant for SerializeStructVariant<'t, W> {
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

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let type_id = self.inner.type_id();
        let mut ok = self.inner.end()?;
        ok.ctx.flush(type_id, self.out)
    }
}
