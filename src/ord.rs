use crate::{
    RangedI8, RangedI16, RangedI32, RangedI64, RangedI128, RangedU8, RangedU16,
    RangedU32, RangedU64, RangedU128, multirange::Ranged, range::Range,
};

macro_rules! const_ord {
    ($ty: ident, $p: ty) => {
        impl<const MIN: $p, const MAX: $p> $ty<MIN, MAX> {
            /// Return the minimum of two ranged integers.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($ty), ";")]
            #[doc = concat!("let a = ", stringify!($ty), "::<4, 24>::new::<12>();")]
            #[doc = concat!("let b = ", stringify!($ty), "::<6, 12>::new::<6>();")]
            #[doc = concat!("let output: ", stringify!($ty), "<4, 12> = a.min_ranged_to(b);")]
            ///
            /// assert_eq!(output, 6);
            /// ```
            pub const fn min_ranged_to<Other: Range<$p>, Out: Range<$p>>(
                self,
                other: Ranged<$p, Other>,
            ) -> Ranged<$p, Out> {
                const {
                    let min = if MIN < Other::MIN { MIN } else { Other::MIN };
                    let max = if MAX < Other::MAX { MAX } else { Other::MAX };

                    if Out::MIN != min {
                        panic!("Mimatched minimum")
                    }

                    if Out::MAX != max {
                        panic!("Mimatched maximum")
                    }
                }

                let this = self.get();
                let other = as_repr::as_repr::<$p>(other);

                Ranged::from_unchecked(if this < other { this } else { other })
            }

            /// Return the maximum of two ranged integers.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($ty), ";")]
            #[doc = concat!("let a = ", stringify!($ty), "::<4, 24>::new::<12>();")]
            #[doc = concat!("let b = ", stringify!($ty), "::<6, 12>::new::<6>();")]
            #[doc = concat!("let output: ", stringify!($ty), "<6, 24> = a.max_ranged_to(b);")]
            ///
            /// assert_eq!(output, 12);
            /// ```
            pub const fn max_ranged_to<Other: Range<$p>, Out: Range<$p>>(
                self,
                other: Ranged<$p, Other>,
            ) -> Ranged<$p, Out> {
                const {
                    let min = if MIN > Other::MIN { MIN } else { Other::MIN };
                    let max = if MAX > Other::MAX { MAX } else { Other::MAX };

                    if Out::MIN != min {
                        panic!("Mimatched minimum")
                    }

                    if Out::MAX != max {
                        panic!("Mimatched maximum")
                    }
                }

                let this = self.get();
                let other = as_repr::as_repr::<$p>(other);

                Ranged::from_unchecked(if this > other { this } else { other })
            }

            /// Restrict a value to a certain interval.
            ///
            /// # Panics
            ///
            ///  - If `min > max`
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($ty), ";")]
            #[doc = concat!("let a = ", stringify!($ty), "::<6, 24>::new::<12>();")]
            #[doc = concat!("let min = ", stringify!($ty), "::<4, 12>::new::<8>();")]
            #[doc = concat!("let max = ", stringify!($ty), "::<8, 16>::new::<10>();")]
            #[doc = concat!("let output: ", stringify!($ty), "<6, 16> = a.clamp_ranged_to(min, max);")]
            ///
            /// assert_eq!(output, 10);
            /// ```
            pub const fn clamp_ranged_to<
                Min: Range<$p>,
                Max: Range<$p>,
                Out: Range<$p>,
            >(
                self,
                min: Ranged<$p, Min>,
                max: Ranged<$p, Max>,
            ) -> Ranged<$p, Out> {
                const {
                    let min = if MIN > Min::MIN { MIN } else { Min::MIN };
                    let max = if MAX < Max::MAX { MAX } else { Max::MAX };

                    if Out::MIN != min {
                        panic!("Mimatched minimum")
                    }

                    if Out::MAX != max {
                        panic!("Mimatched maximum")
                    }
                }

                let this = self.get();
                let min = as_repr::as_repr::<$p>(min);
                let max = as_repr::as_repr::<$p>(max);

                if min > max {
                    panic!("min > max")
                }

                Ranged::from_unchecked(if this < min {
                    min
                } else if this > max {
                    max
                } else {
                    this
                })
            }
        }
    };
}

const_ord!(RangedI8, i8);
const_ord!(RangedI16, i16);
const_ord!(RangedI32, i32);
const_ord!(RangedI64, i64);
const_ord!(RangedI128, i128);

const_ord!(RangedU8, u8);
const_ord!(RangedU16, u16);
const_ord!(RangedU32, u32);
const_ord!(RangedU64, u64);
const_ord!(RangedU128, u128);
