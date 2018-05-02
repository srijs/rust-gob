use std::borrow::Cow;
use std::fmt;

use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Visitor, MapAccess};
use serde::de::value::Error;
use serde::ser::SerializeStruct;
use serde_schema::types::{Type, StructField, EnumVariant};

use super::{ArrayType, CommonType, SliceType, StructType, MapType, FieldType, TypeId};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WireType {
    #[serde(rename="ArrayT")]
    Array(ArrayType),
    #[serde(rename="SliceT")]
    Slice(SliceType),
    #[serde(rename="StructT")]
    Struct(StructType),
    #[serde(rename="MapT")]
    Map(MapType)
}

pub static WIRE_TYPE_DEF: Type<TypeId> = {
    Type::Enum {
        name: Cow::Borrowed("WireType"),
        variants: Cow::Borrowed(&[
            EnumVariant::Newtype { name: Cow::Borrowed("ArrayT"), value: TypeId::ARRAY_TYPE },
            EnumVariant::Newtype { name: Cow::Borrowed("SliceT"), value: TypeId::SLICE_TYPE },
            EnumVariant::Newtype { name: Cow::Borrowed("StructT"), value: TypeId::STRUCT_TYPE },
            EnumVariant::Newtype { name: Cow::Borrowed("MapT"), value: TypeId::MAP_TYPE },
        ])
    }
};

impl WireType {
    pub fn common(&self) -> &CommonType {
        match self {
            &WireType::Array(ref inner) => &inner.common,
            &WireType::Slice(ref inner) => &inner.common,
            &WireType::Struct(ref inner) => &inner.common,
            &WireType::Map(ref inner) => &inner.common
        }
    }

    pub fn common_mut(&mut self) -> &mut CommonType {
        match self {
            &mut WireType::Array(ref mut inner) => &mut inner.common,
            &mut WireType::Slice(ref mut inner) => &mut inner.common,
            &mut WireType::Struct(ref mut inner) => &mut inner.common,
            &mut WireType::Map(ref mut inner) => &mut inner.common
        }
    }

    pub fn from_type(id: TypeId, ty: &Type<TypeId>) -> Result<WireType, Error> {
        match ty {
            &Type::Struct { ref name, ref fields } => {
                Ok(WireType::Struct(StructType {
                    common: CommonType { name: name.clone(), id },
                    fields: fields.iter().map(|field| {
                        FieldType {
                            name: field.name.to_owned(),
                            id: field.id
                        }
                    }).collect()
                }))
            },
            &Type::Seq { len: Some(len), element } => {
                Ok(WireType::Array(ArrayType {
                    common: CommonType { name: Cow::Borrowed(""), id },
                    len: len as i64,
                    elem: element
                }))
            },
            &Type::Seq { len: None, element } => {
                Ok(WireType::Slice(SliceType {
                    common: CommonType { name: Cow::Borrowed(""), id },
                    elem: element
                }))
            },
            &Type::Map { key, value } => {
                Ok(WireType::Map(MapType {
                    common: CommonType { name: Cow::Borrowed(""), id },
                    key: key,
                    elem: value
                }))
            },
            &Type::Enum { ref name, ref variants } => {
                let fields = variants.iter().map(|variant| {
                    match variant {
                        &EnumVariant::Newtype { ref name, value } =>
                            Ok(FieldType { name: name.to_owned(), id: value }),
                        _ =>
                            Err(::serde::de::Error::custom("unsupported variant type"))
                    }
                }).collect::<Result<_, Error>>()?;
                Ok(WireType::Struct(StructType {
                    common: CommonType { name: name.to_owned(), id },
                    fields: Cow::Owned(fields)
                }))
            },
            _ => {
                return Err(::serde::de::Error::custom("unsupported type"));
            }
        }
    }
}
