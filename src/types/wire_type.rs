use std::fmt;

use serde::{self, Deserialize, Deserializer};
use serde::de::{Visitor, MapAccess};

use super::{ArrayType, CommonType, SliceType, StructType, MapType, FieldType, Fields, TypeId};

#[derive(Clone, Debug)]
pub enum WireType {
    Array(ArrayType),
    Slice(SliceType),
    Struct(StructType),
    Map(MapType)
}

impl WireType {
    pub fn def() -> WireType {
        WireType::Struct(StructType {
            common: CommonType { name: "WireType".to_owned(), id: TypeId::WIRE_TYPE },
            fields: Fields(vec![
                FieldType { name: "ArrayT".to_owned(), id: TypeId::ARRAY_TYPE },
                FieldType { name: "SliceT".to_owned(), id: TypeId::SLICE_TYPE },
                FieldType { name: "StructT".to_owned(), id: TypeId::STRUCT_TYPE },
                FieldType { name: "MapT".to_owned(), id: TypeId::MAP_TYPE },
            ])
        })
    }

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
