use std::borrow::Cow;
use std::fmt;

use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Visitor, MapAccess};
use serde::de::value::Error;
use serde::ser::SerializeStruct;
use serde_schema::{Type, StructField};

use super::{ArrayType, CommonType, SliceType, StructType, MapType, FieldType, TypeId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WireType {
    Array(ArrayType),
    Slice(SliceType),
    Struct(StructType),
    Map(MapType)
}

pub static WIRE_TYPE_DEF: Type<TypeId> = {
    Type::Struct {
        name: Cow::Borrowed("WireType"),
        fields: Cow::Borrowed(&[
            StructField { name: Cow::Borrowed("ArrayT"), id: TypeId::ARRAY_TYPE },
            StructField { name: Cow::Borrowed("SliceT"), id: TypeId::SLICE_TYPE },
            StructField { name: Cow::Borrowed("StructT"), id: TypeId::STRUCT_TYPE },
            StructField { name: Cow::Borrowed("MapT"), id: TypeId::MAP_TYPE },
        ])
    }
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

    pub fn common_mut(&mut self) -> &mut CommonType {
        match self {
            &mut WireType::Array(ref mut inner) => &mut inner.common,
            &mut WireType::Slice(ref mut inner) => &mut inner.common,
            &mut WireType::Struct(ref mut inner) => &mut inner.common,
            &mut WireType::Map(ref mut inner) => &mut inner.common
        }
    }

    pub fn to_type(&self) -> Type<TypeId> {
        match self {
            &WireType::Array(ref array_type) => {
                Type::Seq {
                    len: Some(array_type.len as usize),
                    element: array_type.elem
                }
            },
            &WireType::Slice(ref slice_type) => {
                Type::Seq {
                    len: None,
                    element: slice_type.elem
                }
            },
            &WireType::Struct(ref struct_type) => {
                Type::Struct {
                    name: struct_type.common.name.to_owned(),
                    fields: Cow::Owned(struct_type.fields.iter().map(|field| {
                        StructField {
                            name: field.name.to_owned(),
                            id: field.id
                        }
                    }).collect())
                }
            },
            &WireType::Map(ref map_type) => {
                Type::Map {
                    key: map_type.key,
                    value: map_type.elem
                }
            }
        }
    }

    pub fn from_type(id: TypeId, ty: &Type<TypeId>) -> Result<WireType, Error> {
        match ty {
            &Type::Struct { ref name, ref fields } => {
                Ok(WireType::Struct(StructType {
                    common: CommonType { name: name.clone(), id },
                    fields: fields.iter().map(|field| {
                        FieldType {
                            name: field.name.to_owned(),
                            id: field.id
                        }
                    }).collect()
                }))
            },
            &Type::Seq { len: Some(len), element } => {
                Ok(WireType::Array(ArrayType {
                    common: CommonType { name: Cow::Borrowed(""), id },
                    len: len as i64,
                    elem: element
                }))
            },
            &Type::Seq { len: None, element } => {
                Ok(WireType::Slice(SliceType {
                    common: CommonType { name: Cow::Borrowed(""), id },
                    elem: element
                }))
            },
            _ => {
                return Err(::serde::de::Error::custom("unsupported type"));
            }
        }
    }
}

impl Serialize for WireType {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut ser_struct = ser.serialize_struct("WireType", 1)?;
        match self {
            &WireType::Array(ref array_type) => {
                ser_struct.serialize_field("ArrayT", array_type)?;
                ser_struct.skip_field("SliceT")?;
                ser_struct.skip_field("StructT")?;
                ser_struct.skip_field("MapT")?;
            },
            &WireType::Slice(ref slice_type) => {
                ser_struct.skip_field("ArrayT")?;
                ser_struct.serialize_field("SliceT", slice_type)?;
                ser_struct.skip_field("StructT")?;
                ser_struct.skip_field("MapT")?;
            },
            &WireType::Struct(ref struct_type) => {
                ser_struct.skip_field("ArrayT")?;
                ser_struct.skip_field("SliceT")?;
                ser_struct.serialize_field("StructT", struct_type)?;
                ser_struct.skip_field("MapT")?;
            },
            &WireType::Map(ref map_type) => {
                ser_struct.skip_field("ArrayT")?;
                ser_struct.skip_field("SliceT")?;
                ser_struct.skip_field("StructT")?;
                ser_struct.serialize_field("MapT", map_type)?;
            }
        }
        ser_struct.end()
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
