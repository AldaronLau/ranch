use core::num::NonZero;

use crate::{multirange::Ranged, range::Range, *};

macro_rules! ops_nonzero_unsigned {
    ($name:ident, $ranged:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> $name<MIN, MAX> {
            /// Add two numbers together.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<1, 3>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<2, 6> = a.add_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 3);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<1, 3>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<1, 6> = a.add_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 3);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn add_ranged_to<Rhs: Range<$p>, Out: Range<$p>>(
                self,
                rhs: Ranged<NonZero<$p>, Rhs>,
            ) -> Ranged<NonZero<$p>, Out> {
                as_repr::as_repr::<$ranged<MIN, MAX>>(self)
                    .add_ranged_to::<Rhs, Out>(rhs.to_ranged())
                    .to_ranged_nonzero()
                    .unwrap()
            }

            /// Multiply two numbers together.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<2, 3>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<2, 9> = a.mul_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 2);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<2, 3>::new::<2>();")]
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

            /// Raise to an integer power.
            ///
            /// ```rust
            /// # use ranch::*;
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<2>();")]
            /// let b = RangedU32::<2, 3>::new::<2>();
            #[doc = concat!("let output: ", stringify!($name), "::<1, 27> = a.pow_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 4);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            /// # use ranch::*;
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<2>();")]
            /// let b = RangedU32::<2, 3>::new::<2>();
            #[doc = concat!("let output: ", stringify!($name), "::<0, 27> = a.pow_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 4);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn pow_ranged_to<Rhs: Range<u32>, Out: Range<$p>>(
                self,
                rhs: Ranged<u32, Rhs>,
            ) -> Ranged<NonZero<$p>, Out> {
                as_repr::as_repr::<$ranged<MIN, MAX>>(self)
                    .pow_ranged_to::<Rhs, Out>(rhs)
                    .to_ranged_nonzero()
                    .unwrap()
            }
        }
    };
}

ops_nonzero_unsigned!(RangedNonZeroU8, RangedU8, u8);
ops_nonzero_unsigned!(RangedNonZeroU16, RangedU16, u16);
ops_nonzero_unsigned!(RangedNonZeroU32, RangedU32, u32);
ops_nonzero_unsigned!(RangedNonZeroU64, RangedU64, u64);
ops_nonzero_unsigned!(RangedNonZeroU128, RangedU128, u128);
