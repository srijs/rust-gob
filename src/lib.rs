#![deny(warnings)]

extern crate bytes;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_schema;
extern crate smallvec;

mod internal;

pub mod de;
mod schema;
pub mod ser;

pub use de::Deserializer;
pub use ser::StreamSerializer;
