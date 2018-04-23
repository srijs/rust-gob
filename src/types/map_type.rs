use super::{CommonType, TypeId, StructType, Fields, FieldType, WireType};

#[derive(Clone, Debug, Deserialize)]
pub struct MapType {
    pub common: CommonType,
    #[serde(rename = "Key")]
    pub key: TypeId,
    #[serde(rename = "Elem")]
    pub elem: TypeId
}

impl MapType {
    pub fn def() -> WireType {
        WireType::Struct(StructType {
            common: CommonType { name: "MapType".to_owned(), id: TypeId::MAP_TYPE },
            fields: Fields(vec![
                FieldType { name: "common".to_owned(), id: TypeId::COMMON_TYPE },
                FieldType { name: "Key".to_owned(), id: TypeId::INT },
                FieldType { name: "Elem".to_owned(), id: TypeId::INT }
            ])
        })
    }
}
