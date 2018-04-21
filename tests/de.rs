extern crate gob;
extern crate serde;
extern crate serde_bytes;

use gob::de::Deserializer;
use serde::Deserialize;
use serde_bytes::ByteBuf;

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