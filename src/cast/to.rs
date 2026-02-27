use crate::{
    cast::as_primitive::{self, AsPrimitive},
    cmp::Cmp,
    multirange::Ranged,
    num::rangeable_primitive::RangeablePrimitive,
    range::Range,
    *,
};

macro_rules! to {
    ($type:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Convert to a different [`Ranged`] type.
            ///
            /// The output type's range must include the range of `Self`.
            ///
            /// ```rust
            /// # use ranch::*;
            #[doc = concat!("let ranged = ", stringify!($type), "::<0, 2>::new::<2>();")]
            ///
            /// let expanded_u8: RangedU8<0, 4> = ranged.to_ranged();
            /// let expanded_u16: RangedU16<0, 4> = ranged.to_ranged();
            /// let expanded_u32: RangedU32<0, 4> = ranged.to_ranged();
            /// let expanded_u64: RangedU64<0, 4> = ranged.to_ranged();
            /// let expanded_u128: RangedU128<0, 4> = ranged.to_ranged();
            /// let expanded_i8: RangedI8<0, 4> = ranged.to_ranged();
            /// let expanded_i16: RangedI16<0, 4> = ranged.to_ranged();
            /// let expanded_i32: RangedI32<0, 4> = ranged.to_ranged();
            /// let expanded_i64: RangedI64<0, 4> = ranged.to_ranged();
            /// let expanded_i128: RangedI128<0, 4> = ranged.to_ranged();
            ///
            /// assert_eq!(ranged.get(), expanded_u8.get() as _);
            /// assert_eq!(ranged.get(), expanded_u16.get() as _);
            /// assert_eq!(ranged.get(), expanded_u32.get() as _);
            /// assert_eq!(ranged.get(), expanded_u64.get() as _);
            /// assert_eq!(ranged.get(), expanded_u128.get() as _);
            /// assert_eq!(ranged.get(), expanded_i8.get() as _);
            /// assert_eq!(ranged.get(), expanded_i16.get() as _);
            /// assert_eq!(ranged.get(), expanded_i32.get() as _);
            /// assert_eq!(ranged.get(), expanded_i64.get() as _);
            /// assert_eq!(ranged.get(), expanded_i128.get() as _);
            /// ```
            ///
            /// Does not compile:
            ///
            /// ```rust,compile_fail
            /// # use ranch::*;
            #[doc = concat!("let ranged = ", stringify!($type), "::<0, 2>::new::<2>();")]
            ///
            /// let expanded_u32: RangedU32<1, 4> = ranged.to_ranged();
            /// ```
            pub const fn to_ranged<T, R>(self) -> Ranged<T, R>
            where
                T: RangeablePrimitive<ZeroablePrimitive = T> + Cmp,
                R: Range<T>,
                Self: AsPrimitive<T>,
                Ranged<T, R>: AsPrimitive<$p>,
            {
                const {
                    // validate range
                    if size_of::<T>() < size_of::<Self>() {
                        // shrinking - expand the output range for comparison
                        let min = as_primitive::as_primitive_expanding(
                            Ranged::<T, R>::MIN,
                        );
                        let max = as_primitive::as_primitive_expanding(
                            Ranged::<T, R>::MAX,
                        );

                        if cmp::gt(min, MIN) {
                            panic!("minimum must be lower or match");
                        }

                        if cmp::lt(max, MAX) {
                            panic!("maximum must be higher or match");
                        }
                    } else {
                        // expanding - expand the input range for comparison
                        let min = as_primitive::as_primitive_expanding(Self::MIN);
                        let max = as_primitive::as_primitive_expanding(Self::MAX);

                        if cmp::gt(R::MIN, min) {
                            panic!("minimum must be lower or match");
                        }

                        if cmp::lt(R::MAX, max) {
                            panic!("maximum must be higher or match");
                        }
                    }
                }

                Ranged::<T, R>::from_unchecked(as_primitive::as_primitive(self))
            }
        }
    };
}

// FIXME: NonZero
to!(RangedU8, u8);
to!(RangedU16, u16);
to!(RangedU32, u32);
to!(RangedU64, u64);
to!(RangedU128, u128);
to!(RangedI8, i8);
to!(RangedI16, i16);
to!(RangedI32, i32);
to!(RangedI64, i64);
to!(RangedI128, i128);
