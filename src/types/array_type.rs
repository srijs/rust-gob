use super::{CommonType, TypeId, StructType, Fields, FieldType, WireType};

#[derive(Clone, Debug, Deserialize)]
pub struct ArrayType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId,
    #[serde(rename = "Len")]
    pub len: i64
}

impl ArrayType {
    pub fn def() -> WireType {
        WireType::Struct(StructType {
            common: CommonType { name: "ArrayType".to_owned(), id: TypeId::ARRAY_TYPE },
            fields: Fields(vec![
                FieldType { name: "common".to_owned(), id: TypeId::COMMON_TYPE },
                FieldType { name: "Elem".to_owned(), id: TypeId::INT },
                FieldType { name: "Len".to_owned(), id: TypeId::INT }
            ])
        })
    }
}
