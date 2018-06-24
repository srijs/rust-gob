use std::borrow::Borrow;

use serde::{self, Serialize, Serializer};
use serde_schema::types::Type;

use ser::{Schema, TypeId};

pub struct SerializeEmptyValue<S> {
    schema: S,
    type_id: TypeId,
}

impl<S: Borrow<Schema>> SerializeEmptyValue<S> {
    pub fn new(schema: S, type_id: TypeId) -> Self {
        SerializeEmptyValue { schema, type_id }
    }

    fn serialize_with_type<Z: Serializer>(
        &self,
        ty: &Type<TypeId>,
        ser: Z,
    ) -> Result<Z::Ok, Z::Error> {
        match ty {
            &Type::Option(ref option_type) => {
                SerializeEmptyValue::new(self.schema.borrow(), *option_type.inner_type())
                    .serialize(ser)
            }
            &Type::NewtypeStruct(ref newtype_struct_type) => {
                let value = SerializeEmptyValue::new(
                    self.schema.borrow(),
                    *newtype_struct_type.inner_type(),
                );
                ser.serialize_newtype_struct("", &value)
            }
            &Type::Seq(_) => {
                let ser_seq = ser.serialize_seq(Some(0))?;
                serde::ser::SerializeSeq::end(ser_seq)
            }
            &Type::Tuple(ref tuple_type) => {
                let mut ser_tup = ser.serialize_tuple(tuple_type.element_types().len())?;
                for element_type in tuple_type.element_types() {
                    let value = SerializeEmptyValue::new(self.schema.borrow(), *element_type);
                    serde::ser::SerializeTuple::serialize_element(&mut ser_tup, &value)?;
                }
                serde::ser::SerializeTuple::end(ser_tup)
            }
            &Type::TupleStruct(ref tuple_struct_type) => {
                let mut ser_tup =
                    ser.serialize_tuple_struct("", tuple_struct_type.element_types().len())?;
                for element_type in tuple_struct_type.element_types() {
                    let value = SerializeEmptyValue::new(self.schema.borrow(), *element_type);
                    serde::ser::SerializeTupleStruct::serialize_field(&mut ser_tup, &value)?;
                }
                serde::ser::SerializeTupleStruct::end(ser_tup)
            }
            &Type::Map(_) => {
                let ser_map = ser.serialize_map(Some(0))?;
                serde::ser::SerializeMap::end(ser_map)
            }
            &Type::Struct(ref struct_type) => {
                let mut ser_struct = ser.serialize_struct("", struct_type.fields().len())?;
                for _field in struct_type.fields() {
                    serde::ser::SerializeStruct::skip_field(&mut ser_struct, "")?;
                }
                serde::ser::SerializeStruct::end(ser_struct)
            }
            _ => Err(serde::ser::Error::custom(format!(
                "empty representation not available for type with id {}",
                self.type_id.0
            ))),
        }
    }
}

impl<S: Borrow<Schema>> Serialize for SerializeEmptyValue<S> {
    fn serialize<Z: Serializer>(&self, ser: Z) -> Result<Z::Ok, Z::Error> {
        match self.type_id {
            TypeId::BOOL => ser.serialize_bool(false),
            TypeId::INT => ser.serialize_i8(0),
            TypeId::UINT => ser.serialize_u8(0),
            TypeId::FLOAT => ser.serialize_f32(0.0),
            TypeId::BYTES => ser.serialize_bytes(&[]),
            TypeId::STRING => ser.serialize_str(""),
            _ => {
                if let Some(ty) = self.schema.borrow().lookup(self.type_id) {
                    self.serialize_with_type(&*ty, ser)
                } else {
                    Err(serde::ser::Error::custom(format!(
                        "empty representation not available for type with id {}",
                        self.type_id.0
                    )))
                }
            }
        }
    }
}
