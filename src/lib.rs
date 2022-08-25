use std::{cmp::Ordering, fmt::Debug, marker::PhantomData, ops::Deref};

pub mod validators;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wrapper<T, V>(T, PhantomData<V>);

pub trait Validator<T> {
    type Error: std::error::Error;

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

// impl<T, V, Error> TryFrom<T> for Wrapper<T,V>where

// V: Validator<T, Error = Error>,
// {
//     type Error = Error;

//     fn try_from(value: T) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

impl<T, V> AsRef<T> for Wrapper<T, V> {
    fn as_ref(&self) -> &T {
        &self.0
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
