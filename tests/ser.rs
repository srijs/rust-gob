extern crate gob;
extern crate serde;
extern crate serde_bytes;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use gob::{Schema, TypeId};
use gob::ser::Serializer;
use serde::Serialize;
use serde_bytes::Bytes;

#[test]
fn bool_true() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::BOOL, &mut buffer);
        true.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 1]);
}

#[test]
fn bool_false() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::BOOL,  &mut buffer);
        false.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 0]);
}

#[test]
fn u8_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::UINT, &mut buffer);
        0u8.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u16_zero() {
        let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::UINT, &mut buffer);
        0u16.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u32_zero() {
        let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::UINT, &mut buffer);
        0u32.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_zero() {
        let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::UINT, &mut buffer);
        0u64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_small() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::UINT, &mut buffer);
        42u8.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 42]);
}

#[test]
fn u64_big() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::UINT, &mut buffer);
        1234u64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 6, 0, 254, 4, 210]);
}

#[test]
fn u64_max() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::UINT, &mut buffer);
        ::std::u64::MAX.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[11, 6, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn i8_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        0i8.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i16_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        0i16.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i32_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        0i32.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        0i64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_small_pos() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        42i64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 84]);
}

#[test]
fn i64_small_neg() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        (-42i64).serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 83]);
}

#[test]
fn i64_big_pos() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        1234i64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 164]);
}

#[test]
fn i64_big_neg() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        (-1234i64).serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 163]);
}

#[test]
fn i64_min() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        ::std::i64::MIN.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn i64_max() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        ::std::i64::MAX.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 254]);
}

#[test]
fn f32_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::FLOAT, &mut buffer);
        0f32.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::FLOAT, &mut buffer);
        0f64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_pos() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::FLOAT, &mut buffer);
        42f64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 64]);
}

#[test]
fn f64_neg() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::FLOAT, &mut buffer);
        (-42f64).serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 192]);
}

#[test]
fn char_ascii() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        'f'.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[4, 4, 0, 255, 204]);
}

#[test]
fn char_unicode() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::INT, &mut buffer);
        '語'.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[6, 4, 0, 253, 1, 21, 60]);
}

#[test]
fn bytes_empty() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::BYTES, &mut buffer);
        let bytes = Bytes::new(&[]);
        bytes.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 10, 0, 0]);
}

#[test]
fn bytes_non_empty() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::BYTES, &mut buffer);
        let bytes = Bytes::new(&[1, 2, 3, 4]);
        bytes.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[7, 10, 0, 4, 1, 2, 3, 4]);
}

#[test]
fn str_empty() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::STRING, &mut buffer);
        "".serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 12, 0, 0]);
}

#[test]
fn str_non_empty() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(TypeId::STRING, &mut buffer);
        "foo".serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[6, 12, 0, 3, 102, 111, 111]);
}

#[test]
fn vec_of_bool_empty() {
    let mut buffer = Vec::new();
    let mut schema = Schema::new();
    let id = schema.register_slice_type(TypeId::BOOL);
    {
        let serializer = Serializer::with_schema(id, &mut schema, &mut buffer);
        Vec::<bool>::new().serialize(serializer).unwrap();
    }
    assert_eq!(buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 4, 255, 130, 0, 0]);
}

#[test]
fn vec_of_bool_non_empty() {
    let mut buffer = Vec::new();
    let mut schema = Schema::new();
    let id = schema.register_slice_type(TypeId::BOOL);
    {
        let serializer = Serializer::with_schema(id, &mut schema, &mut buffer);
        vec![true, false].serialize(serializer).unwrap();
    }
    assert_eq!(buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 6, 255, 130, 0, 2, 1, 0]);
}

#[test]
fn point_struct() {
    #[derive(Serialize)]
    struct Point {
        #[serde(rename = "X")] x: i64,
        #[serde(rename = "Y")] y: i64
    }

    let mut buffer = Vec::new();
    let mut schema = Schema::new();
    let id = schema.register_struct_type("Point")
        .field("X", TypeId::INT)
        .field("Y", TypeId::INT)
        .finish();
    {
        let serializer = Serializer::with_schema(id, &mut schema, &mut buffer);
        (Point { x: 22, y: 33}).serialize(serializer).unwrap();
    }
    assert_eq!(buffer, [
        0x1f, 0xff, 0x81, 0x03, 0x01, 0x01, 0x05, 0x50,
        0x6f, 0x69, 0x6e, 0x74, 0x01, 0xff, 0x82, 0x00,
        0x01, 0x02, 0x01, 0x01, 0x58, 0x01, 0x04, 0x00,
        0x01, 0x01, 0x59, 0x01, 0x04, 0x00, 0x00, 0x00,
        0x07, 0xff, 0x82, 0x01, 0x2c, 0x01, 0x42, 0x00
    ].as_ref());
}
