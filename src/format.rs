use core::fmt;

use super::*;

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
