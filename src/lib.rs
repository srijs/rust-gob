extern crate bytes;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;

mod internal;

pub mod schema;
pub mod ser;
pub mod de;

pub use schema::{Schema, TypeId};
pub use ser::StreamSerializer;
pub use de::Deserializer;
