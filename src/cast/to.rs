use core::num::NonZero;

use as_repr::AsRepr;

use crate::{
    cast::as_primitive::{self, AsPrimitive},
    cmp::Cmp,
    multirange::Ranged,
    num::rangeable_primitive::RangeablePrimitive,
    range::Range,
    *,
};

pub trait IsNonZero {}

macro_rules! to {
    ($nonzero:ident, $type:ident, $p:ty) => {
        impl IsNonZero for NonZero<$p> {}

        impl<const MIN: $p, const MAX: $p> $nonzero<MIN, MAX> {
            /// Convert to a new [`Ranged`] type, optionally expanding the
            /// range.
            ///
            /// The output type's range must include the range of `Self`.
            ///
            /// ```rust
            /// # use ranch::*;
            #[doc = concat!("let ranged = ", stringify!($nonzero), "::<1, 100>::new::<42>();")]
            ///
            /// let expanded_u8: RangedU8<1, 100> = ranged.to_ranged();
            /// let expanded_u16: RangedU16<1, 100> = ranged.to_ranged();
            /// let expanded_u32: RangedU32<1, 100> = ranged.to_ranged();
            /// let expanded_u64: RangedU64<1, 100> = ranged.to_ranged();
            /// let expanded_u128: RangedU128<1, 100> = ranged.to_ranged();
            /// let expanded_i8: RangedI8<1, 100> = ranged.to_ranged();
            /// let expanded_i16: RangedI16<1, 100> = ranged.to_ranged();
            /// let expanded_i32: RangedI32<1, 100> = ranged.to_ranged();
            /// let expanded_i64: RangedI64<1, 100> = ranged.to_ranged();
            /// let expanded_i128: RangedI128<1, 100> = ranged.to_ranged();
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
            pub const fn to_ranged<T, R>(self) -> Ranged<T, R>
            where
                T: RangeablePrimitive<ZeroablePrimitive = T> + Cmp,
                R: Range<T>,
                $type<MIN, MAX>: AsPrimitive<T>,
                Ranged<T, R>: AsPrimitive<$p>,
            {
                let ranged: $type<MIN, MAX> = as_repr::as_repr(self);

                ranged.to_ranged()
            }

            /// Convert to a new non-zero [`Ranged`] type, optionally expanding
            /// the range.
            ///
            /// The output type's range must include the range of `Self`.
            ///
            /// ```rust
            /// # use ranch::*;
            #[doc = concat!("let ranged = ", stringify!($nonzero), "::<1, 50>::new::<42>();")]
            ///
            /// let expanded_u8: RangedNonZeroU8<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_u16: RangedNonZeroU16<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_u32: RangedNonZeroU32<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_u64: RangedNonZeroU64<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_u128: RangedNonZeroU128<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_i8: RangedNonZeroI8<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_i16: RangedNonZeroI16<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_i32: RangedNonZeroI32<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_i64: RangedNonZeroI64<1, 100> = ranged.to_ranged_nonzero();
            /// let expanded_i128: RangedNonZeroI128<1, 100> = ranged.to_ranged_nonzero();
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
            pub const fn to_ranged_nonzero<T, R>(self) -> Ranged<T, R>
            where
                T: RangeablePrimitive + IsNonZero,
                T::ZeroablePrimitive:
                    RangeablePrimitive<ZeroablePrimitive = T::ZeroablePrimitive>
                    + Cmp,
                R: Range<T::ZeroablePrimitive>,
                $type<MIN, MAX>: AsPrimitive<T::ZeroablePrimitive>,
                Ranged<T::ZeroablePrimitive, R>: AsPrimitive<$p>,
                Ranged<T::ZeroablePrimitive, R>: AsRepr<Option<Ranged<T, R>>>,
            {
                let ranged: $type<MIN, MAX> = as_repr::as_repr(self);
                let Some(ranged) = ranged.to_ranged_nonzero() else {
                    unreachable!()
                };

                ranged
            }
        }

        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Convert to a new [`Ranged`] type, optionally expanding the
            /// range.
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
            /// ```rust,compile_fail,E0080
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
                        let min = as_primitive::as_primitive_expanding(
                            Self::MIN,
                        );
                        let max = as_primitive::as_primitive_expanding(
                            Self::MAX,
                        );

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

            /// Convert to a new non-zero [`Ranged`] type, optionally expanding
            /// the range.
            ///
            /// The output type's range must include the range of `Self`.
            ///
            /// If you don't need to change the range (range neither includes
            /// zero nor needs to be expanded), try using
            #[doc = concat!("[`", stringify!($nonzero), "::from_ranged()`].")]
            ///
            /// ```rust
            /// # use ranch::*;
            #[doc = concat!("let ranged = ", stringify!($type), "::<0, 2>::new::<2>();")]
            ///
            /// let expanded_u8: RangedNonZeroU8<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_u16: RangedNonZeroU16<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_u32: RangedNonZeroU32<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_u64: RangedNonZeroU64<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_u128: RangedNonZeroU128<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_i8: RangedNonZeroI8<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_i16: RangedNonZeroI16<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_i32: RangedNonZeroI32<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_i64: RangedNonZeroI64<1, 4> = ranged.to_ranged_nonzero().unwrap();
            /// let expanded_i128: RangedNonZeroI128<1, 4> = ranged.to_ranged_nonzero().unwrap();
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
            pub const fn to_ranged_nonzero<T, R>(self)
            -> Option<Ranged<T, R>>
            where
                T: RangeablePrimitive + IsNonZero,
                T::ZeroablePrimitive:
                    RangeablePrimitive<ZeroablePrimitive = T::ZeroablePrimitive>
                    + Cmp,
                R: Range<T::ZeroablePrimitive>,
                Self: AsPrimitive<T::ZeroablePrimitive>,
                Ranged<T::ZeroablePrimitive, R>: AsPrimitive<$p>,
                Ranged<T::ZeroablePrimitive, R>: AsRepr<Option<Ranged<T, R>>>,
            {
                const {
                    if cmp::is_zero(R::MIN) {
                        panic!("A non-zero integer's minimum cannot be zero");
                    }

                    if cmp::is_zero(R::MAX) {
                        panic!("A non-zero integer's maximum cannot be zero");
                    }

                    // validate range
                    if size_of::<T::ZeroablePrimitive>() < size_of::<Self>() {
                        // shrinking - expand the output range for comparison
                        let min = as_primitive::as_primitive_expanding(
                            Ranged::<T::ZeroablePrimitive, R>::MIN,
                        );
                        let max = as_primitive::as_primitive_expanding(
                            Ranged::<T::ZeroablePrimitive, R>::MAX,
                        );

                        if cmp::gt(min, MIN) && MIN != 0 && min - 1 != 0 {
                            panic!(
                                "minimum must be lower or match or exclude \
                                 zero",
                            );
                        }

                        if cmp::lt(max, MAX) && MAX != 0 && max + 1 != 0 {
                            panic!(
                                "maximum must be higher or match or exclude \
                                 zero",
                            );
                        }
                    } else {
                        // expanding - expand the input range for comparison
                        let min = as_primitive::as_primitive_expanding(
                            Self::MIN,
                        );
                        let max = as_primitive::as_primitive_expanding(
                            Self::MAX,
                        );

                        if cmp::gt(R::MIN, min)
                            && MIN != 0
                            && !cmp::is_one(R::MIN)
                        {
                            panic!(
                                "minimum must be lower or match or exclude \
                                 zero",
                            );
                        }

                        if cmp::lt(R::MAX, max)
                            && MAX != 0
                            && !cmp::is_minus_one(R::MAX)
                        {
                            panic!(
                                "maximum must be higher or match or exclude \
                                 zero",
                            );
                        }
                    }
                }

                let ranged = Ranged::<T::ZeroablePrimitive, R>::from_unchecked(
                    as_primitive::as_primitive(self),
                );

                as_repr::as_repr(ranged)
            }
        }
    };
}

// FIXME: NonZero
to!(RangedNonZeroU8, RangedU8, u8);
to!(RangedNonZeroU16, RangedU16, u16);
to!(RangedNonZeroU32, RangedU32, u32);
to!(RangedNonZeroU64, RangedU64, u64);
to!(RangedNonZeroU128, RangedU128, u128);
to!(RangedNonZeroI8, RangedI8, i8);
to!(RangedNonZeroI16, RangedI16, i16);
to!(RangedNonZeroI32, RangedI32, i32);
to!(RangedNonZeroI64, RangedI64, i64);
to!(RangedNonZeroI128, RangedI128, i128);
