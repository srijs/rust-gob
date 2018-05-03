use std::borrow::Cow;
use std::marker::PhantomData;

use types::*;

pub struct StructBuilder<T: TypeId> {
    name: &'static str,
    fields: Vec<StructField<T>>,
}

impl<T: TypeId> StructBuilder<T> {
    #[inline]
    pub(crate) fn new(name: &'static str, len: usize) -> Self {
        StructBuilder {
            name,
            fields: Vec::with_capacity(len),
        }
    }

    #[inline]
    pub fn field(mut self, name: &'static str, id: T) -> Self {
        self.fields.push(StructField {
            name: Cow::Borrowed(name),
            id,
        });
        self
    }

    #[inline]
    pub fn end(self) -> Type<T> {
        Type::Struct(StructType {
            name: Cow::Borrowed(self.name),
            fields: Cow::Owned(self.fields),
        })
    }
}

pub struct TupleBuilder<T: TypeId> {
    elements: Vec<T>,
}

impl<T: TypeId> TupleBuilder<T> {
    #[inline]
    pub(crate) fn new(len: usize) -> Self {
        TupleBuilder {
            elements: Vec::with_capacity(len),
        }
    }

    #[inline]
    pub fn element(mut self, id: T) -> Self {
        self.elements.push(id);
        self
    }

    #[inline]
    pub fn end(self) -> Type<T> {
        Type::Tuple(TupleType {
            elements: Cow::Owned(self.elements),
        })
    }
}

pub struct TupleStructBuilder<T: TypeId> {
    name: &'static str,
    elements: Vec<T>,
}

impl<T: TypeId> TupleStructBuilder<T> {
    #[inline]
    pub(crate) fn new(name: &'static str, len: usize) -> Self {
        TupleStructBuilder {
            name,
            elements: Vec::with_capacity(len),
        }
    }

    #[inline]
    pub fn element(mut self, id: T) -> Self {
        self.elements.push(id);
        self
    }

    #[inline]
    pub fn end(self) -> Type<T> {
        Type::TupleStruct(TupleStructType {
            name: Cow::Borrowed(self.name),
            elements: Cow::Owned(self.elements),
        })
    }
}

pub struct StructVariantBuilder<T: TypeId> {
    name: &'static str,
    fields: Vec<StructField<T>>,
    enum_builder: EnumBuilder<T>,
}

impl<T: TypeId> StructVariantBuilder<T> {
    #[inline]
    pub fn field(mut self, name: &'static str, id: T) -> Self {
        self.fields.push(StructField {
            name: Cow::Borrowed(name),
            id,
        });
        self
    }

    #[inline]
    pub fn end(mut self) -> EnumBuilder<T> {
        self.enum_builder
            .variants
            .push(EnumVariant::Struct(StructVariant {
                name: Cow::Borrowed(self.name),
                fields: Cow::Owned(self.fields),
            }));
        self.enum_builder
    }
}

pub struct TupleVariantBuilder<T: TypeId> {
    name: &'static str,
    elements: Vec<T>,
    enum_builder: EnumBuilder<T>,
}

impl<T: TypeId> TupleVariantBuilder<T> {
    #[inline]
    pub fn element(mut self, id: T) -> Self {
        self.elements.push(id);
        self
    }

    #[inline]
    pub fn end(mut self) -> EnumBuilder<T> {
        self.enum_builder
            .variants
            .push(EnumVariant::Tuple(TupleVariant {
                name: Cow::Borrowed(self.name),
                elements: Cow::Owned(self.elements),
            }));
        self.enum_builder
    }
}

pub struct EnumBuilder<T: TypeId> {
    name: &'static str,
    variants: Vec<EnumVariant<T>>,
}

impl<T: TypeId> EnumBuilder<T> {
    #[inline]
    pub(crate) fn new(name: &'static str, len: usize) -> Self {
        EnumBuilder {
            name,
            variants: Vec::with_capacity(len),
        }
    }

    #[inline]
    pub fn unit_variant(mut self, name: &'static str) -> Self {
        self.variants.push(EnumVariant::Unit(UnitVariant {
            name: Cow::Borrowed(name),
            _phan: PhantomData,
        }));
        self
    }

    #[inline]
    pub fn newtype_variant(mut self, name: &'static str, id: T) -> Self {
        self.variants.push(EnumVariant::Newtype(NewtypeVariant {
            name: Cow::Borrowed(name),
            value: id,
        }));
        self
    }

    #[inline]
    pub fn tuple_variant(self, name: &'static str, len: usize) -> TupleVariantBuilder<T> {
        TupleVariantBuilder {
            name,
            elements: Vec::with_capacity(len),
            enum_builder: self,
        }
    }

    #[inline]
    pub fn struct_variant(self, name: &'static str, len: usize) -> StructVariantBuilder<T> {
        StructVariantBuilder {
            name,
            fields: Vec::with_capacity(len),
            enum_builder: self,
        }
    }

    #[inline]
    pub fn end(self) -> Type<T> {
        Type::Enum(EnumType {
            name: Cow::Borrowed(self.name),
            variants: Cow::Owned(self.variants),
        })
    }
}

pub struct TypeBuilder<T: TypeId> {
    _phan: PhantomData<T>,
}

impl<T: TypeId> TypeBuilder<T> {
    #[inline]
    pub(crate) fn new() -> TypeBuilder<T> {
        TypeBuilder { _phan: PhantomData }
    }

    #[inline]
    pub fn option_type(self, value: T) -> Type<T> {
        Type::Option(OptionType { value })
    }

    #[inline]
    pub fn unit_struct_type(self, name: &'static str) -> Type<T> {
        Type::UnitStruct(UnitStructType {
            _phan: PhantomData,
            name: Cow::Borrowed(name),
        })
    }

    #[inline]
    pub fn newtype_struct_type(self, name: &'static str, value: T) -> Type<T> {
        Type::NewtypeStruct(NewtypeStructType {
            name: Cow::Borrowed(name),
            value,
        })
    }

    #[inline]
    pub fn seq_type(self, len: Option<usize>, element: T) -> Type<T> {
        Type::Seq(SeqType { len, element })
    }

    #[inline]
    pub fn tuple_type(self, len: usize) -> TupleBuilder<T> {
        TupleBuilder::new(len)
    }

    #[inline]
    pub fn tuple_struct_type(self, name: &'static str, len: usize) -> TupleStructBuilder<T> {
        TupleStructBuilder::new(name, len)
    }

    #[inline]
    pub fn map_type(self, key: T, value: T) -> Type<T> {
        Type::Map(MapType { key, value })
    }

    #[inline]
    pub fn struct_type(self, name: &'static str, len: usize) -> StructBuilder<T> {
        StructBuilder::new(name, len)
    }

    #[inline]
    pub fn enum_type(self, name: &'static str, len: usize) -> EnumBuilder<T> {
        EnumBuilder::new(name, len)
    }
}
