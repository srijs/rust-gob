use std::borrow::Cow;

use serde_schema::types::{EnumVariant, Type};

use super::{ArrayType, CommonType, FieldType, MapType, SliceType, StructType, TypeId};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum WireType {
    #[serde(rename = "ArrayT")]
    Array(ArrayType),
    #[serde(rename = "SliceT")]
    Slice(SliceType),
    #[serde(rename = "StructT")]
    Struct(StructType),
    #[serde(rename = "MapT")]
    Map(MapType),
}

pub static WIRE_TYPE_DEF: Type<TypeId> = {
    Type::Enum {
        name: Cow::Borrowed("WireType"),
        variants: Cow::Borrowed(&[
            EnumVariant::Newtype {
                name: Cow::Borrowed("ArrayT"),
                value: TypeId::ARRAY_TYPE,
            },
            EnumVariant::Newtype {
                name: Cow::Borrowed("SliceT"),
                value: TypeId::SLICE_TYPE,
            },
            EnumVariant::Newtype {
                name: Cow::Borrowed("StructT"),
                value: TypeId::STRUCT_TYPE,
            },
            EnumVariant::Newtype {
                name: Cow::Borrowed("MapT"),
                value: TypeId::MAP_TYPE,
            },
        ]),
    }
};

pub static WIRE_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType {
            name: Cow::Borrowed("WireType"),
            id: TypeId::WIRE_TYPE,
        },
        fields: Cow::Borrowed(&[
            FieldType {
                name: Cow::Borrowed("ArrayT"),
                id: TypeId::ARRAY_TYPE,
            },
            FieldType {
                name: Cow::Borrowed("SliceT"),
                id: TypeId::SLICE_TYPE,
            },
            FieldType {
                name: Cow::Borrowed("StructT"),
                id: TypeId::STRUCT_TYPE,
            },
            FieldType {
                name: Cow::Borrowed("MapT"),
                id: TypeId::MAP_TYPE,
            },
        ]),
    })
};

impl WireType {
    pub fn common(&self) -> &CommonType {
        match self {
            &WireType::Array(ref inner) => &inner.common,
            &WireType::Slice(ref inner) => &inner.common,
            &WireType::Struct(ref inner) => &inner.common,
            &WireType::Map(ref inner) => &inner.common,
        }
    }
}
