use crate::Validator;
use std::convert::Infallible;
macro_rules! TupleValidators {
([$($i: ident : $t : ident),*]) => {
    impl<T, $($t),*> Validator<T> for ($($t),*)
    where
    $(
        $t : Validator<T>,
        <$t as Validator<T>>::Error : 'static,
    )*
    {
        type Error = TupleErrors;
        fn validate(v: &T) -> Result<(), Self::Error>{
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

#[rustfmt::skip]
TupleValidators!( [a:A, b:B]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O]);
#[rustfmt::skip]
TupleValidators!( [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O, p:P]);

impl<T> Validator<T> for () {
    type Error = Infallible;

    fn validate(_t: &T) -> Result<(), Self::Error> {
        Ok(())
    }
}
