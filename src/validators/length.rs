use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

use crate::Validator;

pub struct MinLen<const MIN: usize>;
pub struct MaxLen<const MAX: usize>;
pub struct Len<const MIN: usize, const MAX: usize>;
pub type NotEmpty = MinLen<1>;

pub trait LengthType {
    fn get_len(&self) -> usize;
}

impl<T> LengthType for [T] {
    fn get_len(&self) -> usize {
        self.len()
    }
}

impl<T> LengthType for Vec<T> {
    fn get_len(&self) -> usize {
        self.len()
    }
}

impl LengthType for str {
    fn get_len(&self) -> usize {
        self.len()
    }
}

impl<K, V, S> LengthType for HashMap<K, V, S> {
    fn get_len(&self) -> usize {
        self.len()
    }
}

impl<T, S> LengthType for HashSet<T, S> {
    fn get_len(&self) -> usize {
        self.len()
    }
}

impl<const MIN: usize, T: LengthType> Validator<T> for MinLen<MIN> {
    type Error = LengthError;

    fn validate(t: &T) -> Result<(), Self::Error> {
        let len = t.get_len();
        if len >= MIN {
            Ok(())
        } else {
            Err(LengthError {
                actual_length: len,
                min: Some(MIN),
                max: None,
            })
        }
    }
}

impl<const MAX: usize, T: LengthType> Validator<T> for MaxLen<MAX> {
    type Error = LengthError;

    fn validate(t: &T) -> Result<(), Self::Error> {
        let len = t.get_len();
        if len <= MAX {
            Ok(())
        } else {
            Err(LengthError {
                actual_length: len,
                max: Some(MAX),
                min: None,
            })
        }
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthType> Validator<T> for Len<MIN, MAX> {
    type Error = LengthError;

    fn validate(t: &T) -> Result<(), Self::Error> {
        let len = t.get_len();
        if len <= MAX {
            if len >= MIN {
                Ok(())
            } else {
                Err(LengthError {
                    actual_length: len,
                    min: Some(MIN),
                    max: None,
                })
            }
        } else {
            Err(LengthError {
                actual_length: len,
                max: Some(MAX),
                min: None,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct LengthError {
    actual_length: usize,
    min: Option<usize>,
    max: Option<usize>,
}

impl Display for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.min, self.max) {
            (None, Some(max)) => {
                f.write_str("Length error : length ")?;
                self.actual_length.fmt(f)?;
                f.write_str(" is longer then ")?;
                max.fmt(f)?;
                f.write_str(".")
            }
            (Some(min), None) => {
                f.write_str("Length error : length ")?;
                self.actual_length.fmt(f)?;
                f.write_str(" is shorter then ")?;
                min.fmt(f)?;
                f.write_str(".")
            }
            (None, None) | (Some(_), Some(_)) => unreachable!(),
        }
    }
}

impl Error for LengthError {}
