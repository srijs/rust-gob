use serde::ser::{self, Serialize};
use serde::de::value::Error;
use serde_schema::types::{Type, EnumVariant};

use ::internal::types::TypeId;

use super::{SerializationOk, SerializationCtx, FieldValueSerializer};
use super::SerializeStructValue;

pub(crate) struct SerializeVariantValue<'t> {
    ctx: SerializationCtx<'t>,
    type_id: TypeId,
    variant: EnumVariant<TypeId>,
    variant_idx: u32
}

impl<'t> SerializeVariantValue<'t> {
    pub(crate) fn new(ctx: SerializationCtx<'t>, type_id: TypeId, variant_idx: u32) -> Result<Self, Error> {
        let variant = match ctx.schema.lookup(type_id) {
            Some(&Type::Enum { ref variants, .. }) => {
                if let Some(variant) = variants.get(variant_idx as usize) {
                    variant.clone()
                } else {
                    return Err(ser::Error::custom("unknown enum variant type"));
                }
            },
            _ => {
                return Err(ser::Error::custom("schema mismatch, not an enum"));
            }
        };

        Ok(SerializeVariantValue {
            ctx, type_id, variant, variant_idx
        })
    }

    fn write_header(&mut self) -> Result<(), Error> {
        self.ctx.value.write_uint(self.variant_idx as u64 + 1)?;
        Ok(())
    } 

    fn write_footer(&mut self) -> Result<(), Error> {
        self.ctx.value.write_uint(0)?;
        Ok(())
    }

    pub(crate) fn serialize_newtype<T: ?Sized>(mut self, value: &T) -> Result<SerializationOk<'t>, Error>
        where T: Serialize
    {
        self.write_header()?;
        if let EnumVariant::Newtype { value: type_id, .. } = self.variant {
            let ctx = ::std::mem::replace(&mut self.ctx, SerializationCtx::new());
            let de = FieldValueSerializer { ctx, type_id };
            let ok = value.serialize(de)?;
            self.ctx = ok.ctx;

            self.write_footer()?;

            Ok(SerializationOk {
                ctx: self.ctx,
                is_empty: false
            })
        } else {
            Err(ser::Error::custom("variant type mismatch, expected newtype variant"))
        }
    }

    pub(crate) fn serialize_struct(mut self) -> Result<SerializeStructVariantValue<'t>, Error> {
        self.write_header()?;
        if let EnumVariant::Struct { fields, .. } = self.variant {
            Ok(SerializeStructVariantValue {
                inner: SerializeStructValue::from_parts(self.ctx, self.type_id, fields.into_owned())
            })
        } else {
            Err(ser::Error::custom("variant type mismatch, expected newtype variant"))
        }
    }
}

pub(crate) struct SerializeStructVariantValue<'t> {
    inner: SerializeStructValue<'t>
}

impl<'t> SerializeStructVariantValue<'t> {
    pub(crate) fn type_id(&self) -> TypeId {
        self.inner.type_id()
    }
}

impl<'t> ser::SerializeStructVariant for SerializeStructVariantValue<'t> {
    type Ok = SerializationOk<'t>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        ser::SerializeStruct::serialize_field(&mut self.inner, key, value)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        ser::SerializeStruct::skip_field(&mut self.inner, key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut ok = ser::SerializeStruct::end(self.inner)?;
        ok.ctx.value.write_uint(0)?;
        ok.is_empty = false;
        Ok(ok)
    }
}
