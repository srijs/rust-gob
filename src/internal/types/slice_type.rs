use std::borrow::Cow;

use serde_schema::types::{StructField, Type};

use super::{CommonType, FieldType, StructType, TypeId, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SliceType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId,
}

pub static SLICE_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("SliceType"),
        fields: Cow::Borrowed(&[
            StructField {
                name: Cow::Borrowed("common"),
                id: TypeId::COMMON_TYPE,
            },
            StructField {
                name: Cow::Borrowed("Elem"),
                id: TypeId::INT,
            },
        ]),
    }
};

pub static SLICE_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType {
            name: Cow::Borrowed("SliceType"),
            id: TypeId::SLICE_TYPE,
        },
        fields: Cow::Borrowed(&[
            FieldType {
                name: Cow::Borrowed("common"),
                id: TypeId::COMMON_TYPE,
            },
            FieldType {
                name: Cow::Borrowed("Elem"),
                id: TypeId::INT,
            },
        ]),
    })
};
