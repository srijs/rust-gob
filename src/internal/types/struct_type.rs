use std::borrow::Cow;

use serde_schema::{Type, StructField};

use super::{CommonType, TypeId, WireType, SliceType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StructType {
    pub common: CommonType,
    // the fields of the struct
    #[serde(rename = "Fields")]
    pub fields: Cow<'static, [FieldType]>
}

pub static STRUCT_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("StructType"),
        fields: Cow::Borrowed(&[
            StructField { name: Cow::Borrowed("common"), id: TypeId::COMMON_TYPE },
            StructField { name: Cow::Borrowed("Fields"), id: TypeId::FIELD_TYPE_SLICE }
        ])
    }
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldType {
    // the name of the field
    #[serde(rename = "Name")]
    pub name: Cow<'static, str>,
    // the type id of the field, which must be already defined
    #[serde(rename = "Id")]
    pub id: TypeId
}

pub static FIELD_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("FieldType"),
        fields: Cow::Borrowed(&[
            StructField { name: Cow::Borrowed("Name"), id: TypeId::STRING },
            StructField { name: Cow::Borrowed("Id"), id: TypeId::INT }
        ])
    }
};

pub static FIELD_TYPE_SLICE_DEF: Type<TypeId> = {
    Type::Seq {
        len: None,
        element: TypeId::FIELD_TYPE
    }
};
