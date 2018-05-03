use serde;
use serde::de::value::Error;

mod complex_value;
mod field_value;
mod map_value;
mod seq_value;
mod struct_value;
mod value;

pub(crate) use self::field_value::FieldValueDeserializer;
pub(crate) use self::value::ValueDeserializer;

impl From<::internal::gob::Error> for Error {
    fn from(err: ::internal::gob::Error) -> Error {
        serde::de::Error::custom(format!("{:?}", err))
    }
}
