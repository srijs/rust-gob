use serde::{Deserialize, Deserializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId(pub i64);

impl TypeId {
    pub const BOOL: TypeId = TypeId(1);
    pub const INT: TypeId = TypeId(2);
    pub const UINT: TypeId = TypeId(3);
    pub const FLOAT: TypeId = TypeId(4);
    pub const BYTES: TypeId = TypeId(5);
    pub const STRING: TypeId = TypeId(6);
    pub const COMPLEX: TypeId = TypeId(7);
    pub const INTERFACE: TypeId = TypeId(8);
    pub const WIRE_TYPE: TypeId = TypeId(16);
    pub const ARRAY_TYPE: TypeId = TypeId(17);
    pub const COMMON_TYPE: TypeId = TypeId(18);
    pub const SLICE_TYPE: TypeId = TypeId(19);
    pub const STRUCT_TYPE: TypeId = TypeId(20);
    pub const FIELD_TYPE: TypeId = TypeId(21);
    pub const FIELD_TYPE_SLICE: TypeId = TypeId(22);
    pub const MAP_TYPE: TypeId = TypeId(23);
}

impl<'de> Deserialize<'de> for TypeId {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        i64::deserialize(de).map(TypeId)
    }
}
