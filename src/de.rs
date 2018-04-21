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
}

impl<'de> serde::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        let type_id = self.msg.read_int()?;
        let type_tag = self.msg.read_int()?;
        if type_tag == 0 {
            match type_id {
                1 => visitor.visit_bool(self.msg.read_bool()?),
                2 => visitor.visit_i64(self.msg.read_int()?),
                3 => visitor.visit_u64(self.msg.read_uint()?),
                4 => visitor.visit_f64(self.msg.read_float()?),
                5 => visitor.visit_bytes(self.msg.read_bytes()?),
                6 => {
                    let bytes = self.msg.read_bytes()?;
                    let string = ::std::str::from_utf8(bytes)
                        .map_err(|err| <Error as serde::de::Error>::custom(err))?;
                    visitor.visit_str(string)
                },
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

#[cfg(test)]
mod tests {
    extern crate serde_bytes;
    use serde::Deserialize;
    use self::serde_bytes::ByteBuf;
    use super::Deserializer;

    #[test]
    fn bool_true() {
        let deserializer = Deserializer::from_slice(&[2, 0, 1]);
        let decoded = bool::deserialize(deserializer).unwrap();
        assert_eq!(decoded, true);
    }

    #[test]
    fn bool_false() {
        let deserializer = Deserializer::from_slice(&[2, 0, 0]);
        let decoded = bool::deserialize(deserializer).unwrap();
        assert_eq!(decoded, false);
    }

    #[test]
    fn u64_zero() {
        let deserializer = Deserializer::from_slice(&[6, 0, 0]);
        let decoded = u64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 0);
    }

    #[test]
    fn u64_small() {
        let deserializer = Deserializer::from_slice(&[6, 0, 42]);
        let decoded = u64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 42);
    }

    #[test]
    fn u64_big() {
        let deserializer = Deserializer::from_slice(&[6, 0, 254, 4, 210]);
        let decoded = u64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 1234);
    }

    #[test]
    fn i64_zero() {
        let deserializer = Deserializer::from_slice(&[4, 0, 0]);
        let decoded = i64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 0);
    }

    #[test]
    fn i64_small_pos() {
        let deserializer = Deserializer::from_slice(&[4, 0, 84]);
        let decoded = i64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 42);
    }

    #[test]
    fn i64_small_neg() {
        let deserializer = Deserializer::from_slice(&[4, 0, 83]);
        let decoded = i64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, -42);
    }

    #[test]
    fn i64_big_pos() {
        let deserializer = Deserializer::from_slice(&[4, 0, 254, 9, 164]);
        let decoded = i64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 1234);
    }

    #[test]
    fn i64_big_neg() {
        let deserializer = Deserializer::from_slice(&[4, 0, 254, 9, 163]);
        let decoded = i64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, -1234);
    }

    #[test]
    fn f64_zero() {
        let deserializer = Deserializer::from_slice(&[8, 0, 0]);
        let decoded = f64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 0f64);
    }

    #[test]
    fn f64_pos() {
        let deserializer = Deserializer::from_slice(&[8, 0, 254, 69, 64]);
        let decoded = f64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, 42f64);
    }

    #[test]
    fn f64_neg() {
        let deserializer = Deserializer::from_slice(&[8, 0, 254, 69, 192]);
        let decoded = f64::deserialize(deserializer).unwrap();
        assert_eq!(decoded, -42f64);
    }

    #[test]
    fn bytebuf_empty() {
        let deserializer = Deserializer::from_slice(&[10, 0, 0]);
        let decoded = ByteBuf::deserialize(deserializer).unwrap();
        assert_eq!(&*decoded, &[]);
    }

    #[test]
    fn bytebuf_non_empty() {
        let deserializer = Deserializer::from_slice(&[10, 0, 4, 1, 2, 3, 4]);
        let decoded = ByteBuf::deserialize(deserializer).unwrap();
        assert_eq!(&*decoded, &[1, 2, 3, 4]);
    }

    #[test]
    fn string_empty() {
        let deserializer = Deserializer::from_slice(&[12, 0, 0]);
        let decoded = String::deserialize(deserializer).unwrap();
        assert_eq!(decoded, "");
    }

    #[test]
    fn string_non_empty() {
        let deserializer = Deserializer::from_slice(&[12, 0, 3, 102, 111, 111]);
        let decoded = String::deserialize(deserializer).unwrap();
        assert_eq!(decoded, "foo");
    }
}
