use std::borrow::Cow;

use serde_schema::types::Type;

use super::{CommonType, FieldType, StructType, TypeId, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MapType {
    pub common: CommonType,
    #[serde(rename = "Key")]
    pub key: TypeId,
    #[serde(rename = "Elem")]
    pub elem: TypeId,
}

lazy_static! {
    pub static ref MAP_TYPE_DEF: Type<TypeId> = {
        Type::build().struct_type("MapType", 3)
            .field("common", TypeId::COMMON_TYPE)
            .field("Key", TypeId::INT)
            .field("Elem", TypeId::INT)
            .end()
    };
}

pub static MAP_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType {
            name: Cow::Borrowed("MapType"),
            id: TypeId::MAP_TYPE,
        },
        fields: Cow::Borrowed(&[
            FieldType {
                name: Cow::Borrowed("common"),
                id: TypeId::COMMON_TYPE,
            },
            FieldType {
                name: Cow::Borrowed("Key"),
                id: TypeId::INT,
            },
            FieldType {
                name: Cow::Borrowed("Elem"),
                id: TypeId::INT,
            },
        ]),
    })
};
