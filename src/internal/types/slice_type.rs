use std::borrow::Cow;

use serde_schema::types::{Type, StructField};

use super::{CommonType, TypeId, StructType, FieldType, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SliceType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId
}

pub static SLICE_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("SliceType"),
        fields: Cow::Borrowed(&[
            StructField { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            StructField { name: Cow::Borrowed("Elem"), id: TypeId::INT }
        ])
    }
};
