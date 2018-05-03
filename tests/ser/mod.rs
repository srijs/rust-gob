use std::collections::BTreeMap;

use gob::StreamSerializer;
use serde_bytes::Bytes;
use serde_schema::types::{Type, TypeId};
use serde_schema::{Schema, SchemaSerialize};

mod integration;

#[test]
fn bool_true() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&true).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 1]);
}

#[test]
fn bool_false() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&false).unwrap();
    }
    assert_eq!(buffer, &[3, 2, 0, 0]);
}

#[test]
fn u8_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0u8).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u16_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0u16).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u32_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0u32).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0u64).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 0]);
}

#[test]
fn u64_small() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&42u64).unwrap();
    }
    assert_eq!(buffer, &[3, 6, 0, 42]);
}

#[test]
fn u64_big() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&1234u64).unwrap();
    }
    assert_eq!(buffer, &[5, 6, 0, 254, 4, 210]);
}

#[test]
fn u64_max() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&::std::u64::MAX).unwrap();
    }
    assert_eq!(
        buffer,
        &[11, 6, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]
    );
}

#[test]
fn i8_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0i8).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i16_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0i16).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i32_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0i32).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0i64).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 0]);
}

#[test]
fn i64_small_pos() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&42i64).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 84]);
}

#[test]
fn i64_small_neg() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&-42i64).unwrap();
    }
    assert_eq!(buffer, &[3, 4, 0, 83]);
}

#[test]
fn i64_big_pos() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&1234i64).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 164]);
}

#[test]
fn i64_big_neg() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&-1234i64).unwrap();
    }
    assert_eq!(buffer, &[5, 4, 0, 254, 9, 163]);
}

#[test]
fn i64_min() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&::std::i64::MIN).unwrap();
    }
    assert_eq!(
        buffer,
        &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 255]
    );
}

#[test]
fn i64_max() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&::std::i64::MAX).unwrap();
    }
    assert_eq!(
        buffer,
        &[11, 4, 0, 248, 255, 255, 255, 255, 255, 255, 255, 254]
    );
}

#[test]
fn f32_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0f32).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_zero() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&0f64).unwrap();
    }
    assert_eq!(buffer, &[3, 8, 0, 0]);
}

#[test]
fn f64_pos() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&42f64).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 64]);
}

#[test]
fn f64_neg() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&-42f64).unwrap();
    }
    assert_eq!(buffer, &[5, 8, 0, 254, 69, 192]);
}

#[test]
fn char_ascii() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&'f').unwrap();
    }
    assert_eq!(buffer, &[4, 4, 0, 255, 204]);
}

#[test]
fn char_unicode() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&'語').unwrap();
    }
    assert_eq!(buffer, &[6, 4, 0, 253, 1, 21, 60]);
}

#[test]
fn bytes_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Bytes::new(&[])).unwrap();
    }
    assert_eq!(buffer, &[3, 10, 0, 0]);
}

#[test]
fn bytes_non_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Bytes::new(&[1, 2, 3, 4])).unwrap();
    }
    assert_eq!(buffer, &[7, 10, 0, 4, 1, 2, 3, 4]);
}

#[test]
fn str_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&"").unwrap();
    }
    assert_eq!(buffer, &[3, 12, 0, 0]);
}

#[test]
fn str_non_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&"foo").unwrap();
    }
    assert_eq!(buffer, &[6, 12, 0, 3, 102, 111, 111]);
}

#[test]
fn vec_of_bool_to_empty_slice() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Vec::<bool>::new()).unwrap();
    }
    assert_eq!(
        buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 4, 255, 130, 0, 0,]
    );
}

#[test]
fn vec_of_bool_to_non_empty_slice() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&vec![true, false]).unwrap();
    }
    assert_eq!(
        buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 6, 255, 130, 0, 2, 1, 0,]
    );
}

#[test]
fn vec_of_bool_to_empty_array() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize::<[bool; 0]>(&[]).unwrap();
    }
    assert_eq!(
        buffer,
        &[12, 255, 129, 1, 1, 2, 255, 130, 0, 1, 2, 0, 0, 4, 255, 130, 0, 0,]
    );
}

#[test]
fn vec_of_bool_to_non_empty_array() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&[true, false]).unwrap();
    }
    assert_eq!(
        buffer,
        &[14, 255, 129, 1, 1, 2, 255, 130, 0, 1, 2, 1, 4, 0, 0, 6, 255, 130, 0, 2, 1, 0,]
    );
}

#[test]
fn vec_of_bool_to_empty_slice_twice() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Vec::<bool>::new()).unwrap();
        stream.serialize(&Vec::<bool>::new()).unwrap();
    }
    assert_eq!(
        buffer,
        &[12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 4, 255, 130, 0, 0, 4, 255, 130, 0, 0,]
    );
}

#[test]
fn vec_of_bool_from_non_empty_slice_twice() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&vec![true, false]).unwrap();
        stream.serialize(&vec![false, true]).unwrap();
    }
    assert_eq!(
        buffer,
        &[
            12, 255, 129, 2, 1, 2, 255, 130, 0, 1, 2, 0, 0, 6, 255, 130, 0, 2, 1, 0, 6, 255, 130,
            0, 2, 0, 1,
        ]
    );
}

#[test]
fn map_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&<BTreeMap<String, bool>>::new()).unwrap();
    }
    assert_eq!(
        buffer,
        &[14, 255, 129, 4, 1, 2, 255, 130, 0, 1, 12, 1, 2, 0, 0, 4, 255, 130, 0, 0,]
    );
}

#[test]
fn map_non_empty() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        let mut map = BTreeMap::new();
        map.insert("foo", true);
        map.insert("bar", false);
        stream.serialize(&map).unwrap();
    }
    assert_eq!(
        buffer,
        &[
            14, 255, 129, 4, 1, 2, 255, 130, 0, 1, 12, 1, 2, 0, 0, 14, 255, 130, 0, 2, 3, 98, 97,
            114, 0, 3, 102, 111, 111, 1,
        ]
    );
}

#[derive(Serialize)]
struct Point {
    #[serde(rename = "X")]
    x: i64,
    #[serde(rename = "Y")]
    y: i64,
}

impl SchemaSerialize for Point {
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        schema.register_type(
            Type::build()
                .struct_type("Point", 2)
                .field("X", TypeId::I64)
                .field("Y", TypeId::I64)
                .end(),
        )
    }
}

#[test]
fn point_struct() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Point { x: 22, y: 33 }).unwrap();
    }
    assert_eq!(
        buffer,
        [
            0x1f, 0xff, 0x81, 0x03, 0x01, 0x01, 0x05, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x01, 0xff,
            0x82, 0x00, 0x01, 0x02, 0x01, 0x01, 0x58, 0x01, 0x04, 0x00, 0x01, 0x01, 0x59, 0x01,
            0x04, 0x00, 0x00, 0x00, 0x07, 0xff, 0x82, 0x01, 0x2c, 0x01, 0x42, 0x00,
        ].as_ref()
    );
}

#[test]
fn point_struct_skip_x() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Point { x: 0, y: 42 }).unwrap();
    }
    assert_eq!(
        buffer,
        [
            31, 255, 129, 3, 1, 1, 5, 80, 111, 105, 110, 116, 1, 255, 130, 0, 1, 2, 1, 1, 88, 1, 4,
            0, 1, 1, 89, 1, 4, 0, 0, 0, 5, 255, 130, 2, 84, 0,
        ].as_ref()
    );
}

#[derive(Serialize)]
struct BoolStruct {
    #[serde(rename = "V")]
    v: bool,
}

impl SchemaSerialize for BoolStruct {
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        schema.register_type(
            Type::build()
                .struct_type("BoolStruct", 2)
                .field("V", TypeId::BOOL)
                .end(),
        )
    }
}

#[test]
fn bool_struct() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&BoolStruct { v: true }).unwrap();
    }
    assert_eq!(
        buffer,
        [
            0x1e, 0xff, 0x81, 0x03, 0x01, 0x01, 0x0a, 0x42, 0x6f, 0x6f, 0x6c, 0x53, 0x74, 0x72,
            0x75, 0x63, 0x74, 0x01, 0xff, 0x82, 0x00, 0x01, 0x01, 0x01, 0x01, 0x56, 0x01, 0x02,
            0x00, 0x00, 0x00, 0x05, 0xff, 0x82, 0x01, 0x01, 0x00,
        ].as_ref()
    );
}

#[test]
fn enum_with_newtype_variants_and_external_tags() {
    #[derive(Serialize)]
    enum Enum {
        #[serde(rename = "Var1")]
        #[allow(unused)]
        V1(bool),
        #[serde(rename = "Var2")]
        V2(i64),
        #[serde(rename = "Var3")]
        #[allow(unused)]
        V3(String),
    }

    impl SchemaSerialize for Enum {
        fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
            schema.register_type(
                Type::build()
                    .enum_type("Enum", 3)
                    .newtype_variant("Var1", TypeId::BOOL)
                    .newtype_variant("Var2", TypeId::I64)
                    .newtype_variant("Var3", TypeId::STR)
                    .end(),
            )
        }
    }

    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Enum::V2(42)).unwrap();
    }
    assert_eq!(
        buffer,
        [
            45, 255, 129, 3, 1, 1, 4, 69, 110, 117, 109, 1, 255, 130, 0, 1, 3, 1, 4, 86, 97, 114,
            49, 1, 2, 0, 1, 4, 86, 97, 114, 50, 1, 4, 0, 1, 4, 86, 97, 114, 51, 1, 12, 0, 0, 0, 5,
            255, 130, 2, 84, 0,
        ].as_ref()
    );
}

#[test]
fn enum_with_struct_variants_and_external_tags() {
    #[derive(Serialize)]
    enum Enum {
        #[allow(unused)]
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
        #[allow(unused)]
        V3 {
            #[serde(rename = "Quux")]
            quux: String,
        },
    }

    impl SchemaSerialize for Enum {
        fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
            schema.register_type(
                Type::build()
                    .enum_type("Enum", 3)
                    .struct_variant("V1", 1)
                    .field("Foo", TypeId::BOOL)
                    .end()
                    .struct_variant("V2", 2)
                    .field("Bar", TypeId::I64)
                    .field("Baz", TypeId::U64)
                    .end()
                    .struct_variant("V3", 1)
                    .field("Quux", TypeId::STR)
                    .end()
                    .end(),
            )
        }
    }

    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Enum::V2 { bar: 42, baz: 1234 }).unwrap();
    }
    assert_eq!(
        buffer,
        [
            42, 255, 129, 3, 1, 1, 4, 69, 110, 117, 109, 1, 255, 130, 0, 1, 3, 1, 2, 86, 49, 1,
            255, 132, 0, 1, 2, 86, 50, 1, 255, 134, 0, 1, 2, 86, 51, 1, 255, 136, 0, 0, 0, 24, 255,
            131, 3, 1, 1, 2, 86, 49, 1, 255, 132, 0, 1, 1, 1, 3, 70, 111, 111, 1, 2, 0, 0, 0, 32,
            255, 133, 3, 1, 1, 2, 86, 50, 1, 255, 134, 0, 1, 2, 1, 3, 66, 97, 114, 1, 4, 0, 1, 3,
            66, 97, 122, 1, 6, 0, 0, 0, 25, 255, 135, 3, 1, 1, 2, 86, 51, 1, 255, 136, 0, 1, 1, 1,
            4, 81, 117, 117, 120, 1, 12, 0, 0, 0, 11, 255, 130, 2, 1, 84, 1, 254, 4, 210, 0, 0,
        ].as_ref()
    );
}
