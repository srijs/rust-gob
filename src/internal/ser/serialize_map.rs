use serde::de::value::Error;
use serde::ser::{self, Serialize};
use serde_schema::types::Type;

use internal::types::TypeId;

use super::{FieldValueSerializer, SerializationCtx, SerializationOk};

pub(crate) struct SerializeMapValue<'t> {
    needs_init: bool,
    ctx: SerializationCtx<'t>,
    type_id: TypeId,
    len: usize,
    key: TypeId,
    value: TypeId,
}

impl<'t> SerializeMapValue<'t> {
    pub(crate) fn new(
        ctx: SerializationCtx<'t>,
        ser_len: Option<usize>,
        type_id: TypeId,
    ) -> Result<Self, Error> {
        let (len, key, value) = match ctx.schema.lookup(type_id) {
            Some(&Type::Map(ref map_type)) => {
                if let Some(len) = ser_len {
                    (len, *map_type.key_type(), *map_type.value_type())
                } else {
                    return Err(ser::Error::custom(
                        "maps without known length not supported",
                    ));
                }
            }
            _ => {
                return Err(ser::Error::custom("schema mismatch, not a map"));
            }
        };

        Ok(SerializeMapValue {
            needs_init: true,
            ctx,
            type_id,
            len,
            key,
            value,
        })
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<'t> ser::SerializeMap for SerializeMapValue<'t> {
    type Ok = SerializationOk<'t>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.needs_init {
            self.ctx.value.write_uint(self.len as u64)?;
            self.needs_init = false;
        }
        let ctx = ::std::mem::replace(&mut self.ctx, SerializationCtx::new());
        let de = FieldValueSerializer {
            ctx,
            type_id: self.key,
        };
        let ok = key.serialize(de)?;
        self.ctx = ok.ctx;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let ctx = ::std::mem::replace(&mut self.ctx, SerializationCtx::new());
        let de = FieldValueSerializer {
            ctx,
            type_id: self.value,
        };
        let ok = value.serialize(de)?;
        self.ctx = ok.ctx;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let is_empty = self.len == 0;

        if is_empty {
            self.ctx.value.write_uint(0)?;
        }

        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: is_empty,
        })
    }
}
