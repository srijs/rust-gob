use serde::ser::Error;

use types::{Type, TypeId};

pub trait Schema {
    type TypeId: TypeId;
    type Error: Error;

    fn register_type(&mut self, ty: Type<Self::TypeId>) -> Result<Self::TypeId, Self::Error>;
}

impl<'a, T: Schema> Schema for &'a mut T {
    type TypeId = T::TypeId;
    type Error = T::Error;

    fn register_type(&mut self, ty: Type<Self::TypeId>) -> Result<Self::TypeId, Self::Error> {
        T::register_type(*self, ty)
    }
}

impl<T: Schema> Schema for Box<T> {
    type TypeId = T::TypeId;
    type Error = T::Error;

    fn register_type(&mut self, ty: Type<Self::TypeId>) -> Result<Self::TypeId, Self::Error> {
        T::register_type(self, ty)
    }
}
