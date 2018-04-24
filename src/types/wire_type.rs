use std::borrow::Cow;
use std::fmt;

use serde::{self, Deserialize, Deserializer};
use serde::de::{Visitor, MapAccess};

use super::{ArrayType, CommonType, SliceType, StructType, MapType, FieldType, TypeId};

#[derive(Clone, Debug)]
pub enum WireType {
    Array(ArrayType),
    Slice(SliceType),
    Struct(StructType),
    Map(MapType)
}

pub static WIRE_TYPE_DEF: WireType = {
    WireType::Struct(StructType {
        common: CommonType { name: Cow::Borrowed("WireType"), id: TypeId::WIRE_TYPE },
        fields: Cow::Borrowed(&[
            FieldType { name: Cow::Borrowed("ArrayT"), id: TypeId::ARRAY_TYPE },
            FieldType { name: Cow::Borrowed("SliceT"), id: TypeId::SLICE_TYPE },
            FieldType { name: Cow::Borrowed("StructT"), id: TypeId::STRUCT_TYPE },
            FieldType { name: Cow::Borrowed("MapT"), id: TypeId::MAP_TYPE },
        ])
    })
};

impl WireType {
    pub fn common(&self) -> &CommonType {
        match self {
            &WireType::Array(ref inner) => &inner.common,
            &WireType::Slice(ref inner) => &inner.common,
            &WireType::Struct(ref inner) => &inner.common,
            &WireType::Map(ref inner) => &inner.common
        }
    }
}

impl<'de> Deserialize<'de> for WireType {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = WireType;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "an WireType struct")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let wire_type = match map.next_key::<String>()? {
                    None => Err(serde::de::Error::custom("no fields in WireType")),
                    Some(ref field) => {
                        match field.as_ref() {
                            "ArrayT" => map.next_value::<ArrayType>().map(WireType::Array),
                            "SliceT" => map.next_value::<SliceType>().map(WireType::Slice),
                            "StructT" => map.next_value::<StructType>().map(WireType::Struct),
                            "MapT" => map.next_value::<MapType>().map(WireType::Map),
                            _ => Err(serde::de::Error::custom("unknown field in WireType"))
                        }
                    }
                }?;

                match map.next_key::<String>()? {
                    None => Ok(wire_type),
                    Some(_) => Err(serde::de::Error::custom("more than one field in WireType"))
                }
            }
        }

        de.deserialize_map(MyVisitor)
    }
}
