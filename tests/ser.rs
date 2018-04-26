extern crate gob;
extern crate serde;
extern crate serde_bytes;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use gob::Serializer;
use serde::Serialize;
use serde_bytes::{Bytes, ByteBuf};

#[test]
fn bool_true() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        true.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 1]);
}

#[test]
fn bool_false() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        false.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 0]);
}

#[test]
fn u8_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0u8.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u16_zero() {
        let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0u16.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u32_zero() {
        let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0u32.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_zero() {
        let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0u64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_small() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        42u8.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 42]);
}

#[test]
fn u64_big() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        1234u64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 6, 0, 254, 4, 210]);
}

#[test]
fn u64_max() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        ::std::u64::MAX.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[11, 6, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn i8_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0i8.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i16_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0i16.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i32_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0i32.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0i64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_small_pos() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        42i64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 84]);
}

#[test]
fn i64_small_neg() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        (-42i64).serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 83]);
}

#[test]
fn i64_big_pos() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        1234i64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 164]);
}

#[test]
fn i64_big_neg() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        (-1234i64).serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 163]);
}

#[test]
fn i64_min() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        ::std::i64::MIN.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn i64_max() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        ::std::i64::MAX.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 254]);
}

#[test]
fn f32_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0f32.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_zero() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        0f64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_pos() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        42f64.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 64]);
}

#[test]
fn f64_neg() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        (-42f64).serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 192]);
}

#[test]
fn char_ascii() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        'f'.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[4, 4, 0, 255, 204]);
}

#[test]
fn char_unicode() {
    let mut buffer = Vec::new();
    {
        let serializer = Serializer::new(&mut buffer);
        '語'.serialize(serializer).unwrap();
    }
    assert_eq!(buffer, &[6, 4, 0, 253, 1, 21, 60]);
}
