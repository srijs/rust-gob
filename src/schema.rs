//! Schema management

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

use serde::de::value::Error;
use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

use ::internal::utils::UniqVec;
use ::internal::gob::{Message, Writer};
use ::internal::types::{Types, WireType, CommonType, StructType, FieldType, ArrayType, SliceType, MapType};
use ::internal::ser::{SerializationCtx, FieldValueSerializer};

pub struct Schema {
    pub(crate) types: Types,
    pending_wire_types: Vec<(TypeId, Vec<u8>)>,
    schema_types: UniqVec<::serde_schema::Type<TypeId>>
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            types: Types::new(),
            pending_wire_types: Vec::new(),
            schema_types: UniqVec::new()
        }
    }

    pub(crate) fn write_pending<W: Write>(&mut self, mut out: Writer<W>) -> Result<(), Error> {
        for (type_id, wire_type_buffer) in self.pending_wire_types.drain(..) {
            out.write_section(-type_id.0, &wire_type_buffer)?;
        }
        Ok(())
    }

    fn queue_wire_type(&mut self, wire_type: &WireType) -> Result<(), Error> {
        let mut wire_type_ctx = SerializationCtx::new();
        let ser = FieldValueSerializer {
            ctx: wire_type_ctx,
            type_id: TypeId::WIRE_TYPE
        };
        let ok = wire_type.serialize(ser)?;
        self.pending_wire_types.push(
            (wire_type.common().id, ok.ctx.value.into_inner()));
        Ok(())
    }
}

impl ::serde_schema::Schema for Schema {
    type TypeId = TypeId;
    type Error = Error;

    fn register_type(&mut self, ty: ::serde_schema::Type<TypeId>) -> Result<TypeId, Error> {
        let mut wire_type = match ty {
            ::serde_schema::Type::Struct { ref name, ref fields } => {
                WireType::Struct(StructType {
                    common: CommonType { name: name.clone(), id: TypeId(0) },
                    fields: fields.iter().map(|field| {
                        FieldType {
                            name: field.name.to_owned(),
                            id: field.id
                        }
                    }).collect()
                })
            },
            ::serde_schema::Type::Seq { len: Some(len), element } => {
                WireType::Array(ArrayType {
                    common: CommonType { name: Cow::Borrowed(""), id: TypeId(0) },
                    len: len as i64,
                    elem: element
                })
            },
            ::serde_schema::Type::Seq { len: None, element } => {
                WireType::Slice(SliceType {
                    common: CommonType { name: Cow::Borrowed(""), id: TypeId(0) },
                    elem: element
                })
            },
            _ => {
                return Err(::serde::de::Error::custom("unsupported type"));
            }
        };

        let (idx, new) = self.schema_types.push(ty);
        let id = TypeId::from_vec_idx(idx);

        if new {
            wire_type.common_mut().id = id;
            self.queue_wire_type(&wire_type)?;
            self.types.insert(wire_type);
        }

        Ok(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(pub(crate) i64);

impl TypeId {
    pub const BOOL: TypeId = TypeId(1);
    pub const INT: TypeId = TypeId(2);
    pub const UINT: TypeId = TypeId(3);
    pub const FLOAT: TypeId = TypeId(4);
    pub const BYTES: TypeId = TypeId(5);
    pub const STRING: TypeId = TypeId(6);
    pub const COMPLEX: TypeId = TypeId(7);
    pub(crate) const INTERFACE: TypeId = TypeId(8);
    pub(crate) const WIRE_TYPE: TypeId = TypeId(16);
    pub(crate) const ARRAY_TYPE: TypeId = TypeId(17);
    pub(crate) const COMMON_TYPE: TypeId = TypeId(18);
    pub(crate) const SLICE_TYPE: TypeId = TypeId(19);
    pub(crate) const STRUCT_TYPE: TypeId = TypeId(20);
    pub(crate) const FIELD_TYPE: TypeId = TypeId(21);
    pub(crate) const FIELD_TYPE_SLICE: TypeId = TypeId(22);
    pub(crate) const MAP_TYPE: TypeId = TypeId(23);

    fn from_vec_idx(idx: usize) -> TypeId {
        TypeId((idx + 65) as i64)
    }

    fn to_vec_idx(&self) -> usize {
        self.0 as usize - 65
    }
}

impl ::serde_schema::TypeId for TypeId {
    const BOOL: TypeId = TypeId(1);
    const I8: TypeId = TypeId(2);
    const I16: TypeId = TypeId(2);
    const I32: TypeId = TypeId(2);
    const I64: TypeId = TypeId(2);
    const CHAR: TypeId = TypeId(2);
    const U8: TypeId = TypeId(3);
    const U16: TypeId = TypeId(3);
    const U32: TypeId = TypeId(3);
    const U64: TypeId = TypeId(3);
    const F32: TypeId = TypeId(4);
    const F64: TypeId = TypeId(4);
    const BYTES: TypeId = TypeId(5);
    const STR: TypeId = TypeId(6);

    // not supported yet
    const UNIT: TypeId = TypeId(0);
}

#[doc(hidden)]
impl<'de> Deserialize<'de> for TypeId {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        i64::deserialize(de).map(TypeId)
    }
}

#[doc(hidden)]
impl Serialize for TypeId {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        if self.0 <= 0 {
            return Err(::serde::ser::Error::custom(format!("invalid type id {}", self.0)));
        }
        self.0.serialize(ser)
    }
}
