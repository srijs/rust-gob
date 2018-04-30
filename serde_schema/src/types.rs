use std::borrow::Cow;

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
    pub name: Cow<'static, str>,
    pub id: T
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumVariant<T: TypeId> {
    Unit { name: Cow<'static, str> },
    Newtype { name: Cow<'static, str>, value: T },
    Tuple { name: Cow<'static, str>, elements: Cow<'static, [T]> },
    Struct { name: Cow<'static, str>, fields: Cow<'static, [StructField<T>]> }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type<T: TypeId> {
    Option { value: T },
    UnitStruct { name: Cow<'static, str> },
    NewtypeStruct { name: Cow<'static, str>, value: T },
    Seq { len: Option<usize>, element: T },
    Tuple { elements: Cow<'static, [T]> },
    TupleStruct { name: Cow<'static, str>, elements: Cow<'static, [T]> },
    Map { key: T, value: T },
    Struct { name: Cow<'static, str>, fields: Cow<'static, [StructField<T>]> },
    Enum { name: Cow<'static, str>, variants: Cow<'static, [EnumVariant<T>]> }
}
