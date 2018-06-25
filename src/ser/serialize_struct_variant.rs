use serde::ser::{self, Serialize};

use error::Error;
use internal::ser::SerializeStructVariantValue;
use internal::utils::Bow;
use schema::Schema;

use super::output::Output;

pub struct SerializeStructVariant<'t, O> {
    inner: SerializeStructVariantValue<Bow<'t, Schema>>,
    out: O,
}

impl<'t, O: Output> SerializeStructVariant<'t, O> {
    pub(crate) fn new(
        inner: SerializeStructVariantValue<Bow<'t, Schema>>,
        out: O,
    ) -> Result<Self, Error> {
        Ok(SerializeStructVariant { inner, out })
    }
}

impl<'t, O: Output> ser::SerializeStructVariant for SerializeStructVariant<'t, O> {
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
        let mut ok = self.inner.end()?;
        ok.ctx.flush(self.out)
    }
}
