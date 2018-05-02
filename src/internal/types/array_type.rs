use std::borrow::Cow;

use serde_schema::types::{Type, StructField};

use super::{CommonType, TypeId, StructType, FieldType, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId,
    #[serde(rename = "Len", default)]
    pub len: i64
}

pub static ARRAY_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("ArrayType"),
        fields: Cow::Borrowed(&[
            StructField { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            StructField { name: Cow::Borrowed("Elem"), id: TypeId::INT },
            StructField { name: Cow::Borrowed("Len"), id: TypeId::INT }
        ])
    }
};

pub static ARRAY_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType { name: Cow::Borrowed("ArrayType"), id: TypeId::ARRAY_TYPE },
        fields: Cow::Borrowed(&[
            FieldType { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            FieldType { name: Cow::Borrowed("Elem"), id: TypeId::INT },
            FieldType { name: Cow::Borrowed("Len"), id: TypeId::INT }
        ])
    })
};
