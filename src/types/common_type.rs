use std::borrow::Cow;

use super::{TypeId, StructType, FieldType, WireType};

#[derive(Clone, Debug, Deserialize)]
pub struct CommonType {
    #[serde(rename = "Name", default)]
    pub name: Cow<'static, str>,
    #[serde(rename = "Id")]
    pub id: TypeId
}

pub static COMMON_TYPE_DEF: WireType = {
    WireType::Struct(StructType {
        common: CommonType { name: Cow::Borrowed("CommonType"), id: TypeId::COMMON_TYPE },
        fields: Cow::Borrowed(&[
            FieldType { name: Cow::Borrowed("Name"), id: TypeId::STRING },
            FieldType { name: Cow::Borrowed("Id"), id: TypeId::INT }
        ])
    })
};
