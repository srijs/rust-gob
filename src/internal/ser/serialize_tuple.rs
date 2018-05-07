use std::borrow::Borrow;

use serde::de::value::Error;
use serde::ser::{self, Serialize};

use internal::types::TypeId;
use schema::Schema;

use super::{SerializationCtx, SerializationOk, SerializeSeqValue};

pub(crate) enum SerializeTupleValue<S> {
    Homogeneous(SerializeSeqValue<S>),
}

impl<S: Borrow<Schema>> SerializeTupleValue<S> {
    pub(crate) fn homogeneous(ctx: SerializationCtx<S>, type_id: TypeId) -> Result<Self, Error> {
        let inner = SerializeSeqValue::new(ctx, None, type_id)?;
        Ok(SerializeTupleValue::Homogeneous(inner))
    }

    pub(crate) fn type_id(&self) -> TypeId {
        match self {
            &SerializeTupleValue::Homogeneous(ref inner) => inner.type_id(),
        }
    }
}

impl<S: Borrow<Schema>> ser::SerializeTuple for SerializeTupleValue<S> {
    type Ok = SerializationOk<S>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        match self {
            &mut SerializeTupleValue::Homogeneous(ref mut inner) => {
                ser::SerializeSeq::serialize_element(inner, value)
            }
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            SerializeTupleValue::Homogeneous(inner) => ser::SerializeSeq::end(inner),
        }
    }
}
