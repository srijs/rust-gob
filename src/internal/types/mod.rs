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

pub(crate) fn lookup_builtin2(id: TypeId) -> Option<&'static WireType> {
    match id {
        TypeId::ARRAY_TYPE => Some(&self::array_type::ARRAY_TYPE_DEF_2),
        TypeId::MAP_TYPE => Some(&self::map_type::MAP_TYPE_DEF_2),
        TypeId::SLICE_TYPE => Some(&self::slice_type::SLICE_TYPE_DEF_2),
        TypeId::FIELD_TYPE => Some(&self::struct_type::FIELD_TYPE_DEF_2),
        TypeId::FIELD_TYPE_SLICE => Some(&self::struct_type::FIELD_TYPE_SLICE_DEF_2),
        TypeId::STRUCT_TYPE => Some(&self::struct_type::STRUCT_TYPE_DEF_2),
        TypeId::WIRE_TYPE => Some(&self::wire_type::WIRE_TYPE_DEF_2),
        TypeId::COMMON_TYPE => Some(&self::common_type::COMMON_TYPE_DEF_2),
        _ => None
    }
}

impl Types {
    pub fn new() -> Types {
        Types { map: BTreeMap::new() }
    }

    pub(crate) fn insert(&mut self, def: WireType) {
        self.map.insert(def.common().id, def);
    }

    pub(crate) fn lookup(&self, id: TypeId) -> Option<&WireType> {
        lookup_builtin2(id).or_else(|| self.map.get(&id))
    }
}
