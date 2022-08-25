use crate::Validator;
use std::marker::PhantomData;
macro_rules! TupleValidators {
($err:ident, [$($i: ident : $t : ident),*]) => {
    impl<T, $($t),*> Validator<T> for ($($t),*)
    where
    $(
        $t : Validator<T>,
    )*
    {
        type Error = $err<T, $($t),*>;
        fn validate(v: &T) -> Result<(), Self::Error>{
            let mut x = $err::default();
            let mut flag = false;
            $(
                let err = $t::validate(v);
                flag |= err.is_err();
                x.$i = err.err();
            )*
            if flag {
                Ok(())
            }else {
                Err(x)
            }
        }
    }

    pub struct $err<T, $($t : Validator<T>),*>{
        pdt : PhantomData<T>,
        $(
            pub $i : Option<<$t as Validator<T>>::Error>
        ),*
    }

    impl<T, $($t : Validator<T>),*> Default for $err<T, $($t ),*>{
        fn default() -> Self {
            Self {
                pdt : PhantomData::<T>,
                $(
                    $i : Option::None,
                )*
            }
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
