use std::borrow::Cow;

use serde_schema::types::Type;

use super::{CommonType, FieldType, StructType, TypeId, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId,
    #[serde(rename = "Len", default)]
    pub len: i64,
}

lazy_static! {
    pub static ref ARRAY_TYPE_DEF: Type<TypeId> = {
        Type::build()
            .struct_type("ArrayType", 3)
            .field("common", TypeId::COMMON_TYPE)
            .field("Elem", TypeId::INT)
            .field("Len", TypeId::INT)
            .end()
    };
}

pub static ARRAY_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType {
            name: Cow::Borrowed("ArrayType"),
            id: TypeId::ARRAY_TYPE,
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
            FieldType {
                name: Cow::Borrowed("Len"),
                id: TypeId::INT,
            },
        ]),
    })
};
