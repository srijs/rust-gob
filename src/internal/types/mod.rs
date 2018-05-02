use std::collections::BTreeMap;

use serde_schema::types::Type;

mod wire_type;
pub(crate) use self::wire_type::WireType;

mod common_type;
pub(crate) use self::common_type::CommonType;

mod array_type;
pub(crate) use self::array_type::ArrayType;

mod slice_type;
pub(crate) use self::slice_type::SliceType;

mod struct_type;
pub(crate) use self::struct_type::{FieldType, StructType};

mod map_type;
pub(crate) use self::map_type::MapType;

pub use ::schema::TypeId;

#[derive(Debug)]
pub struct Types {
    map: BTreeMap<TypeId, WireType>
}

pub(crate) fn lookup_builtin(id: TypeId) -> Option<&'static Type<TypeId>> {
    match id {
        TypeId::ARRAY_TYPE => Some(&self::array_type::ARRAY_TYPE_DEF),
        TypeId::MAP_TYPE => Some(&self::map_type::MAP_TYPE_DEF),
        TypeId::SLICE_TYPE => Some(&self::slice_type::SLICE_TYPE_DEF),
        TypeId::FIELD_TYPE => Some(&self::struct_type::FIELD_TYPE_DEF),
        TypeId::FIELD_TYPE_SLICE => Some(&self::struct_type::FIELD_TYPE_SLICE_DEF),
        TypeId::STRUCT_TYPE => Some(&self::struct_type::STRUCT_TYPE_DEF),
        TypeId::WIRE_TYPE => Some(&self::wire_type::WIRE_TYPE_DEF),
        TypeId::COMMON_TYPE => Some(&self::common_type::COMMON_TYPE_DEF),
        _ => None
    }
}

impl Types {
    pub fn new() -> Types {
        let mut types = Types { map: BTreeMap::new() };
        types.insert(WireType::from_type(TypeId::ARRAY_TYPE, &self::array_type::ARRAY_TYPE_DEF).unwrap());
        types.insert(WireType::from_type(TypeId::MAP_TYPE, &self::map_type::MAP_TYPE_DEF).unwrap());
        types.insert(WireType::from_type(TypeId::SLICE_TYPE, &self::slice_type::SLICE_TYPE_DEF).unwrap());
        types.insert(WireType::from_type(TypeId::FIELD_TYPE, &self::struct_type::FIELD_TYPE_DEF).unwrap());
        types.insert(WireType::from_type(TypeId::FIELD_TYPE_SLICE, &self::struct_type::FIELD_TYPE_SLICE_DEF).unwrap());
        types.insert(WireType::from_type(TypeId::STRUCT_TYPE, &self::struct_type::STRUCT_TYPE_DEF).unwrap());
        types.insert(WireType::from_type(TypeId::WIRE_TYPE, &self::wire_type::WIRE_TYPE_DEF).unwrap());
        types.insert(WireType::from_type(TypeId::COMMON_TYPE, &self::common_type::COMMON_TYPE_DEF).unwrap());
        types
    }

    pub(crate) fn insert(&mut self, def: WireType) {
        self.map.insert(def.common().id, def);
    }

    pub(crate) fn lookup(&self, id: TypeId) -> Option<&WireType> {
        self.map.get(&id)
    }
}
