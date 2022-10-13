use crate::Validator;
use std::convert::Infallible;
macro_rules! TupleValidatorsInner {
($( $t : ident),*) => {
    impl<T0, $($t),*> Validator<T0> for ($($t),*)
    where
    $(
        $t : Validator<T0>,
        <$t as Validator<T0>>::Error : 'static,
    )*
    {
        type Error = TupleErrors;
        fn validate(v: &T0) -> Result<(), Self::Error>{
            let mut vec = TupleErrors::new();
            $(
                if let Err(e) = $t::validate(v) {
                    vec.push_error(e);
                }
            )*
            if vec.is_empty() {
                Ok(())
            }else {
                Err(vec)
            }
        }
    }
};
}

macro_rules! TupleValidators {
    ($t0: ident,) => {};
    ($t0: ident,  $( $t1 : tt, )+ ) => {
        TupleValidatorsInner!{$t0, $($t1),*}
        TupleValidators!{$($t1,)*}
    };
}

#[derive(Debug, Default)]
pub struct TupleErrors(Vec<Box<dyn std::error::Error>>);
impl std::ops::Deref for TupleErrors {
    type Target = Vec<Box<dyn std::error::Error>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TupleErrors {
    fn new() -> Self {
        Self(vec![])
    }

    #[inline(never)]
    fn push_error<E: std::error::Error + 'static>(&mut self, e: E) {
        self.0.push(Box::new(e) as Box<dyn std::error::Error>);
    }
}
impl std::fmt::Display for TupleErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.is_empty() {
            f.write_str("Mutiple validation errors : [")?;
            for e in &self.0 {
                e.fmt(f)?;
                f.write_str(", ")?;
            }
            f.write_str("]")
        } else {
            Ok(())
        }
    }
}
impl std::error::Error for TupleErrors {}

TupleValidators!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T,);

impl<T> Validator<T> for () {
    type Error = Infallible;

    fn validate(_t: &T) -> Result<(), Self::Error> {
        Ok(())
    }
}
