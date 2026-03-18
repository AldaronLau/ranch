use core::num::NonZero;

use crate::{multirange::Ranged, range::Range, *};

macro_rules! ops_nonzero_signed {
    ($name:ident, $ranged:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> $name<MIN, MAX> {
            /// Multiply two numbers together.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<-2, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<-1, 3>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<-6, 9> = a.mul_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 2);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<-2, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<-1, 3>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<1, 9> = a.mul_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn mul_ranged_to<Rhs: Range<$p>, Out: Range<$p>>(
                self,
                rhs: Ranged<NonZero<$p>, Rhs>,
            ) -> Ranged<NonZero<$p>, Out> {
                as_repr::as_repr::<$ranged<MIN, MAX>>(self)
                    .mul_ranged_to::<Rhs, Out>(rhs.to_ranged())
                    .to_ranged_nonzero()
                    .unwrap()
            }
        }
    };
}

ops_nonzero_signed!(RangedNonZeroI8, RangedI8, i8);
ops_nonzero_signed!(RangedNonZeroI16, RangedI16, i16);
ops_nonzero_signed!(RangedNonZeroI32, RangedI32, i32);
ops_nonzero_signed!(RangedNonZeroI64, RangedI64, i64);
ops_nonzero_signed!(RangedNonZeroI128, RangedI128, i128);
