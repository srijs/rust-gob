use serde::de::value::Error;
use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};
use serde_schema::types::{EnumVariant, StructField, Type};

use schema::TypeId;

use super::{FieldValueSerializer, SerializationCtx};

pub(crate) struct SerializeWireTypes<'a> {
    len_pre: usize,
    wire_types: &'a mut Vec<(TypeId, Vec<u8>)>,
}

impl<'a> SerializeWireTypes<'a> {
    pub fn new(wire_types: &'a mut Vec<(TypeId, Vec<u8>)>) -> Self {
        SerializeWireTypes {
            len_pre: wire_types.len(),
            wire_types,
        }
    }

    pub fn serialize_wire_types(&mut self, id: TypeId, ty: &Type<TypeId>) -> Result<usize, Error> {
        self.serialize_main_type(id, ty)?;
        self.serialize_enum_variants(id.next(), ty)?;
        Ok(self.wire_types.len() - self.len_pre)
    }

    fn serialize_main_type(&mut self, id: TypeId, ty: &Type<TypeId>) -> Result<(), Error> {
        let ctx = SerializationCtx::new();
        let ser = FieldValueSerializer {
            ctx,
            type_id: TypeId::WIRE_TYPE,
        };
        let ok = match ty {
            &Type::Struct {
                ref name,
                ref fields,
            } => ser.serialize_newtype_variant(
                "WireType",
                2,
                "StructT",
                &SerializeStructType { id, name, fields },
            )?,
            &Type::Seq { len, element } => {
                if let Some(len) = len {
                    ser.serialize_newtype_variant(
                        "WireType",
                        0,
                        "ArrayT",
                        &SerializeArrayType {
                            id,
                            len: len as i64,
                            elem: element,
                        },
                    )?
                } else {
                    ser.serialize_newtype_variant(
                        "WireType",
                        1,
                        "SliceT",
                        &SerializeSliceType { id, elem: element },
                    )?
                }
            }
            &Type::Map { key, value } => ser.serialize_newtype_variant(
                "WireType",
                3,
                "MapT",
                &SerializeMapType {
                    id,
                    key: key,
                    elem: value,
                },
            )?,
            &Type::Enum {
                ref name,
                ref variants,
            } => ser.serialize_newtype_variant(
                "WireType",
                2,
                "StructT",
                &SerializeEnumStructType { id, name, variants },
            )?,
            _ => {
                return Err(::serde::de::Error::custom("unsupported type"));
            }
        };
        self.wire_types.push((id, ok.ctx.value.into_inner()));
        Ok(())
    }

    fn serialize_enum_variants(
        &mut self,
        mut next_id: TypeId,
        ty: &Type<TypeId>,
    ) -> Result<(), Error> {
        if let &Type::Enum { ref variants, .. } = ty {
            for variant in variants.iter() {
                if let &EnumVariant::Struct {
                    ref name,
                    ref fields,
                } = variant
                {
                    let ctx = SerializationCtx::new();
                    let ok = {
                        let mut ser = FieldValueSerializer {
                            ctx,
                            type_id: TypeId::WIRE_TYPE,
                        };
                        ser.serialize_newtype_variant(
                            "WireType",
                            2,
                            "StructT",
                            &SerializeStructType {
                                id: next_id,
                                name,
                                fields,
                            },
                        )?
                    };
                    self.wire_types.push((next_id, ok.ctx.value.into_inner()));
                    next_id = next_id.next();
                }
            }
        }
        Ok(())
    }
}

struct SerializeEnumStructType<'a> {
    id: TypeId,
    name: &'a str,
    variants: &'a [EnumVariant<TypeId>],
}

impl<'a> Serialize for SerializeEnumStructType<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("StructType", 2)?;
        s.serialize_field(
            "common",
            &SerializeCommonType {
                id: self.id,
                name: self.name,
            },
        )?;
        s.serialize_field(
            "Fields",
            &SerializeEnumStructFields {
                id: self.id,
                variants: self.variants,
            },
        )?;
        s.end()
    }
}

struct SerializeEnumStructFields<'a> {
    id: TypeId,
    variants: &'a [EnumVariant<TypeId>],
}

impl<'a> Serialize for SerializeEnumStructFields<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_seq(Some(self.variants.len()))?;
        let mut next_id = self.id.next();
        for variant in self.variants {
            match variant {
                &EnumVariant::Newtype { ref name, value } => {
                    s.serialize_element(&SerializeStructField { name, id: value })?
                }
                &EnumVariant::Struct { ref name, .. } => {
                    s.serialize_element(&SerializeStructField { name, id: next_id })?;
                    next_id = next_id.next();
                }
                _ => {
                    return Err(::serde::ser::Error::custom("unsupported variant type"));
                }
            }
        }
        s.end()
    }
}

struct SerializeMapType {
    id: TypeId,
    key: TypeId,
    elem: TypeId,
}

impl Serialize for SerializeMapType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("MapType", 3)?;
        s.serialize_field(
            "common",
            &SerializeCommonType {
                id: self.id,
                name: "",
            },
        )?;
        s.serialize_field("Key", &self.key)?;
        s.serialize_field("Elem", &self.elem)?;
        s.end()
    }
}

struct SerializeSliceType {
    id: TypeId,
    elem: TypeId,
}

impl Serialize for SerializeSliceType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("SliceType", 2)?;
        s.serialize_field(
            "common",
            &SerializeCommonType {
                id: self.id,
                name: "",
            },
        )?;
        s.serialize_field("Elem", &self.elem)?;
        s.end()
    }
}

struct SerializeArrayType {
    id: TypeId,
    elem: TypeId,
    len: i64,
}

impl Serialize for SerializeArrayType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("ArrayType", 3)?;
        s.serialize_field(
            "common",
            &SerializeCommonType {
                id: self.id,
                name: "",
            },
        )?;
        s.serialize_field("Elem", &self.elem)?;
        s.serialize_field("Len", &self.len)?;
        s.end()
    }
}

struct SerializeStructType<'a> {
    id: TypeId,
    name: &'a str,
    fields: &'a [StructField<TypeId>],
}

impl<'a> Serialize for SerializeStructType<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("StructType", 2)?;
        s.serialize_field(
            "common",
            &SerializeCommonType {
                id: self.id,
                name: self.name,
            },
        )?;
        s.serialize_field(
            "Fields",
            &SerializeStructFields {
                fields: self.fields,
            },
        )?;
        s.end()
    }
}

struct SerializeStructFields<'a> {
    fields: &'a [StructField<TypeId>],
}

impl<'a> Serialize for SerializeStructFields<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_seq(Some(self.fields.len()))?;
        for field in self.fields {
            s.serialize_element(&SerializeStructField {
                name: &field.name,
                id: field.id,
            })?;
        }
        s.end()
    }
}

struct SerializeStructField<'a> {
    name: &'a str,
    id: TypeId,
}

impl<'a> Serialize for SerializeStructField<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("FieldType", 2)?;
        s.serialize_field("Name", &self.name)?;
        s.serialize_field("Id", &self.id)?;
        s.end()
    }
}

struct SerializeCommonType<'a> {
    id: TypeId,
    name: &'a str,
}

impl<'a> Serialize for SerializeCommonType<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("CommonType", 2)?;
        s.serialize_field("Name", self.name)?;
        s.serialize_field("Id", &self.id)?;
        s.end()
    }
}
