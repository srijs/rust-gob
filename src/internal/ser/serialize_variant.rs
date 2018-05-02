use serde::ser::{self, Serialize};
use serde::de::value::Error;
use serde_schema::types::{Type, EnumVariant};

use ::internal::types::{TypeId, WireType};

use super::{SerializationOk, SerializationCtx, FieldValueSerializer};

pub(crate) struct SerializeNewtypeVariantValue<'t> {
    ctx: SerializationCtx<'t>,
    value: TypeId,
    variant_idx: u32
}

impl<'t> SerializeNewtypeVariantValue<'t> {
    pub(crate) fn new(ctx: SerializationCtx<'t>, type_id: TypeId, variant_idx: u32) -> Result<Self, Error> {
        let value = match ctx.schema.lookup(type_id) {
            Some(&Type::Enum { ref variants, .. }) => {
                if let Some(&EnumVariant::Newtype { value, .. }) = variants.get(variant_idx as usize) {
                    value
                } else {
                    return Err(ser::Error::custom("unsupported enum variant type"));
                }
            },
            _ => {
                return Err(ser::Error::custom("schema mismatch, not an enum"));
            }
        };

        Ok(SerializeNewtypeVariantValue {
            ctx, value, variant_idx
        })
    }

    pub(crate) fn serialize_value<T: ?Sized>(mut self, value: &T) -> Result<SerializationOk<'t>, Error>
        where T: Serialize
    {
        self.ctx.value.write_uint(self.variant_idx as u64 + 1)?;

        let ctx = ::std::mem::replace(&mut self.ctx, SerializationCtx::new());
        let de = FieldValueSerializer { ctx, type_id: self.value };
        let ok = value.serialize(de)?;
        self.ctx = ok.ctx;

        self.ctx.value.write_uint(0)?;

        Ok(SerializationOk {
            ctx: self.ctx,
            is_empty: false
        })
    }
}
