use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};
use serde_schema::types::{EnumVariant, StructField, Type};

use error::Error;
use schema::{Schema, TypeId};

use super::{FieldValueSerializer, SerializationCtx};

pub(crate) struct SerializeWireTypes<'a> {
    len_pre: usize,
    wire_types: &'a mut Vec<Vec<u8>>,
}

impl<'a> SerializeWireTypes<'a> {
    pub fn new(wire_types: &'a mut Vec<Vec<u8>>) -> Self {
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
        let mut ctx = SerializationCtx::with_schema(Schema::new());
        ctx.value.write_int(-id.0);
        let ser = FieldValueSerializer {
            ctx,
            type_id: TypeId::WIRE_TYPE,
        };
        let ok = match ty {
            &Type::Struct(ref struct_type) => ser.serialize_newtype_variant(
                "WireType",
                2,
                "StructT",
                &SerializeStructType {
                    id,
                    name: struct_type.name(),
                    fields: struct_type.fields(),
                },
            )?,
            &Type::Seq(ref seq_type) => {
                if let Some(len) = seq_type.len() {
                    ser.serialize_newtype_variant(
                        "WireType",
                        0,
                        "ArrayT",
                        &SerializeArrayType {
                            id,
                            len: len as i64,
                            elem: *seq_type.element_type(),
                        },
                    )?
                } else {
                    ser.serialize_newtype_variant(
                        "WireType",
                        1,
                        "SliceT",
                        &SerializeSliceType {
                            id,
                            elem: *seq_type.element_type(),
                        },
                    )?
                }
            }
            &Type::Map(ref map_type) => ser.serialize_newtype_variant(
                "WireType",
                3,
                "MapT",
                &SerializeMapType {
                    id,
                    key: *map_type.key_type(),
                    elem: *map_type.value_type(),
                },
            )?,
            &Type::Enum(ref enum_type) => ser.serialize_newtype_variant(
                "WireType",
                2,
                "StructT",
                &SerializeEnumStructType {
                    id,
                    name: enum_type.name(),
                    variants: enum_type.variants(),
                },
            )?,
            _ => {
                return Err(::serde::de::Error::custom("unsupported type"));
            }
        };
        self.wire_types.push(ok.ctx.value.into_inner());
        Ok(())
    }

    fn serialize_enum_variants(
        &mut self,
        mut next_id: TypeId,
        ty: &Type<TypeId>,
    ) -> Result<(), Error> {
        if let &Type::Enum(ref enum_type) = ty {
            for variant in enum_type.variants() {
                if let Some(struct_variant) = variant.as_struct_variant() {
                    let mut ctx = SerializationCtx::with_schema(Schema::new());
                    ctx.value.write_int(-next_id.0);
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
                                name: struct_variant.name(),
                                fields: struct_variant.fields(),
                            },
                        )?
                    };
                    self.wire_types.push(ok.ctx.value.into_inner());
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
                &EnumVariant::Newtype(ref newtype_variant) => {
                    s.serialize_element(&SerializeStructField {
                        name: newtype_variant.name(),
                        id: *newtype_variant.inner_type(),
                    })?
                }
                &EnumVariant::Struct(ref struct_variant) => {
                    s.serialize_element(&SerializeStructField {
                        name: struct_variant.name(),
                        id: next_id,
                    })?;
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
                name: field.name(),
                id: *field.field_type(),
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
