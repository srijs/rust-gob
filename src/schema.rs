//! Schema management

use std::borrow::Cow;

use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

use ::internal::types::{Types, WireType, CommonType, StructType, FieldType, ArrayType, SliceType, MapType};

pub struct Schema {
    pub(crate) types: Types,
    pub(crate) last_sent_type_id: Option<TypeId>
}

impl Schema {
    pub fn new() -> Schema {
        Schema { types: Types::new(), last_sent_type_id: None }
    }

    pub fn register_struct_type<S>(&mut self, name: S) -> RegisterStructType
        where S: Into<Cow<'static, str>>
    {
        RegisterStructType::new(name, &mut self.types)
    }

    pub fn register_slice_type(&mut self, element: TypeId) -> TypeId {
        let id = self.types.next_custom_id();
        let wire_type = WireType::Slice(SliceType {
            common: CommonType { name: Cow::Borrowed(""), id: id },
            elem: element
        });
        self.types.insert(wire_type);
        id
    }

    pub fn register_array_type(&mut self, element: TypeId, len: usize) -> TypeId {
        let id = self.types.next_custom_id();
        let wire_type = WireType::Array(ArrayType {
            common: CommonType { name: Cow::Borrowed(""), id: id },
            elem: element,
            len: len as i64
        });
        self.types.insert(wire_type);
        id
    }

    pub fn register_map_type(&mut self, key: TypeId, value: TypeId) -> TypeId {
        let id = self.types.next_custom_id();
        let wire_type = WireType::Map(MapType {
            common: CommonType { name: Cow::Borrowed(""), id: id },
            key: key,
            elem: value
        });
        self.types.insert(wire_type);
        id
    }
}

pub struct RegisterStructType<'a> {
    types: &'a mut Types,
    name: Cow<'static, str>,
    fields: Vec<FieldType>
}

impl<'a> RegisterStructType<'a> {
    pub(crate) fn new<S: Into<Cow<'static, str>>>(name: S, types: &'a mut Types) -> Self {
        RegisterStructType { types, name: name.into(), fields: Vec::new() }
    }

    pub fn field<S>(mut self, name: S, id: TypeId) -> Self
        where S: Into<Cow<'static, str>>
    {
        self.fields.push(FieldType { name: name.into(), id });
        self
    }

    pub fn finish(self) -> TypeId {
        let id = self.types.next_custom_id();
        let wire_type = WireType::Struct(StructType {
            common: CommonType { name: self.name, id: id },
            fields: Cow::Owned(self.fields)
        });
        self.types.insert(wire_type);
        id
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
        self.0.serialize(ser)
    }
}
