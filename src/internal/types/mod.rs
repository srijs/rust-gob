use std::collections::BTreeMap;

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
    custom: BTreeMap<TypeId, WireType>
}

impl Types {
    pub fn new() -> Types {
        Types { custom: BTreeMap::new() }
    }

    pub(crate) fn insert(&mut self, def: WireType) {
        self.custom.insert(def.common().id, def);
    }

    pub(crate) fn lookup(&self, id: TypeId) -> Option<&WireType> {
        match id {
            TypeId::ARRAY_TYPE => Some(&self::array_type::ARRAY_TYPE_DEF),
            TypeId::MAP_TYPE => Some(&self::map_type::MAP_TYPE_DEF),
            TypeId::SLICE_TYPE => Some(&self::slice_type::SLICE_TYPE_DEF),
            TypeId::FIELD_TYPE => Some(&self::struct_type::FIELD_TYPE_DEF),
            TypeId::FIELD_TYPE_SLICE => Some(&self::struct_type::FIELD_TYPE_SLICE_DEF),
            TypeId::STRUCT_TYPE => Some(&self::struct_type::STRUCT_TYPE_DEF),
            TypeId::WIRE_TYPE => Some(&self::wire_type::WIRE_TYPE_DEF),
            TypeId::COMMON_TYPE => Some(&self::common_type::COMMON_TYPE_DEF),
            _ => self.custom.get(&id)
        }
    }

    pub(crate) fn custom(&self, last_type_id: Option<TypeId>) -> CustomTypes {
        if let Some(type_id) = last_type_id {
            let next_type_id = TypeId(type_id.0 + 1);
            CustomTypes(self.custom.range(::std::ops::RangeFrom { start: next_type_id }))
        } else {
            CustomTypes(self.custom.range(::std::ops::RangeFull))
        }
    }
}

pub(crate) struct CustomTypes<'a>(::std::collections::btree_map::Range<'a, TypeId, WireType>);

impl<'a> Iterator for CustomTypes<'a> {
    type Item = &'a WireType;
    fn next(&mut self) -> Option<&'a WireType> {
        self.0.next().map(|(_, t)| t)
    }
}
