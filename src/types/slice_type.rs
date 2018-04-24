use std::borrow::Cow;

use super::{CommonType, TypeId, StructType, FieldType, WireType};

#[derive(Clone, Debug, Deserialize)]
pub struct SliceType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId
}

pub static SLICE_TYPE_DEF: WireType = {
    WireType::Struct(StructType {
        common: CommonType { name: Cow::Borrowed("SliceType"), id: TypeId::SLICE_TYPE },
        fields: Cow::Borrowed(&[
            FieldType { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            FieldType { name: Cow::Borrowed("Elem"), id: TypeId::INT }
        ])
    })
};
