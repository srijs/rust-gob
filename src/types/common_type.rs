use super::{TypeId, StructType, Fields, FieldType, WireType};

#[derive(Clone, Debug, Deserialize)]
pub struct CommonType {
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: TypeId
}

impl CommonType {
    pub fn def() -> WireType {
        WireType::Struct(StructType {
            common: CommonType { name: "CommonType".to_owned(), id: TypeId::COMMON_TYPE },
            fields: Fields(vec![
                FieldType { name: "Name".to_owned(), id: TypeId::STRING },
                FieldType { name: "Id".to_owned(), id: TypeId::INT }
            ])
        })
    }
}
