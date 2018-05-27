use std::borrow::Cow;

use serde_schema::types::Type;

use super::{CommonType, SliceType, TypeId, WireType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StructType {
    pub common: CommonType,
    // the fields of the struct
    #[serde(rename = "Fields", default)]
    pub fields: Cow<'static, [FieldType]>,
}

lazy_static! {
    pub static ref STRUCT_TYPE_DEF: Type<TypeId> = {
        Type::build()
            .struct_type("StructType", 2)
            .field("common", TypeId::COMMON_TYPE)
            .field("Fields", TypeId::FIELD_TYPE_SLICE)
            .end()
    };
}

pub static STRUCT_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType {
            name: Cow::Borrowed("StructType"),
            id: TypeId::STRUCT_TYPE,
        },
        fields: Cow::Borrowed(&[
            FieldType {
                name: Cow::Borrowed("common"),
                id: TypeId::COMMON_TYPE,
            },
            FieldType {
                name: Cow::Borrowed("Fields"),
                id: TypeId::FIELD_TYPE_SLICE,
            },
        ]),
    })
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldType {
    // the name of the field
    #[serde(rename = "Name")]
    pub name: Cow<'static, str>,
    // the type id of the field, which must be already defined
    #[serde(rename = "Id")]
    pub id: TypeId,
}

lazy_static! {
    pub static ref FIELD_TYPE_DEF: Type<TypeId> = {
        Type::build()
            .struct_type("FieldType", 2)
            .field("Name", TypeId::STRING)
            .field("Id", TypeId::INT)
            .end()
    };
}

pub static FIELD_TYPE_DEF_2: WireType = {
    WireType::Struct(StructType {
        common: CommonType {
            name: Cow::Borrowed("FieldType"),
            id: TypeId::FIELD_TYPE,
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

lazy_static! {
    pub static ref FIELD_TYPE_SLICE_DEF: Type<TypeId> =
        { Type::build().seq_type(None, TypeId::FIELD_TYPE) };
}

pub static FIELD_TYPE_SLICE_DEF_2: WireType = {
    WireType::Slice(SliceType {
        common: CommonType {
            name: Cow::Borrowed(""),
            id: TypeId::FIELD_TYPE_SLICE,
        },
        elem: TypeId::FIELD_TYPE,
    })
};
