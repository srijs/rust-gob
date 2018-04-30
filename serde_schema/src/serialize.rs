use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::borrow::Cow;
use std::hash::{Hash, BuildHasher};

use serde::ser::Serialize;

#[cfg(feature = "bytes")]
use serde_bytes::{ByteBuf, Bytes};

use ::{Schema, SchemaSerializer, StructField, TypeId, Type};

pub trait SchemaSerialize: Serialize {
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error>;

    /// This can be overridden when a schema should be used that is dependent on the value,
    /// and will therefore only be known at runtime.
    fn schema_serialize<S: SchemaSerializer>(&self, mut s: S) -> Result<S::Ok, S::Error> {
        let id = Self::schema_register(s.schema_mut())?;
        self.serialize(s.serializer(id)?)
    }
}

// # Implementatiions

// ## Primitive Types

macro_rules! primitive_impl {
    ($t: ty, $id: tt) => {
        impl SchemaSerialize for $t {
            #[inline]
            fn schema_register<S: Schema>(_: &mut S) -> Result<S::TypeId, S::Error> {
                Ok(TypeId::$id)
            }
        }
    }
}

primitive_impl!(bool, BOOL);
primitive_impl!(i8, I8);
primitive_impl!(i16, I16);
primitive_impl!(i32, I32);
primitive_impl!(i64, I64);
primitive_impl!(isize, I64);
primitive_impl!(u8, U8);
primitive_impl!(u16, U16);
primitive_impl!(u32, U32);
primitive_impl!(u64, U64);
primitive_impl!(usize, U64);
primitive_impl!(f32, F32);
primitive_impl!(f64, F64);
primitive_impl!(char, CHAR);

// ## Strings

impl<'a> SchemaSerialize for &'a str {
    #[inline]
    fn schema_register<S: Schema>(_: &mut S) -> Result<S::TypeId, S::Error> {
        Ok(TypeId::STR)
    }
}

impl SchemaSerialize for String {
    #[inline]
    fn schema_register<S: Schema>(_: &mut S) -> Result<S::TypeId, S::Error> {
        Ok(TypeId::STR)
    }
}


// ## Bytes

#[cfg(feature = "bytes")]
impl<'a> SchemaSerialize for Bytes<'a> {
    #[inline]
    fn schema_register<S: Schema>(_: &mut S) -> Result<S::TypeId, S::Error> {
        Ok(TypeId::BYTES)
    }
}

#[cfg(feature = "bytes")]
impl SchemaSerialize for ByteBuf {
    #[inline]
    fn schema_register<S: Schema>(_: &mut S) -> Result<S::TypeId, S::Error> {
        Ok(TypeId::BYTES)
    }
}

// ## Option

impl<T: SchemaSerialize> SchemaSerialize for Option<T> {
    #[inline]
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        let id = T::schema_register(schema)?;
        schema.register_type(Type::Option { value: id })
    }
}

// ## PhantomData

impl<T> SchemaSerialize for ::std::marker::PhantomData<T> {
    #[inline]
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        schema.register_type(Type::UnitStruct { name: Cow::Borrowed("PhantomData") })
    }
}

// ## Arrays

macro_rules! array_impls {
    {$($len:tt)+} => {
        $(
            impl<T: SchemaSerialize> SchemaSerialize for [T; $len] {
                #[inline]
                fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
                    let id = T::schema_register(schema)?;
                    schema.register_type(Type::Seq { len: Some($len), element: id })
                }
            }
        )+
    }
}

array_impls! {
    00 01 02 03 04 05 06 07 08 09
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

// ## Slices

impl<T: SchemaSerialize> SchemaSerialize for [T] {
    #[inline]
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        let id = T::schema_register(schema)?;
        schema.register_type(Type::Seq { len: None, element: id })
    }
}

// ## Sequence Collections

macro_rules! seq_impl {
    ($ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound:ident)* >) => {
        impl<T $(, $typaram)*> SchemaSerialize for $ty<T $(, $typaram)*>
        where
            T: SchemaSerialize $(+ $tbound1 $(+ $tbound2)*)*,
            $($typaram: $bound,)*
        {
            #[inline]
            fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
                let id = T::schema_register(schema)?;
                schema.register_type(Type::Seq { len: None, element: id })
            }
        }
    }
}

seq_impl!(BinaryHeap<T: Ord>);
seq_impl!(BTreeSet<T: Ord>);
seq_impl!(HashSet<T: Eq + Hash, H: BuildHasher>);
seq_impl!(LinkedList<T>);
seq_impl!(Vec<T>);
seq_impl!(VecDeque<T>);

// ## Range

impl<Idx: SchemaSerialize> SchemaSerialize for ::std::ops::Range<Idx> {
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        let id = Idx::schema_register(schema)?;
        schema.register_type(Type::Struct {
            name: Cow::Borrowed("Range"),
            fields: Cow::Owned(vec![
                StructField { name: Cow::Borrowed("start"), id: id.clone() },
                StructField { name: Cow::Borrowed("end"), id }
            ])
        })
    }
}

// ## Unit

impl SchemaSerialize for () {
    #[inline]
    fn schema_register<S: Schema>(_: &mut S) -> Result<S::TypeId, S::Error> {
        Ok(TypeId::UNIT)
    }
}

// ## Tuples

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> SchemaSerialize for ($($name,)+)
            where
                $($name: SchemaSerialize,)+
            {
                #[inline]
                fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
                    let elements = vec![
                        $(
                            $name::schema_register(schema)?,
                        )+
                    ];
                    schema.register_type(Type::Tuple { elements: Cow::Owned(elements) })
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

// ## Map Collections

macro_rules! map_impl {
    ($ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound:ident)* >) => {
        impl<K, V $(, $typaram)*> SchemaSerialize for $ty<K, V $(, $typaram)*>
        where
            K: SchemaSerialize $(+ $kbound1 $(+ $kbound2)*)*,
            V: SchemaSerialize,
            $($typaram: $bound,)*
        {
            #[inline]
            fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
                let k = K::schema_register(schema)?;
                let v = V::schema_register(schema)?;
                schema.register_type(Type::Map { key: k, value: v })
            }
        }
    }
}

map_impl!(BTreeMap<K: Ord, V>);
map_impl!(HashMap<K: Eq + Hash, V, H: BuildHasher>);

// ## References

impl<'a, T: SchemaSerialize> SchemaSerialize for &'a T {
    #[inline]
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        T::schema_register(schema)
    }
}

impl<'a, T: SchemaSerialize> SchemaSerialize for &'a mut T {
    #[inline]
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        T::schema_register(schema)
    }
}

impl<T: SchemaSerialize> SchemaSerialize for Box<T> {
    #[inline]
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        T::schema_register(schema)
    }
}

impl<'a, T: SchemaSerialize + ToOwned> SchemaSerialize for Cow<'a, T> {
    #[inline]
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        T::schema_register(schema)
    }
}
