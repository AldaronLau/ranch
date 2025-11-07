use core::fmt;

use super::*;

macro_rules! impl_quotient_fmt {
    ($type:ident, [$($Trait:ident),* $(,)?] $(,)?) => {
        $(
            impl<T> fmt::$Trait for Quotient<T> where T: fmt::$Trait {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        Quotient::Nan => f.write_str("NaN"),
                        Quotient::Number(number) => number.fmt(f),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_ranged_fmt {
    ($type:ident, $primitive:ty, [$($Trait:ident),* $(,)?] $(,)?) => {
        $(
            impl<const MIN: $primitive, const MAX: $primitive> fmt::$Trait
            for $type<MIN, MAX>
            where
            {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    <$primitive as fmt::$Trait>::fmt(&self.get(), f)
                }
            }
        )*
    };
}

impl_quotient_fmt!(
    Quotient,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU8,
    u8,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU16,
    u16,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU32,
    u32,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU64,
    u64,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU128,
    u128,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedI8,
    i8,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedI16,
    i16,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedI32,
    i32,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedI64,
    i64,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedI128,
    i128,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);
