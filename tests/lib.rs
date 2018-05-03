#[macro_use]
extern crate duct;
extern crate gob;
extern crate serde;
extern crate serde_bytes;
extern crate tempfile;
#[macro_use]
extern crate serde_derive;
extern crate serde_schema;
#[macro_use]
extern crate failure;

#[macro_use]
mod utils;
mod de;
mod ser;
