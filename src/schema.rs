//! Schema management

use std::io::Write;

use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};
use serde_schema::types::Type;

use error::Error;
use internal::gob::Stream;
use internal::ser::SerializeWireTypes;
use internal::utils::UniqMap;

pub struct Schema {
    pending_wire_types: Vec<(TypeId, Vec<u8>)>,
    next_type_id: TypeId,
    schema_types: UniqMap<TypeId, Type<TypeId>>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            pending_wire_types: Vec::new(),
            next_type_id: TypeId(65),
            schema_types: UniqMap::new(),
        }
    }

    pub(crate) fn lookup(&self, id: TypeId) -> Option<&Type<TypeId>> {
        ::internal::types::lookup_builtin(id).or_else(|| self.schema_types.get(&id))
    }

    pub(crate) fn write_pending<W: Write>(&mut self, mut out: Stream<W>) -> Result<(), Error> {
        for (type_id, wire_type_buffer) in self.pending_wire_types.drain(..) {
            out.write_section(-type_id.0, &wire_type_buffer)?;
        }
        Ok(())
    }
}

impl ::serde_schema::Schema for Schema {
    type TypeId = TypeId;
    type Error = Error;

    fn register_type(&mut self, ty: Type<TypeId>) -> Result<TypeId, Error> {
        let next_id = self.next_type_id;

        if let Type::Option(ref option_type) = ty {
            return Ok(*option_type.inner_type());
        }

        if let Some(id) = self.schema_types.insert(next_id, ty) {
            return Ok(id);
        }

        let delta = SerializeWireTypes::new(&mut self.pending_wire_types)
            .serialize_wire_types(next_id, self.schema_types.get(&next_id).unwrap())?;

        self.next_type_id = TypeId((self.next_type_id.0 as usize + delta) as i64);

        Ok(next_id)
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
    pub(crate) const WIRE_TYPE: TypeId = TypeId(16);
    pub(crate) const ARRAY_TYPE: TypeId = TypeId(17);
    pub(crate) const COMMON_TYPE: TypeId = TypeId(18);
    pub(crate) const SLICE_TYPE: TypeId = TypeId(19);
    pub(crate) const STRUCT_TYPE: TypeId = TypeId(20);
    pub(crate) const FIELD_TYPE: TypeId = TypeId(21);
    pub(crate) const FIELD_TYPE_SLICE: TypeId = TypeId(22);
    pub(crate) const MAP_TYPE: TypeId = TypeId(23);

    pub(crate) fn next(&self) -> TypeId {
        TypeId(self.0 + 1)
    }
}

impl ::serde_schema::types::TypeId for TypeId {
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
            return Err(::serde::ser::Error::custom(format!(
                "invalid type id {}",
                self.0
            )));
        }
        self.0.serialize(ser)
    }
}
