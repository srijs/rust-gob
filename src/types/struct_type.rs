use serde::{Deserialize, Deserializer};

use super::{CommonType, TypeId, WireType, SliceType};

#[derive(Clone, Debug, Deserialize)]
pub struct StructType {
    pub common: CommonType,
    // the fields of the struct
    #[serde(rename = "Fields")]
    pub fields: Fields
}

impl StructType {
    pub fn def() -> WireType {
        WireType::Struct(StructType {
            common: CommonType { name: "StructType".to_owned(), id: TypeId::STRUCT_TYPE },
            fields: Fields(vec![
                FieldType { name: "common".to_owned(), id: TypeId::COMMON_TYPE },
                FieldType { name: "Fields".to_owned(), id: TypeId::FIELDS }
            ])
        })
    }
}

#[derive(Clone, Debug)]
pub struct Fields(pub Vec<FieldType>);

impl Fields {
    pub fn def() -> WireType {
        WireType::Slice(SliceType {
            common: CommonType { name: "Fields".to_owned(), id: TypeId::FIELDS },
            elem: TypeId::FIELD_TYPE
        })
    }
}

impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        <Vec<FieldType>>::deserialize(de).map(Fields)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FieldType {
    // the name of the field
    #[serde(rename = "Name")]
    pub name: String,
    // the type id of the field, which must be already defined
    #[serde(rename = "Id")]
    pub id: TypeId
}

impl FieldType {
    pub fn def() -> WireType {
        WireType::Struct(StructType {
            common: CommonType { name: "FieldType".to_owned(), id: TypeId::FIELD_TYPE },
            fields: Fields(vec![
                FieldType { name: "Name".to_owned(), id: TypeId::STRING },
                FieldType { name: "Id".to_owned(), id: TypeId::INT }
            ])
        })
    }
}
