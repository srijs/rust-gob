extern crate bytes;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;

mod internal;

mod schema;
pub mod ser;
pub mod de;

pub use de::Deserializer;
pub use ser::StreamSerializer;
