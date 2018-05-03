use std::borrow::Cow;

use serde_schema::types::{StructField, Type};

use super::{FieldType, StructType, TypeId, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommonType {
    #[serde(rename = "Name", default)]
    pub name: Cow<'static, str>,
    #[serde(rename = "Id")]
    pub id: TypeId,
}

pub static COMMON_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("CommonType"),
        fields: Cow::Borrowed(&[
            StructField {
                name: Cow::Borrowed("Name"),
                id: TypeId::STRING,
            },
            StructField {
                name: Cow::Borrowed("Id"),
                id: TypeId::INT,
            },
        ]),
    }
};

pub static COMMON_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType {
            name: Cow::Borrowed("CommonType"),
            id: TypeId::COMMON_TYPE,
        },
        fields: Cow::Borrowed(&[
            FieldType {
                name: Cow::Borrowed("Name"),
                id: TypeId::STRING,
            },
            FieldType {
                name: Cow::Borrowed("Id"),
                id: TypeId::INT,
            },
        ]),
    })
};
