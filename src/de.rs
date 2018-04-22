use std::io::Cursor;

use serde;
use serde::de::Visitor;
use serde::de::value::Error;

use ::gob::Message;

impl From<::gob::Error> for Error {
    fn from(err: ::gob::Error) -> Error {
        serde::de::Error::custom(format!("{:?}", err))
    }
}

pub struct Deserializer<'de> {
    msg: Message<Cursor<&'de [u8]>>
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Deserializer<'de> {
        Deserializer {
            msg: Message::new(Cursor::new(input))
        }
    }

    fn deserialize_byte_slice(&mut self) -> Result<&'de [u8], Error> {
        let len = self.msg.read_bytes_len()?;
        let pos = self.msg.get_ref().position() as usize;
        let bytes = &self.msg.get_ref().get_ref()[pos..pos+len];
        Ok(bytes)
    }

    fn deserialize_str_slice(&mut self) -> Result<&'de str, Error> {
        let bytes = self.deserialize_byte_slice()?;
        ::std::str::from_utf8(bytes)
            .map_err(|err| serde::de::Error::custom(err))
    }
}

impl<'de> serde::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        let _ = self.msg.read_bytes_len()?;
        let type_id = self.msg.read_int()?;
        let type_tag = self.msg.read_int()?;
        if type_tag == 0 {
            match type_id {
                1 => visitor.visit_bool(self.msg.read_bool()?),
                2 => visitor.visit_i64(self.msg.read_int()?),
                3 => visitor.visit_u64(self.msg.read_uint()?),
                4 => visitor.visit_f64(self.msg.read_float()?),
                5 => visitor.visit_borrowed_bytes(self.deserialize_byte_slice()?),
                6 => visitor.visit_borrowed_str(self.deserialize_str_slice()?),
                _ => Err(serde::de::Error::custom(format!("unknown type id {}", type_id)))
            }
        } else {
            Err(serde::de::Error::custom(format!("unknown type tag {}", type_tag)))
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
