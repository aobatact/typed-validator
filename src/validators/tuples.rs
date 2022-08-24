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
            let mut err = f.debug_struct(stringify!($err));
            let mut i = 0;
            $(
                if let Some(ref e) = self.$i {
                    err.field(&i.to_string(), &e);
                    i += 1;
                }
            )*

            err.finish()
        }
    }

    #[allow(unused_assignments)]
    impl<T, $($t : Validator<T>),*> std::fmt::Display for $err<T, $($t ),*>
        where 
        $(
            <$t as Validator<T>>::Error: std::fmt::Display,
        )*
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
};
}

#[rustfmt::skip]
TupleValidators!(TupleError2, [a:A, b:B]);
#[rustfmt::skip]
TupleValidators!(TupleError3, [a:A, b:B, c:C]);
#[rustfmt::skip]
TupleValidators!(TupleError4, [a:A, b:B, c:C, d:D]);
#[rustfmt::skip]
TupleValidators!(TupleError5, [a:A, b:B, c:C, d:D, e:E]);
#[rustfmt::skip]
TupleValidators!(TupleError6, [a:A, b:B, c:C, d:D, e:E, f:F]);
#[rustfmt::skip]
TupleValidators!(TupleError7, [a:A, b:B, c:C, d:D, e:E, f:F, g:G]);
#[rustfmt::skip]
TupleValidators!(TupleError8, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H]);
#[rustfmt::skip]
TupleValidators!(TupleError9, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I]);
#[rustfmt::skip]
TupleValidators!(TupleError10, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J]);
#[rustfmt::skip]
TupleValidators!(TupleError11, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K]);
#[rustfmt::skip]
TupleValidators!(TupleError12, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L]);
#[rustfmt::skip]
TupleValidators!(TupleError13, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M]);
#[rustfmt::skip]
TupleValidators!(TupleError14, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N]);
#[rustfmt::skip]
TupleValidators!(TupleError15, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O]);
#[rustfmt::skip]
TupleValidators!(TupleError16, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O, p:P]);
