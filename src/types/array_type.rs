use std::borrow::Cow;

use super::{CommonType, TypeId, StructType, FieldType, WireType};

#[derive(Clone, Debug, Deserialize)]
pub struct ArrayType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId,
    #[serde(rename = "Len", default)]
    pub len: i64
}

pub static ARRAY_TYPE_DEF: WireType = {
    WireType::Struct(StructType {
        common: CommonType { name: Cow::Borrowed("ArrayType"), id: TypeId::ARRAY_TYPE },
        fields: Cow::Borrowed(&[
            FieldType { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            FieldType { name: Cow::Borrowed("Elem"), id: TypeId::INT },
            FieldType { name: Cow::Borrowed("Len"), id: TypeId::INT }
        ])
    })
};
