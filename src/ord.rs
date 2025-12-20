use crate::{
    RangedI8, RangedI16, RangedI32, RangedI64, RangedI128, RangedU8, RangedU16,
    RangedU32, RangedU64, RangedU128,
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
            #[doc = concat!("let output = ", stringify!($ty), "::<4, 12>::new::<6>();")]
            ///
            /// assert_eq!(a.min(b), output);
            /// ```
            pub const fn min<
                const OTHER_MIN: $p,
                const OTHER_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                other: $ty<OTHER_MIN, OTHER_MAX>,
            ) -> $ty<OUTPUT_MIN, OUTPUT_MAX> {
                const {
                    let min = if MIN < OTHER_MIN { MIN } else { OTHER_MIN };
                    let max = if MAX < OTHER_MAX { MAX } else { OTHER_MAX };

                    if OUTPUT_MIN != min {
                        panic!("Mimatched minimum")
                    }

                    if OUTPUT_MAX != max {
                        panic!("Mimatched maximum")
                    }
                }

                if self.get() < other.get() {
                    $ty(self.get())
                } else {
                    $ty(other.get())
                }
            }

            /// Return the maximum of two ranged integers.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($ty), ";")]
            #[doc = concat!("let a = ", stringify!($ty), "::<4, 24>::new::<12>();")]
            #[doc = concat!("let b = ", stringify!($ty), "::<6, 12>::new::<6>();")]
            #[doc = concat!("let output = ", stringify!($ty), "::<6, 24>::new::<12>();")]
            ///
            /// assert_eq!(a.max(b), output);
            /// ```
            pub const fn max<
                const OTHER_MIN: $p,
                const OTHER_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                other: $ty<OTHER_MIN, OTHER_MAX>,
            ) -> $ty<OUTPUT_MIN, OUTPUT_MAX> {
                const {
                    let min = if MIN > OTHER_MIN { MIN } else { OTHER_MIN };
                    let max = if MAX > OTHER_MAX { MAX } else { OTHER_MAX };

                    if OUTPUT_MIN != min {
                        panic!("Mimatched minimum")
                    }

                    if OUTPUT_MAX != max {
                        panic!("Mimatched maximum")
                    }
                }

                if self.get() > other.get() {
                    $ty(self.get())
                } else {
                    $ty(other.get())
                }
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
            #[doc = concat!("let output = ", stringify!($ty), "::<6, 16>::new::<10>();")]
            ///
            /// assert_eq!(a.clamp(min, max), output);
            /// ```
            pub const fn clamp<
                const MIN_MIN: $p,
                const MIN_MAX: $p,
                const MAX_MIN: $p,
                const MAX_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                min: $ty<MIN_MIN, MIN_MAX>,
                max: $ty<MAX_MIN, MAX_MAX>,
            ) -> $ty<OUTPUT_MIN, OUTPUT_MAX> {
                const {
                    let min = if MIN > MIN_MIN { MIN } else { MIN_MIN };
                    let max = if MAX < MAX_MAX { MAX } else { MAX_MAX };

                    if OUTPUT_MIN != min {
                        panic!("Mimatched minimum")
                    }

                    if OUTPUT_MAX != max {
                        panic!("Mimatched maximum")
                    }
                }

                if min.get() > max.get() {
                    panic!("min > max")
                }

                if self.get() < min.get() {
                    $ty(min.get())
                } else if self.get() > max.get() {
                    $ty(max.get())
                } else {
                    $ty(self.get())
                }
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
