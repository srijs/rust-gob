use std::borrow::Cow;
use std::marker::PhantomData;

pub mod builders;
use self::builders::TypeBuilder;

pub trait TypeId: Clone + 'static {
    const UNIT: Self;
    const BOOL: Self;
    const I8: Self;
    const I16: Self;
    const I32: Self;
    const I64: Self;
    const U8: Self;
    const U16: Self;
    const U32: Self;
    const U64: Self;
    const F32: Self;
    const F64: Self;
    const CHAR: Self;
    const STR: Self;
    const BYTES: Self;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructField<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) id: T,
}

impl<T: TypeId> StructField<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn field_type(&self) -> &T {
        &self.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitVariant<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) _phan: PhantomData<T>,
}

impl<T: TypeId> UnitVariant<T> {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewtypeVariant<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) value: T,
}

impl<T: TypeId> NewtypeVariant<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inner_type(&self) -> &T {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleVariant<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) elements: Cow<'static, [T]>,
}

impl<T: TypeId> TupleVariant<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn element_types(&self) -> &[T] {
        &self.elements
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructVariant<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) fields: Cow<'static, [StructField<T>]>,
}

impl<T: TypeId> StructVariant<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[StructField<T>] {
        &self.fields
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumVariant<T: TypeId> {
    Unit(UnitVariant<T>),
    Newtype(NewtypeVariant<T>),
    Tuple(TupleVariant<T>),
    Struct(StructVariant<T>),
}

impl<T: TypeId> EnumVariant<T> {
    pub fn name(&self) -> &str {
        match self {
            &EnumVariant::Unit(ref v) => v.name(),
            &EnumVariant::Newtype(ref v) => v.name(),
            &EnumVariant::Tuple(ref v) => v.name(),
            &EnumVariant::Struct(ref v) => v.name(),
        }
    }

    pub fn as_unit_variant(&self) -> Option<&UnitVariant<T>> {
        if let &EnumVariant::Unit(ref unit_variant) = self {
            Some(&unit_variant)
        } else {
            None
        }
    }

    pub fn as_newtype_variant(&self) -> Option<&NewtypeVariant<T>> {
        if let &EnumVariant::Newtype(ref newtype_variant) = self {
            Some(&newtype_variant)
        } else {
            None
        }
    }

    pub fn as_tuple_variant(&self) -> Option<&TupleVariant<T>> {
        if let &EnumVariant::Tuple(ref tuple_variant) = self {
            Some(&tuple_variant)
        } else {
            None
        }
    }

    pub fn as_struct_variant(&self) -> Option<&StructVariant<T>> {
        if let &EnumVariant::Struct(ref struct_variant) = self {
            Some(&struct_variant)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionType<T: TypeId> {
    pub(crate) value: T,
}

impl<T: TypeId> OptionType<T> {
    pub fn inner_type(&self) -> &T {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitStructType<T: TypeId> {
    pub(crate) _phan: PhantomData<T>,
    pub(crate) name: Cow<'static, str>,
}

impl<T: TypeId> UnitStructType<T> {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewtypeStructType<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) value: T,
}

impl<T: TypeId> NewtypeStructType<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inner_type(&self) -> &T {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeqType<T: TypeId> {
    pub(crate) len: Option<usize>,
    pub(crate) element: T,
}

impl<T: TypeId> SeqType<T> {
    pub fn len(&self) -> Option<usize> {
        self.len
    }

    pub fn element_type(&self) -> &T {
        &self.element
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleType<T: TypeId> {
    pub(crate) elements: Cow<'static, [T]>,
}

impl<T: TypeId> TupleType<T> {
    pub fn element_types(&self) -> &[T] {
        &self.elements
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleStructType<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) elements: Cow<'static, [T]>,
}

impl<T: TypeId> TupleStructType<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn element_types(&self) -> &[T] {
        &self.elements
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapType<T: TypeId> {
    pub(crate) key: T,
    pub(crate) value: T,
}

impl<T: TypeId> MapType<T> {
    pub fn key_type(&self) -> &T {
        &self.key
    }

    pub fn value_type(&self) -> &T {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructType<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) fields: Cow<'static, [StructField<T>]>,
}

impl<T: TypeId> StructType<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[StructField<T>] {
        &self.fields
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumType<T: TypeId> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) variants: Cow<'static, [EnumVariant<T>]>,
}

impl<T: TypeId> EnumType<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn variants(&self) -> &[EnumVariant<T>] {
        &self.variants
    }

    pub fn variant(&self, idx: u32) -> Option<&EnumVariant<T>> {
        self.variants.get(idx as usize)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type<T: TypeId> {
    Option(OptionType<T>),
    UnitStruct(UnitStructType<T>),
    NewtypeStruct(NewtypeStructType<T>),
    Seq(SeqType<T>),
    Tuple(TupleType<T>),
    TupleStruct(TupleStructType<T>),
    Map(MapType<T>),
    Struct(StructType<T>),
    Enum(EnumType<T>),
}

impl<T: TypeId> Type<T> {
    #[inline]
    pub fn build() -> TypeBuilder<T> {
        TypeBuilder::new()
    }
}
