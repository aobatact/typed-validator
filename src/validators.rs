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
                $i : Option<<$t as Validator<T>>::Error>
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

    };
}

TupleValidators!(TupleError2, [a:A, b:B]);
TupleValidators!(TupleError3, [a:A, b:B, c:C]);
TupleValidators!(TupleError4, [a:A, b:B, c:C, d:D]);
TupleValidators!(TupleError5, [a:A, b:B, c:C, d:D, e:E]);
TupleValidators!(TupleError6, [a:A, b:B, c:C, d:D, e:E, f:F]);
TupleValidators!(TupleError7, [a:A, b:B, c:C, d:D, e:E, f:F, g:G]);
TupleValidators!(TupleError8, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H]);
TupleValidators!(TupleError9, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I]);
TupleValidators!(TupleError10, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J]);
TupleValidators!(TupleError11, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K]);
TupleValidators!(TupleError12, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L]);
TupleValidators!(TupleError13, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M]);
TupleValidators!(TupleError14, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N]);
TupleValidators!(TupleError15, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O]);
TupleValidators!(TupleError16, [a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i: I, j:J, k:K, l:L, m:M, n:N, o:O, p:P]);


