use super::{CommonType, TypeId, StructType, Fields, FieldType, WireType};

#[derive(Clone, Debug, Deserialize)]
pub struct SliceType {
    pub common: CommonType,
    #[serde(rename = "Elem")]
    pub elem: TypeId
}

impl SliceType {
    pub fn def() -> WireType {
        WireType::Struct(StructType {
            common: CommonType { name: "SliceType".to_owned(), id: TypeId::SLICE_TYPE },
            fields: Fields(vec![
                FieldType { name: "common".to_owned(), id: TypeId::COMMON_TYPE },
                FieldType { name: "Elem".to_owned(), id: TypeId::INT }
            ])
        })
    }
}
