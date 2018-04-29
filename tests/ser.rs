extern crate gob;
extern crate serde;
extern crate serde_bytes;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use gob::StreamSerializer;
use gob::ser::TypeId;
use serde_bytes::Bytes;

#[test]
fn bool_true() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::BOOL, &true).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 1]);
}

#[test]
fn bool_false() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::BOOL, &false).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 0]);
}

#[test]
fn u8_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::UINT, &0u8).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u16_zero() {
        let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::UINT, &0u16).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u32_zero() {
        let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::UINT, &0u32).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_zero() {
        let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::UINT, &0u64).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_small() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::UINT, &42u64).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 42]);
}

#[test]
fn u64_big() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::UINT, &1234u64).unwrap();
    }
    assert_eq!(buffer, &[5, 6, 0, 254, 4, 210]);
}

#[test]
fn u64_max() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::UINT, &::std::u64::MAX).unwrap();
    }
    assert_eq!(buffer, &[11, 6, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn i8_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &0i8).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i16_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &0i16).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i32_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &0i32).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &0i64).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_small_pos() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &42i64).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 84]);
}

#[test]
fn i64_small_neg() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &-42i64).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 83]);
}

#[test]
fn i64_big_pos() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &1234i64).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 164]);
}

#[test]
fn i64_big_neg() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &-1234i64).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 163]);
}

#[test]
fn i64_min() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &::std::i64::MIN).unwrap();
    }
    assert_eq!(buffer, &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn i64_max() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &::std::i64::MAX).unwrap();
    }
    assert_eq!(buffer, &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 254]);
}

#[test]
fn f32_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::FLOAT, &0f32).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::FLOAT, &0f64).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_pos() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::FLOAT, &42f64).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 64]);
}

#[test]
fn f64_neg() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::FLOAT, &-42f64).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 192]);
}

#[test]
fn char_ascii() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &'f').unwrap();
    }
    assert_eq!(buffer, &[4, 4, 0, 255, 204]);
}

#[test]
fn char_unicode() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::INT, &'語').unwrap();
    }
    assert_eq!(buffer, &[6, 4, 0, 253, 1, 21, 60]);
}

#[test]
fn bytes_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::BYTES, &Bytes::new(&[])).unwrap();
    }
    assert_eq!(buffer, &[3, 10, 0, 0]);
}

#[test]
fn bytes_non_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::BYTES, &Bytes::new(&[1, 2, 3, 4])).unwrap();
    }
    assert_eq!(buffer, &[7, 10, 0, 4, 1, 2, 3, 4]);
}

#[test]
fn str_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::STRING, &"").unwrap();
    }
    assert_eq!(buffer, &[3, 12, 0, 0]);
}

#[test]
fn str_non_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(TypeId::STRING, &"foo").unwrap();
    }
    assert_eq!(buffer, &[6, 12, 0, 3, 102, 111, 111]);
}

#[test]
fn vec_of_bool_to_empty_slice() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        let id = stream.schema_mut().register_slice_type(TypeId::BOOL);
        stream.serialize(id, &Vec::<bool>::new()).unwrap();
    }
    assert_eq!(buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 4, 255, 130, 0, 0]);
}

#[test]
fn vec_of_bool_to_non_empty_slice() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        let id = stream.schema_mut().register_slice_type(TypeId::BOOL);
        stream.serialize(id, &vec![true, false]).unwrap();
    }
    assert_eq!(buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 6, 255, 130, 0, 2, 1, 0]);
}

#[test]
fn vec_of_bool_to_empty_array() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        let id = stream.schema_mut().register_array_type(TypeId::BOOL, 0);
        stream.serialize(id, &Vec::<bool>::new()).unwrap();
    }
    assert_eq!(buffer,
        &[12, 255, 129, 1, 1, 2, 255, 130, 0, 1, 2, 0, 0, 4, 255, 130, 0, 0]);
}

#[test]
fn vec_of_bool_to_non_empty_array() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        let id = stream.schema_mut().register_array_type(TypeId::BOOL,  2);
        stream.serialize(id, &vec![true, false]).unwrap();
    }
    assert_eq!(buffer,
        &[14, 255, 129, 1, 1, 2, 255, 130, 0, 1, 2, 1, 4, 0, 0, 6, 255, 130, 0, 2, 1, 0]);
}

#[test]
fn vec_of_bool_to_empty_slice_twice() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        let id = stream.schema_mut().register_slice_type(TypeId::BOOL);
        stream.serialize(id, &Vec::<bool>::new()).unwrap();
        stream.serialize(id, &Vec::<bool>::new()).unwrap();
    }
    assert_eq!(buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 4, 255, 130, 0, 0, 4, 255, 130, 0, 0]);
}

#[test]
fn point_struct() {
    #[derive(Serialize)]
    struct Point {
        #[serde(rename = "X")] x: i64,
        #[serde(rename = "Y")] y: i64
    }

    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        let id = stream.schema_mut().register_struct_type("Point")
            .field("X", TypeId::INT)
            .field("Y", TypeId::INT)
            .finish();
        stream.serialize(id, &Point { x: 22, y: 33 }).unwrap();
    }
    assert_eq!(buffer, [
        0x1f, 0xff, 0x81, 0x03, 0x01, 0x01, 0x05, 0x50,
        0x6f, 0x69, 0x6e, 0x74, 0x01, 0xff, 0x82, 0x00,
        0x01, 0x02, 0x01, 0x01, 0x58, 0x01, 0x04, 0x00,
        0x01, 0x01, 0x59, 0x01, 0x04, 0x00, 0x00, 0x00,
        0x07, 0xff, 0x82, 0x01, 0x2c, 0x01, 0x42, 0x00
    ].as_ref());
}
