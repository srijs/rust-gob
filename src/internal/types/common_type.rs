use std::borrow::Cow;

use serde_schema::types::Type;

use super::{FieldType, StructType, TypeId, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommonType {
    #[serde(rename = "Name", default)]
    pub name: Cow<'static, str>,
    #[serde(rename = "Id")]
    pub id: TypeId,
}

lazy_static! {
    pub static ref COMMON_TYPE_DEF: Type<TypeId> = {
        Type::build().struct_type("CommonType", 2)
            .field("Name", TypeId::STRING)
            .field("Id", TypeId::INT)
            .end()
    };
}

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
