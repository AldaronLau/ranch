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
            #[doc = concat!("let b = ", stringify!($name), "::<-1, 3>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<-1, 6> = a.sub_ranged_to(b);")]
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
                rhs: Ranged<$p, Rhs>,
            ) -> Ranged<$p, Out> {
                const {
                    let (min_min, min_max) = (MIN * Rhs::MIN, MIN * Rhs::MAX);
                    let min = if min_min < min_max { min_min } else { min_max };
                    let (max_min, max_max) = (MAX * Rhs::MIN, MAX * Rhs::MAX);
                    let max = if max_min > max_max { max_min } else { max_max };

                    if min != Out::MIN {
                        panic!("Min mismatch");
                    }

                    if max != Out::MAX {
                        panic!("Max mismatch");
                    }
                }

                Ranged::from_unchecked(self.get() * as_repr::as_repr::<$p>(rhs))
            }

            /// Raise to an integer power.
            ///
            /// ```rust
            /// # use ranch::*;
            #[doc = concat!("let a = ", stringify!($name), "::<-1, 3>::new::<2>();")]
            /// let b = RangedU32::<2, 3>::new::<2>();
            #[doc = concat!("let output: ", stringify!($name), "::<-1, 27> = a.pow_ranged_to(b);")]
            ///
            /// assert_eq!(output.get(), 4);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            /// # use ranch::*;
            #[doc = concat!("let a = ", stringify!($name), "::<-1, 3>::new::<2>();")]
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
            ) -> Ranged<$p, Out> {
                const {
                    if MIN.is_negative() {
                        let min = MIN.pow(Rhs::MIN);
                        let max = MAX.pow(Rhs::MAX);
                        let rhs_max = if Rhs::MAX % 2 == 0 {
                            Rhs::MAX - 1
                        } else {
                            Rhs::MAX
                        };
                        let rhs_min = if Rhs::MIN % 2 == 0 {
                            Rhs::MIN - 1
                        } else {
                            Rhs::MIN
                        };
                        let min_min = MIN.pow(rhs_min);
                        let min_max = MAX.pow(rhs_max);
                        let min = if min_min < min { min_min } else { min };
                        let min = if min_max < min { min_max } else { min };

                        if min != Out::MIN {
                            panic!("Min mismatch");
                        } else if max != Out::MAX {
                            panic!("Max mismatch");
                        }
                    } else if MIN.pow(Rhs::MIN) != Out::MIN {
                        panic!("Min mismatch");
                    } else if MAX.pow(Rhs::MAX) != Out::MAX {
                        panic!("Max mismatch");
                    }
                }

                Ranged::from_unchecked(
                    self.get().pow(as_repr::as_repr::<u32>(rhs)),
                )
            }

            /// Divide `self` by a number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<1, 2> = a.div_ranged_to(b).number().unwrap();")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($name), ";")]
            #[doc = concat!("let a = ", stringify!($name), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($name), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($name), "::<0, 2> = a.div_ranged_to(b).number().unwrap();")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_ranged_to<Rhs: Range<$p>, Out: Range<$p>>(
                self,
                rhs: Ranged<$p, Rhs>,
            ) -> Quotient<Ranged<$p, Out>> {
                const {
                    let (min_min, min_max) = (MIN / Rhs::MIN, MIN / Rhs::MAX);
                    let (max_min, max_max) = (MAX / Rhs::MIN, MAX / Rhs::MAX);
                    let min = if min_min < min_max { min_min } else { min_max };
                    let min = if max_min < min { max_min } else { min };
                    let min = if max_max < min { max_max } else { min };
                    let max = if max_min > max_max { max_min } else { max_max };
                    let max = if min_min > min { min_min } else { max };
                    let max = if min_max > min { min_max } else { max };

                    if min != Out::MIN {
                        panic!("Min mismatch");
                    }

                    if max != Out::MAX {
                        panic!("Max mismatch");
                    }
                }

                let rhs = as_repr::as_repr::<$p>(rhs);

                if rhs == 0 {
                    Quotient::Nan
                } else {
                    Quotient::Number(Ranged::from_unchecked(self.get() / rhs))
                }
            }
        }
    };
}

ops_signed!(RangedI8, i8);
ops_signed!(RangedI16, i16);
ops_signed!(RangedI32, i32);
ops_signed!(RangedI64, i64);
ops_signed!(RangedI128, i128);
