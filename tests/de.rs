extern crate gob;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::io::Cursor;

use gob::{Deserializer, StreamDeserializer};
use serde::Deserialize;
use serde_bytes::{ByteBuf, Bytes};

#[test]
fn bool_true() {
    let deserializer = Deserializer::from_slice(&[3, 2, 0, 1]);
    let decoded = bool::deserialize(deserializer).unwrap();
    assert_eq!(decoded, true);
}

#[test]
fn bool_false() {
    let deserializer = Deserializer::from_slice(&[3, 2, 0, 0]);
    let decoded = bool::deserialize(deserializer).unwrap();
    assert_eq!(decoded, false);
}

#[test]
fn u8_zero() {
    let deserializer = Deserializer::from_slice(&[3, 6, 0, 0]);
    let decoded = u8::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn u8_overflow() {
    let deserializer = Deserializer::from_slice(&[5, 6, 0, 254, 4, 210]);
    let result = u8::deserialize(deserializer);
    assert!(result.is_err());
}

#[test]
fn u16_zero() {
    let deserializer = Deserializer::from_slice(&[3, 6, 0, 0]);
    let decoded = u16::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn u32_zero() {
    let deserializer = Deserializer::from_slice(&[3, 6, 0, 0]);
    let decoded = u32::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn u64_zero() {
    let deserializer = Deserializer::from_slice(&[3, 6, 0, 0]);
    let decoded = u64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn u64_small() {
    let deserializer = Deserializer::from_slice(&[3, 6, 0, 42]);
    let decoded = u64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 42);
}

#[test]
fn u64_big() {
    let deserializer = Deserializer::from_slice(&[5, 6, 0, 254, 4, 210]);
    let decoded = u64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 1234);
}

#[test]
fn u64_max() {
    let deserializer =
        Deserializer::from_slice(&[11, 6, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
    let decoded = u64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, ::std::u64::MAX);
}

#[test]
fn i8_zero() {
    let deserializer = Deserializer::from_slice(&[3, 4, 0, 0]);
    let decoded = i8::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn i8_overflow() {
    let deserializer = Deserializer::from_slice(&[5, 4, 0, 254, 4, 210]);
    let result = i8::deserialize(deserializer);
    assert!(result.is_err());
}

#[test]
fn i16_zero() {
    let deserializer = Deserializer::from_slice(&[3, 4, 0, 0]);
    let decoded = i16::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn i32_zero() {
    let deserializer = Deserializer::from_slice(&[3, 4, 0, 0]);
    let decoded = i32::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn i64_zero() {
    let deserializer = Deserializer::from_slice(&[3, 4, 0, 0]);
    let decoded = i64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0);
}

#[test]
fn i64_small_pos() {
    let deserializer = Deserializer::from_slice(&[3, 4, 0, 84]);
    let decoded = i64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 42);
}

#[test]
fn i64_small_neg() {
    let deserializer = Deserializer::from_slice(&[3, 4, 0, 83]);
    let decoded = i64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, -42);
}

#[test]
fn i64_big_pos() {
    let deserializer = Deserializer::from_slice(&[5, 4, 0, 254, 9, 164]);
    let decoded = i64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 1234);
}

#[test]
fn i64_big_neg() {
    let deserializer = Deserializer::from_slice(&[5, 4, 0, 254, 9, 163]);
    let decoded = i64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, -1234);
}

#[test]
fn i64_min() {
    let deserializer =
        Deserializer::from_slice(&[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
    let decoded = i64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, ::std::i64::MIN);
}

#[test]
fn i64_max() {
    let deserializer =
        Deserializer::from_slice(&[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 254]);
    let decoded = i64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, ::std::i64::MAX);
}

#[test]
fn f32_zero() {
    let deserializer = Deserializer::from_slice(&[3, 8, 0, 0]);
    let decoded = f32::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0f32);
}

#[test]
fn f64_zero() {
    let deserializer = Deserializer::from_slice(&[3, 8, 0, 0]);
    let decoded = f64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 0f64);
}

#[test]
fn f64_pos() {
    let deserializer = Deserializer::from_slice(&[5, 8, 0, 254, 69, 64]);
    let decoded = f64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 42f64);
}

#[test]
fn f64_neg() {
    let deserializer = Deserializer::from_slice(&[5, 8, 0, 254, 69, 192]);
    let decoded = f64::deserialize(deserializer).unwrap();
    assert_eq!(decoded, -42f64);
}

#[test]
fn char_ascii() {
    let deserializer = Deserializer::from_slice(&[4, 4, 0, 255, 204]);
    let decoded = char::deserialize(deserializer).unwrap();
    assert_eq!(decoded, 'f');
}

#[test]
fn char_unicode() {
    let deserializer = Deserializer::from_slice(&[6, 4, 0, 253, 1, 21, 60]);
    let decoded = char::deserialize(deserializer).unwrap();
    assert_eq!(decoded, '語');
}

#[test]
fn bytes_empty() {
    let deserializer = Deserializer::from_slice(&[3, 10, 0, 0]);
    let decoded = Bytes::deserialize(deserializer).unwrap();
    assert_eq!(&*decoded, &[]);
}

#[test]
fn bytes_non_empty() {
    let deserializer = Deserializer::from_slice(&[7, 10, 0, 4, 1, 2, 3, 4]);
    let decoded = Bytes::deserialize(deserializer).unwrap();
    assert_eq!(&*decoded, &[1, 2, 3, 4]);
}

#[test]
fn bytebuf_empty() {
    let deserializer = Deserializer::from_slice(&[3, 10, 0, 0]);
    let decoded = ByteBuf::deserialize(deserializer).unwrap();
    assert_eq!(&*decoded, &[]);
}

#[test]
fn bytebuf_non_empty() {
    let deserializer = Deserializer::from_slice(&[7, 10, 0, 4, 1, 2, 3, 4]);
    let decoded = ByteBuf::deserialize(deserializer).unwrap();
    assert_eq!(&*decoded, &[1, 2, 3, 4]);
}

#[test]
fn str_empty() {
    let deserializer = Deserializer::from_slice(&[3, 12, 0, 0]);
    let decoded = <&str>::deserialize(deserializer).unwrap();
    assert_eq!(decoded, "");
}

#[test]
fn str_non_empty() {
    let deserializer = Deserializer::from_slice(&[6, 12, 0, 3, 102, 111, 111]);
    let decoded = <&str>::deserialize(deserializer).unwrap();
    assert_eq!(decoded, "foo");
}

#[test]
fn string_empty() {
    let deserializer = Deserializer::from_slice(&[3, 12, 0, 0]);
    let decoded = String::deserialize(deserializer).unwrap();
    assert_eq!(decoded, "");
}

#[test]
fn string_non_empty() {
    let deserializer = Deserializer::from_slice(&[6, 12, 0, 3, 102, 111, 111]);
    let decoded = String::deserialize(deserializer).unwrap();
    assert_eq!(decoded, "foo");
}

#[test]
fn vec_of_bool_from_empty_slice() {
    let deserializer =
        Deserializer::from_slice(include_bytes!("reference/output/slice_of_bool_empty.gob"));
    let decoded = <Vec<bool>>::deserialize(deserializer).unwrap();
    assert_eq!(decoded, &[]);
}

#[test]
fn vec_of_bool_from_empty_array() {
    let deserializer =
        Deserializer::from_slice(include_bytes!("reference/output/array_of_bool_empty.gob"));
    let decoded = <Vec<bool>>::deserialize(deserializer).unwrap();
    assert_eq!(decoded, &[]);
}

#[test]
fn vec_of_bool_from_non_empty_slice() {
    let deserializer = Deserializer::from_slice(include_bytes!(
        "reference/output/slice_of_bool_non_empty.gob"
    ));
    let decoded = <Vec<bool>>::deserialize(deserializer).unwrap();
    assert_eq!(decoded, &[true, false]);
}

#[test]
fn vec_of_bool_from_non_empty_array() {
    let deserializer = Deserializer::from_slice(include_bytes!(
        "reference/output/array_of_bool_non_empty.gob"
    ));
    let decoded = <Vec<bool>>::deserialize(deserializer).unwrap();
    assert_eq!(decoded, &[true, false]);
}

#[test]
fn vec_of_bool_from_empty_slice_twice() {
    let buffer = include_bytes!("reference/output/slice_of_bool_empty_twice.gob");

    let cursor = Cursor::new(buffer);
    let mut stream = StreamDeserializer::new(cursor);

    let decoded1 = stream.deserialize::<Vec<bool>>().unwrap().unwrap();
    assert_eq!(decoded1, &[]);

    let decoded2 = stream.deserialize::<Vec<bool>>().unwrap().unwrap();
    assert_eq!(decoded2, &[]);
}

#[test]
fn vec_of_bool_from_non_empty_slice_twice() {
    let buffer = include_bytes!("reference/output/slice_of_bool_non_empty_twice.gob");

    let cursor = Cursor::new(buffer);
    let mut stream = StreamDeserializer::new(cursor);

    let decoded1 = stream.deserialize::<Vec<bool>>().unwrap().unwrap();
    assert_eq!(decoded1, &[true, false]);

    let decoded2 = stream.deserialize::<Vec<bool>>().unwrap().unwrap();
    assert_eq!(decoded2, &[false, true]);
}

#[test]
fn map_empty() {
    let deserializer = Deserializer::from_slice(include_bytes!("reference/output/map_empty.gob"));
    let decoded = <HashMap<String, bool>>::deserialize(deserializer).unwrap();
    assert_eq!(decoded.len(), 0);
}

#[test]
fn map_non_empty() {
    let deserializer =
        Deserializer::from_slice(include_bytes!("reference/output/map_non_empty.gob"));
    let decoded = <HashMap<String, bool>>::deserialize(deserializer).unwrap();
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded["foo"], true);
    assert_eq!(decoded["bar"], false);
}

#[test]
fn complex_64() {
    let deserializer = Deserializer::from_slice(&[6, 14, 0, 254, 240, 63, 64]);
    let decoded = <(f32, f32)>::deserialize(deserializer).unwrap();
    assert_eq!(decoded.0, 1f32);
    assert_eq!(decoded.1, 2f32);
}

#[test]
fn complex_128() {
    let deserializer = Deserializer::from_slice(&[6, 14, 0, 254, 240, 63, 64]);
    let decoded = <(f64, f64)>::deserialize(deserializer).unwrap();
    assert_eq!(decoded.0, 1f64);
    assert_eq!(decoded.1, 2f64);
}

#[test]
fn point_struct() {
    #[derive(Deserialize)]
    struct Point {
        #[serde(rename = "X")]
        x: i64,
        #[serde(rename = "Y")]
        y: i64,
    }

    let deserializer =
        Deserializer::from_slice(include_bytes!("reference/output/point_struct.gob"));

    let decoded = Point::deserialize(deserializer).unwrap();
    assert_eq!(decoded.x, 22);
    assert_eq!(decoded.y, 33);
}

#[test]
fn enum_with_newtype_variants_and_external_tags() {
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    enum Enum {
        #[serde(rename = "Var1")]
        V1(bool),
        #[serde(rename = "Var2")]
        V2(i64),
        #[serde(rename = "Var3")]
        V3(String),
    }

    let deserializer = Deserializer::from_slice(include_bytes!(
        "reference/output/enum_with_newtype_variants.gob"
    ));

    let decoded = Enum::deserialize(deserializer).unwrap();
    assert_eq!(decoded, Enum::V2(42));
}

#[test]
fn enum_with_struct_variants_and_external_tags() {
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    enum Enum {
        V1 {
            #[serde(rename = "Foo")]
            foo: bool,
        },
        V2 {
            #[serde(rename = "Bar")]
            bar: i64,
            #[serde(rename = "Baz")]
            baz: u64,
        },
        V3 {
            #[serde(rename = "Quux")]
            quux: String,
        },
    }

    let deserializer = Deserializer::from_slice(include_bytes!(
        "reference/output/enum_with_struct_variants.gob"
    ));

    let decoded = Enum::deserialize(deserializer).unwrap();
    assert_eq!(decoded, Enum::V2 { bar: 42, baz: 1234 });
}
