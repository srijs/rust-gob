use serde_schema::types::{Type, TypeId};
use serde_schema::{Schema, SchemaSerialize};

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! test_builtin {
    ($name:ident, $go_typ:expr, $go_value:expr, $enc_typ:expr, $typ:ident, $value:expr) => {
        ser_test!(
            $name
            go_decls format!("
                type Value struct {{
                    V {}
                }}
            ", $go_typ).trim(),
            go_value Value format!("
                Value {{
                    V: {},
                }}
            ", $go_value).trim(),
            rust_decls {
                #[derive(Serialize, Debug)]
                struct Value {
                    V: $typ,
                }
                impl Default for Value {
                    fn default() -> Value { Value { V: $value } }
                }
                impl SchemaSerialize for Value {
                    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
                        schema.register_type(
                            Type::build()
                                .struct_type("Value", 2)
                                .field("V", $enc_typ)
                                .end(),
                        )
                    }
                }
            },
            validate v: Value {}
        );
    };
}

test_builtin!(test_u8, "uint8", "7", TypeId::U8, u8, 7);
test_builtin!(test_bool, "bool", "true", TypeId::BOOL, bool, true);
