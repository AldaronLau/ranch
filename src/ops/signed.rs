use crate::{multirange::Ranged, range::Range, *};

macro_rules! ops_signed {
    ($name:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> $name<MIN, MAX> {
            /// Add two numbers together.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<-1, 3>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<0, 6> = a.add_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 3);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<1, 3>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<-1, 3>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<1, 6> = a.add_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 3);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn add_ranged_to<
                Rhs: Range<$p>,
                Out: Range<$p>,
            >(
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
            #[doc = concat!("let b = ", stringify!($name), "::<-1, 3>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<-1, 6> = a.sub_ranged(b);")]
            ///
            /// assert_eq!(output.get(), 2);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<-1, 3>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<0, 6> = a.sub_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn sub_ranged_to<
                Rhs: Range<$p>,
                Out: Range<$p>,
            >(
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
        }
    };
}

ops_signed!(RangedI8, i8);
ops_signed!(RangedI16, i16);
ops_signed!(RangedI32, i32);
ops_signed!(RangedI64, i64);
ops_signed!(RangedI128, i128);
