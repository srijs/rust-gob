extern crate serde;

#[cfg(feature = "bytes")]
extern crate serde_bytes;

mod types;
pub use self::types::{TypeId, StructField, EnumVariant, Type};

mod schema;
pub use self::schema::Schema;

mod serializer;
pub use self::serializer::SchemaSerializer;

mod serialize;
pub use self::serialize::SchemaSerialize;
