use serde::ser::{Error, Serializer};

use ::Schema;
use ::types::TypeId;

pub trait SchemaSerializer {
    type Ok;
    type Error: Error;
    type TypeId: TypeId;
    type Schema: Schema<TypeId = Self::TypeId, Error = Self::Error>;
    type Serializer: Serializer<Ok = Self::Ok, Error = Self::Error>;

    fn schema_mut(&mut self) -> &mut Self::Schema;
    fn serializer(self, id: Self::TypeId) -> Result<Self::Serializer, Self::Error>;
}
