extern crate bytes;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;

mod gob;
mod types;
mod de;

pub use de::Deserializer;
