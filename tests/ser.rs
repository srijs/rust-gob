extern crate gob;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_schema;
#[macro_use]
extern crate serde_schema_derive;

use std::collections::BTreeMap;

use gob::StreamSerializer;
use serde_bytes::Bytes;
use serde_schema::types::{Type, TypeId};
use serde_schema::{Schema, SchemaSerialize};

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
        include_bytes!("reference/output/slice_of_bool_empty.gob")
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
        include_bytes!("reference/output/slice_of_bool_non_empty.gob")
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
        include_bytes!("reference/output/array_of_bool_empty.gob")
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
        include_bytes!("reference/output/array_of_bool_non_empty.gob")
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
        include_bytes!("reference/output/slice_of_bool_empty_twice.gob")
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
        include_bytes!("reference/output/slice_of_bool_non_empty_twice.gob")
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
        include_bytes!("reference/output/map_empty.gob").as_ref()
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
        include_bytes!("reference/output/map_non_empty.gob").as_ref()
    );
}

#[derive(Serialize, SchemaSerialize)]
struct Point {
    #[serde(rename = "X")]
    x: i64,
    #[serde(rename = "Y")]
    y: i64,
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
        include_bytes!("reference/output/point_struct.gob").as_ref()
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
        include_bytes!("reference/output/point_struct_skip_x.gob").as_ref()
    );
}

#[derive(Serialize, SchemaSerialize)]
struct BoolStruct {
    #[serde(rename = "V")]
    v: bool,
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
        include_bytes!("reference/output/bool_struct.gob").as_ref()
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
        include_bytes!("reference/output/enum_with_newtype_variants.gob").as_ref()
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
        include_bytes!("reference/output/enum_with_struct_variants.gob").as_ref()
    );
}

#[test]
fn option_none_to_empty_values() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize::<Option<bool>>(&None).unwrap();
        stream.serialize::<Option<u64>>(&None).unwrap();
        stream.serialize::<Option<i64>>(&None).unwrap();
        stream.serialize::<Option<f64>>(&None).unwrap();
        stream.serialize::<Option<String>>(&None).unwrap();
        stream.serialize::<Option<Bytes>>(&None).unwrap();
        stream.serialize::<Option<Vec<bool>>>(&None).unwrap();
    }
    assert_eq!(
        buffer,
        include_bytes!("reference/output/empty_values.gob").as_ref()
    );
}

#[test]
fn option_some_to_non_empty_values() {
    let mut buffer = Vec::new();
    {
        let mut stream = StreamSerializer::new(&mut buffer);
        stream.serialize(&Some(true)).unwrap();
        stream.serialize(&Some(42u64)).unwrap();
        stream.serialize(&Some(42i64)).unwrap();
        stream.serialize(&Some(42f64)).unwrap();
        stream.serialize(&Some("foo")).unwrap();
        stream.serialize(&Some(Bytes::new(&[0x1, 0x2]))).unwrap();
        stream.serialize(&Some(vec![true, false])).unwrap();
    }
    assert_eq!(
        buffer,
        include_bytes!("reference/output/non_empty_values.gob").as_ref()
    );
}
