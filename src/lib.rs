#![deny(warnings)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate byteorder;
extern crate bytes;
extern crate iovec;
#[macro_use]
extern crate lazy_static;
extern crate owning_ref;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_schema;
extern crate smallvec;

mod internal;
mod schema;

pub mod error;

pub mod de;
pub mod ser;

pub use error::Error;

pub use de::{Deserializer, StreamDeserializer};
pub use ser::StreamSerializer;
