use crate::Validator;
use std::{convert::Infallible, marker::PhantomData};
macro_rules! TupleValidators {
($err:ident, [$($i: ident : $t : ident),*]) => {
    impl<T, $($t),*> Validator<T> for ($($t),*)
    where
    $(
        $t : Validator<T>,
    )*
    {
        type Error = Box::<$err<T, $($t),*>>;
        fn validate(v: &T) -> Result<(), Self::Error>{
            let mut x = $err::none();
            let mut count = 0;
            $(
                let err = $t::validate(v);
                count += if err.is_err() {1} else {0};
                x.$i = err.err();
            )*
            if count > 0 {
                Ok(())
            }else {
                x.count = count;
                Err(x)
            }
        }
    }

    pub struct $err<T, $($t : Validator<T>),*>{
        pdt : PhantomData<T>,
        count: usize,
        $(
            pub $i : Option<<$t as Validator<T>>::Error>
        ),*
    }

    impl<T, $($t : Validator<T>),*> $err<T, $($t ),*>{
        pub fn err_count(&self) -> usize {
            self.count
        }

        fn none() -> Box<Self> {
            Box::new(Self {
                pdt : PhantomData::<T>,
                count : 0,
                $(
                    $i : Option::None,
                )*
            })
        }
    }


    #[allow(unused_assignments)]
    impl<T, $($t : Validator<T>),*> std::fmt::Debug for $err<T, $($t ),*>{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut err = f.debug_tuple(stringify!($err));
            $(
                if let Some(ref e) = self.$i {
                    err.field( &e);
                }
            )*

            err.finish()
        }
    }

    #[allow(unused_assignments)]
    impl<T, $($t : Validator<T>),*> std::fmt::Display for $err<T, $($t ),*>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("mutiple validation errors : [")?;
            let mut first = false;
            $(
                if let Some(ref e) = self.$i {
                    if first {
                        f.write_str(", ")?;
                        first = true;
                    }
                    e.fmt(f)?
                }
            )*
            f.write_str("]")
        }
    }

    impl<T, $($t : Validator<T>),*> std::error::Error for $err<T, $($t ),*>
    {
    }
};
}

#[rustfmt::skip]
TupleValidators!(Tuple2Error, [a:A, b:B]);
#[rustfmt::skip]
TupleValidators!(Tuple3Error, [a:A, b:B, c:C]);
#[rustfmt::skip]
TupleValidators!(Tuple4Error, [a:A, b:B, c:C, d:D]);
#[rustfmt::skip]
TupleValidators!(Tuple5Error, [a:A, b:B, c:C, d:D, e:E]);
#[rustfmt::skip]
TupleValidators!(Tuple6Error, [a:A, b:B, c:C, d:D, e:E, f:F]);
#[rustfmt::skip]
TupleValidators!(Tuple7Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G]);
#[rustfmt::skip]
TupleValidators!(Tuple8Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H]);
#[rustfmt::skip]
TupleValidators!(Tuple9Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I]);
#[rustfmt::skip]
TupleValidators!(Tuple10Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J]);
#[rustfmt::skip]
TupleValidators!(Tuple11Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K]);
#[rustfmt::skip]
TupleValidators!(Tuple12Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L]);
#[rustfmt::skip]
TupleValidators!(Tuple13Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M]);
#[rustfmt::skip]
TupleValidators!(Tuple14Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N]);
#[rustfmt::skip]
TupleValidators!(Tuple15Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O]);
#[rustfmt::skip]
TupleValidators!(Tuple16Error, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O, p:P]);

impl<T> Validator<T> for () {
    type Error = Infallible;

    fn validate(_t: &T) -> Result<(), Self::Error> {
        Ok(())
    }
}
