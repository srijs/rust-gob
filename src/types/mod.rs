use std::collections::BTreeMap;

mod wire_type;
pub use self::wire_type::WireType;

mod common_type;
pub use self::common_type::CommonType;

mod array_type;
pub use self::array_type::ArrayType;

mod slice_type;
pub use self::slice_type::SliceType;

mod struct_type;
pub use self::struct_type::{FieldType, StructType};

mod map_type;
pub use self::map_type::MapType;

mod type_id;
pub use self::type_id::TypeId;

#[derive(Debug)]
pub(crate) struct TypeDefs {
    map: BTreeMap<TypeId, WireType>
}

impl TypeDefs {
    pub fn new() -> TypeDefs {
        TypeDefs { map: BTreeMap::new() }
    }

    pub fn insert(&mut self, def: WireType) {
        self.map.insert(def.common().id, def);
    }

    pub fn lookup(&self, id: TypeId) -> Option<&WireType> {
        match id {
            TypeId::ARRAY_TYPE => Some(&self::array_type::ARRAY_TYPE_DEF),
            TypeId::MAP_TYPE => Some(&self::map_type::MAP_TYPE_DEF),
            TypeId::SLICE_TYPE => Some(&self::slice_type::SLICE_TYPE_DEF),
            TypeId::FIELD_TYPE => Some(&self::struct_type::FIELD_TYPE_DEF),
            TypeId::FIELD_TYPE_SLICE => Some(&self::struct_type::FIELD_TYPE_SLICE_DEF),
            TypeId::STRUCT_TYPE => Some(&self::struct_type::STRUCT_TYPE_DEF),
            TypeId::WIRE_TYPE => Some(&self::wire_type::WIRE_TYPE_DEF),
            TypeId::COMMON_TYPE => Some(&self::common_type::COMMON_TYPE_DEF),
            _ => self.map.get(&id)
        }
    }
}
