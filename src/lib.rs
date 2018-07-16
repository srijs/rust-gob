#![deny(warnings)]

#[cfg(test)]
extern crate partial_io;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate byteorder;
extern crate bytes;
extern crate iovec;
#[macro_use]
extern crate lazy_static;
extern crate owning_ref;
extern crate safemem;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_schema;

mod internal;
mod schema;

pub mod error;

pub mod de;
pub mod ser;

pub use error::Error;

pub use de::{Deserializer, StreamDeserializer};
pub use ser::StreamSerializer;
