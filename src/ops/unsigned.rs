use crate::{multirange::Ranged, range::Range, *};

macro_rules! ops_unsigned {
    ($name:ident, $p:ty) => {
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
                rhs: Ranged<$p, Rhs>,
            ) -> Ranged<$p, Out> {
                const {
                    if MIN + Rhs::MIN != Out::MIN {
                        panic!("Min mismatch");
                    }

                    if MAX + Rhs::MAX != Out::MAX {
                        panic!("Max mismatch");
                    }
                }

                Ranged::from_unchecked(self.get() + as_repr::as_repr::<$p>(rhs))
            }

            /// Subtract a number from `self`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<1, 2>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<0, 4> = a.sub_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 2);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<1, 2>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<0, 3> = a.sub_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn sub_ranged_to<Rhs: Range<$p>, Out: Range<$p>>(
                self,
                rhs: Ranged<$p, Rhs>,
            ) -> Ranged<$p, Out> {
                const {
                    if MIN - Rhs::MAX != Out::MIN {
                        panic!("Min mismatch");
                    }

                    if MAX - Rhs::MIN != Out::MAX {
                        panic!("Max mismatch");
                    }
                }

                Ranged::from_unchecked(self.get() - as_repr::as_repr::<$p>(rhs))
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
                self, rhs: Ranged<$p, Rhs>
            ) -> Ranged<$p, Out> {
                const {
                    if MIN * Rhs::MIN != Out::MIN {
                        panic!("Min mismatch");
                    }

                    if MAX * Rhs::MAX != Out::MAX {
                        panic!("Max mismatch");
                    }
                }

                Ranged::from_unchecked(self.get() * as_repr::as_repr::<$p>(rhs))
            }
        }
    };
}

ops_unsigned!(RangedU8, u8);
ops_unsigned!(RangedU16, u16);
ops_unsigned!(RangedU32, u32);
ops_unsigned!(RangedU64, u64);
ops_unsigned!(RangedU128, u128);
