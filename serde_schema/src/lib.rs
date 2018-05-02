extern crate serde;

#[cfg(feature = "bytes")]
extern crate serde_bytes;

pub mod types;

mod schema;
pub use self::schema::Schema;

mod serializer;
pub use self::serializer::SchemaSerializer;

mod serialize;
pub use self::serialize::SchemaSerialize;
