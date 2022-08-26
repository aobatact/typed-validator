use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    error::Error,
    ffi::{OsStr, OsString},
    fmt::Display,
    marker::PhantomData,
};

use crate::Validator;

#[derive(Debug)]
pub struct MinLen<const MIN: usize>;
#[derive(Debug)]
pub struct MaxLen<const MAX: usize>;
#[derive(Debug)]
pub struct Len<const MIN: usize, const MAX: usize>;
pub type NotEmpty = MinLen<1>;

pub trait LengthType {
    fn get_len(&self) -> usize;
}

macro_rules! define_length_types {
    ($($t : ty $({ $($i_t:ident),* })?),* $(,)?) => {
        $(
        impl$(<$($i_t),*>)* LengthType for $t {
            fn get_len(&self) -> usize {
                self.len()
            }
        }
    )*
    };
}

define_length_types!(
    &[T] {T},
    Vec<T> {T},
    &str,
    String,
    &OsStr,
    OsString,
    HashMap<K,V,S> {K,V,S},
    HashSet<K, S> {K,S},
    BTreeMap<K,V> {K,V},
    BTreeSet<T> {T},
    VecDeque<T> {T},
    LinkedList<T> {T}
);

impl<const MIN: usize, T: LengthType> Validator<T> for MinLen<MIN> {
    type Error = LengthError<Self>;

    fn validate(t: &T) -> Result<(), Self::Error> {
        let len = t.get_len();
        if len >= MIN {
            Ok(())
        } else {
            Err(LengthError::new(len))
        }
    }
}

impl<const MAX: usize, T: LengthType> Validator<T> for MaxLen<MAX> {
    type Error = LengthError<Self>;

    fn validate(t: &T) -> Result<(), Self::Error> {
        let len = t.get_len();
        if len <= MAX {
            Ok(())
        } else {
            Err(LengthError::new(len))
        }
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthType> Validator<T> for Len<MIN, MAX> {
    type Error = LengthError<Self>;

    fn validate(t: &T) -> Result<(), Self::Error> {
        let len = t.get_len();
        if len <= MAX {
            if len >= MIN {
                Ok(())
            } else {
                Err(LengthError::new(len))
            }
        } else {
            Err(LengthError::new(len))
        }
    }
}

#[derive(Debug, Clone)]
pub struct LengthError<T> {
    actual: usize,
    t: PhantomData<T>,
}

impl<T> LengthError<T> {
    pub fn new(actual: usize) -> Self {
        Self {
            actual,
            t: PhantomData,
        }
    }
}

impl<const MIN: usize> Display for LengthError<MinLen<MIN>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Length {} is smaller then {}",
            self.actual, MIN
        ))
    }
}

impl<const MAX: usize> Display for LengthError<MaxLen<MAX>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Length {} is larger then {}",
            self.actual, MAX
        ))
    }
}

impl<const MIN: usize, const MAX: usize> Display for LengthError<Len<MIN, MAX>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Length {} is not in range ({} .. {})",
            self.actual, MIN, MAX
        ))
    }
}

impl<T> Error for LengthError<T> where LengthError<T>: Display + std::fmt::Debug {}

#[cfg(test)]
mod test {
    use crate::{
        validators::length::{Len, MaxLen},
        Wrapper,
    };

    use super::MinLen;

    #[test]
    fn length_test() {
        assert!(Wrapper::<_, MinLen<4>>::new("xxxx".to_string()).is_ok());
        assert!(Wrapper::<_, MinLen<5>>::new("xxxx".to_string()).is_err());
        assert!(Wrapper::<_, MinLen<4>>::new("xxxx").is_ok());
        assert!(Wrapper::<_, MinLen<5>>::new("xxxx").is_err());
        assert!(Wrapper::<_, MaxLen<4>>::new("xxxx").is_ok());
        assert!(Wrapper::<_, MaxLen<3>>::new("xxxx").is_err());
        assert!(Wrapper::<_, Len<4, 5>>::new("xxxx").is_ok());
        assert!(Wrapper::<_, Len<4, 5>>::new("xxx").is_err());
        assert!(Wrapper::<_, Len<4, 5>>::new("xxxxxx").is_err());
    }
}
