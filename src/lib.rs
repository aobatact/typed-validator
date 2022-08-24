use std::{cmp::Ordering, fmt::Debug, marker::PhantomData, ops::Deref};

pub mod validators;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wrapper<T, V>(T, PhantomData<V>);

pub trait Validator<T> {
    type Error: Debug;

    fn validate(t: &T) -> Result<(), Self::Error>;
}

impl<T, V, Error> Wrapper<T, V>
where
    V: Validator<T, Error = Error>,
{
    pub fn new(t: T) -> Result<Self, Error> {
        V::validate(&t).map(|_| Self(t, PhantomData))
    }
}

impl<T: AsRef<U>, U, V> AsRef<U> for Wrapper<T, V> {
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}

impl<T, V> Deref for Wrapper<T, V> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PartialEq, V> PartialEq<T> for Wrapper<T, V> {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T: PartialOrd, V> PartialOrd<T> for Wrapper<T, V> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

pub struct X {
    x: i32,
}
impl Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("X").field("x", &self.x).finish()
    }
}
