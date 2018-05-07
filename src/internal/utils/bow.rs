use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

pub enum Bow<'a, T>
where
    T: 'a,
{
    Borrowed(&'a mut T),
    Owned(T),
}

impl<'a, T> Deref for Bow<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            &Bow::Borrowed(ref t) => t,
            &Bow::Owned(ref t) => t,
        }
    }
}

impl<'a, T> Borrow<T> for Bow<'a, T> {
    fn borrow(&self) -> &T {
        &*self
    }
}

impl<'a, T> DerefMut for Bow<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            &mut Bow::Borrowed(ref mut t) => t,
            &mut Bow::Owned(ref mut t) => t,
        }
    }
}

impl<'a, T> BorrowMut<T> for Bow<'a, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut *self
    }
}
