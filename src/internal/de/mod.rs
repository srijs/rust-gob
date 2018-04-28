use std::io::Cursor;

use serde::{self, Deserialize};
use serde::de::Visitor;
use serde::de::value::Error;

use ::internal::gob::Message;
use ::internal::types::{TypeId, Types, WireType};

mod field_value;
mod struct_value;
mod slice_value;
mod array_value;
mod map_value;
mod complex_value;
mod value;

pub(crate) use self::field_value::FieldValueDeserializer;
pub(crate) use self::value::ValueDeserializer;

impl From<::internal::gob::Error> for Error {
    fn from(err: ::internal::gob::Error) -> Error {
        serde::de::Error::custom(format!("{:?}", err))
    }
}
