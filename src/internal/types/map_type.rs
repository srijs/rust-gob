use std::borrow::Cow;

use serde_schema::types::{Type, StructField};

use super::{CommonType, TypeId, StructType, FieldType, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MapType {
    pub common: CommonType,
    #[serde(rename = "Key")]
    pub key: TypeId,
    #[serde(rename = "Elem")]
    pub elem: TypeId
}

pub static MAP_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("MapType"),
        fields: Cow::Borrowed(&[
            StructField { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            StructField { name: Cow::Borrowed("Key"), id: TypeId::INT },
            StructField { name: Cow::Borrowed("Elem"), id: TypeId::INT }
        ])
    }
};

pub static MAP_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType { name: Cow::Borrowed("MapType"), id: TypeId::MAP_TYPE },
        fields: Cow::Borrowed(&[
            FieldType { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            FieldType { name: Cow::Borrowed("Key"), id: TypeId::INT },
            FieldType { name: Cow::Borrowed("Elem"), id: TypeId::INT }
        ])
    })
};
