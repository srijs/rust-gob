use std::borrow::Cow;

use serde_schema::types::Type;

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

lazy_static! {
    pub static ref WIRE_TYPE_DEF: Type<TypeId> = {
        Type::build().enum_type("WireType", 4)
            .newtype_variant("ArrayT", TypeId::ARRAY_TYPE)
            .newtype_variant("SliceT", TypeId::SLICE_TYPE)
            .newtype_variant("StructT", TypeId::STRUCT_TYPE)
            .newtype_variant("MapT", TypeId::MAP_TYPE)
            .end()
    };
}

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
