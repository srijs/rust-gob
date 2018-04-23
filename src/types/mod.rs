use std::collections::HashMap;

mod wire_type;
pub use self::wire_type::WireType;

mod common_type;
pub use self::common_type::CommonType;

mod array_type;
pub use self::array_type::ArrayType;

mod slice_type;
pub use self::slice_type::SliceType;

mod struct_type;
pub use self::struct_type::{FieldType, Fields, StructType};

mod map_type;
pub use self::map_type::MapType;

mod type_id;
pub use self::type_id::TypeId;

#[derive(Debug)]
pub(crate) struct TypeDefs {
    map: HashMap<TypeId, WireType>
}

impl TypeDefs {
    pub fn new() -> TypeDefs {
        let mut defs = TypeDefs { map: HashMap::new() };
        defs.insert(WireType::def());
        defs.insert(CommonType::def());
        defs.insert(ArrayType::def());
        defs.insert(SliceType::def());
        defs.insert(FieldType::def());
        defs.insert(Fields::def());
        defs.insert(StructType::def());
        defs.insert(MapType::def());
        defs
    }

    pub fn insert(&mut self, def: WireType) {
        self.map.insert(def.common().id, def);
    }

    pub fn lookup(&self, id: TypeId) -> Option<&WireType> {
        self.map.get(&id)
    }
}
