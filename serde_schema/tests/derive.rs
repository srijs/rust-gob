extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_schema;
#[macro_use]
extern crate serde_schema_derive;

use serde::de::value::Error;
use serde_schema::types::{Type, TypeId};
use serde_schema::{Schema, SchemaSerialize};

#[derive(Clone, Debug, PartialEq, Eq)]
enum MockTypeId {
    Boolean,
    String,
    Uint64,
    Int64,
    Unknown,
    Custom(usize),
}

impl TypeId for MockTypeId {
    const UNIT: MockTypeId = MockTypeId::Unknown;
    const BOOL: MockTypeId = MockTypeId::Boolean;
    const I8: MockTypeId = MockTypeId::Unknown;
    const I16: MockTypeId = MockTypeId::Unknown;
    const I32: MockTypeId = MockTypeId::Unknown;
    const I64: MockTypeId = MockTypeId::Int64;
    const U8: MockTypeId = MockTypeId::Unknown;
    const U16: MockTypeId = MockTypeId::Unknown;
    const U32: MockTypeId = MockTypeId::Unknown;
    const U64: MockTypeId = MockTypeId::Uint64;
    const F32: MockTypeId = MockTypeId::Unknown;
    const F64: MockTypeId = MockTypeId::Unknown;
    const CHAR: MockTypeId = MockTypeId::Unknown;
    const STR: MockTypeId = MockTypeId::String;
    const BYTES: MockTypeId = MockTypeId::Unknown;
}

struct MockSchema(Vec<Type<MockTypeId>>);

impl Schema for MockSchema {
    type TypeId = MockTypeId;
    type Error = Error;

    fn register_type(&mut self, ty: Type<MockTypeId>) -> Result<MockTypeId, Error> {
        self.0.push(ty);
        Ok(MockTypeId::Custom(self.0.len() - 1))
    }
}

#[test]
fn unit_struct() {
    #[derive(Serialize, SchemaSerialize)]
    struct Unit;

    let mut schema = MockSchema(Vec::new());
    let type_id = Unit::schema_register(&mut schema).unwrap();

    assert_eq!(type_id, MockTypeId::Custom(0));
    assert_eq!(schema.0.len(), 1);
    assert_eq!(schema.0[0], Type::build().unit_struct_type("Unit"));
}

#[test]
fn struct_point_with_field_rename() {
    #[derive(Serialize, SchemaSerialize)]
    struct Point {
        #[serde(rename = "X")]
        x: i64,
        #[serde(rename = "Y")]
        y: i64,
    }

    let mut schema = MockSchema(Vec::new());
    let type_id = Point::schema_register(&mut schema).unwrap();

    assert_eq!(type_id, MockTypeId::Custom(0));
    assert_eq!(schema.0.len(), 1);
    assert_eq!(
        schema.0[0],
        Type::build()
            .struct_type("Point", 2)
            .field("X", MockTypeId::Int64)
            .field("Y", MockTypeId::Int64)
            .end()
    );
}

#[test]
fn enum_with_renamed_newtype_variants() {
    #[derive(Serialize, SchemaSerialize)]
    enum Enum {
        #[serde(rename = "Var1")]
        #[allow(unused)]
        V1(bool),
        #[serde(rename = "Var2")]
        #[allow(unused)]
        V2(i64),
        #[serde(rename = "Var3")]
        #[allow(unused)]
        V3(String),
    }

    let mut schema = MockSchema(Vec::new());
    let type_id = Enum::schema_register(&mut schema).unwrap();

    assert_eq!(type_id, MockTypeId::Custom(0));
    assert_eq!(schema.0.len(), 1);
    assert_eq!(
        schema.0[0],
        Type::build()
            .enum_type("Enum", 3)
            .newtype_variant("Var1", MockTypeId::Boolean)
            .newtype_variant("Var2", MockTypeId::Int64)
            .newtype_variant("Var3", MockTypeId::String)
            .end()
    );
}

#[test]
fn enum_with_struct_variants_and_renamed_fields() {
    #[derive(Serialize, SchemaSerialize)]
    enum Enum {
        #[allow(unused)]
        V1 {
            #[serde(rename = "Foo")]
            foo: bool,
        },
        #[allow(unused)]
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

    let mut schema = MockSchema(Vec::new());
    let type_id = Enum::schema_register(&mut schema).unwrap();

    assert_eq!(type_id, MockTypeId::Custom(0));
    assert_eq!(schema.0.len(), 1);
    assert_eq!(
        schema.0[0],
        Type::build()
            .enum_type("Enum", 2)
            .struct_variant("V1", 1)
            .field("Foo", MockTypeId::Boolean)
            .end()
            .struct_variant("V2", 2)
            .field("Bar", MockTypeId::Int64)
            .field("Baz", MockTypeId::Uint64)
            .end()
            .struct_variant("V3", 1)
            .field("Quux", MockTypeId::String)
            .end()
            .end()
    );
}
