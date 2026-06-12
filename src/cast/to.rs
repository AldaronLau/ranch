use core::num::NonZero;

use as_repr::AsRepr;

use crate::{
    cast::{
        as_primitive::{self, AsPrimitive},
        as_repr_primitive::{self, AsReprPrimitive},
    },
    multirange::Ranged,
    num::rangeable_primitive::RangeablePrimitive,
    range::Range,
    *,
};

pub trait IsNonZero {}

macro_rules! to {
    ($nonzero:ident, $type:ident, $p:ty, $f:ty, $g:ty) => {
        impl IsNonZero for NonZero<$p> {}

        impl<Rn> Ranged<NonZero<$p>, Rn>
        where
            Rn: Range<$p>
        {
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
                T: RangeablePrimitive<ZeroablePrimitive = T>,
                R: Range<T>,
                Ranged<$p, Rn>: AsPrimitive<T>,
                Ranged<T, R>: AsPrimitive<$p>,
            {
                let ranged: Ranged<$p, Rn> = as_repr::as_repr(self);

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
                    RangeablePrimitive<ZeroablePrimitive = T::ZeroablePrimitive>,
                R: Range<T::ZeroablePrimitive>,
                Ranged<$p, Rn>: AsPrimitive<T::ZeroablePrimitive>,
                Ranged<T::ZeroablePrimitive, R>: AsPrimitive<$p>,
                Ranged<T::ZeroablePrimitive, R>: AsRepr<Option<Ranged<T, R>>>,
            {
                let ranged: Ranged<$p, Rn> = as_repr::as_repr(self);
                let Some(ranged) = ranged.to_ranged_nonzero() else {
                    unreachable!()
                };

                ranged
            }
        }

        impl<Rn> Ranged<$p, Rn>
        where
            Rn: Range<$p>
        {
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
            ///
            /// ```rust,compile_fail,E0080
            /// # use ranch::bitwise::*;
            #[doc = concat!("let _: ", stringify!($g), " = ", stringify!($f), "::new::<2>().to_ranged();")]
            /// ```
            ///
            /// ```rust,compile_fail,E0080
            /// # use ranch::bitwise::*;
            #[doc = concat!("let _: ", stringify!($f), " = ", stringify!($g), "::new::<2>().to_ranged();")]
            /// ```
            pub const fn to_ranged<T, R>(self) -> Ranged<T, R>
            where
                T: RangeablePrimitive<ZeroablePrimitive = T>,
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

                        if as_repr_primitive::gt(min, Rn::MIN) {
                            panic!("minimum must be lower or match");
                        }

                        if as_repr_primitive::lt(max, Rn::MAX) {
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

                        if as_repr_primitive::gt(R::MIN, min) {
                            panic!("minimum must be lower or match");
                        }

                        if as_repr_primitive::lt(R::MAX, max) {
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
                    RangeablePrimitive<ZeroablePrimitive = T::ZeroablePrimitive>,
                R: Range<T::ZeroablePrimitive>,
                Self: AsPrimitive<T::ZeroablePrimitive>,
                Ranged<T::ZeroablePrimitive, R>: AsPrimitive<$p>,
                Ranged<T::ZeroablePrimitive, R>: AsRepr<Option<Ranged<T, R>>>,
            {
                const {
                    if as_repr_primitive::is_zero(R::MIN) {
                        panic!("A non-zero integer's minimum cannot be zero");
                    }

                    if as_repr_primitive::is_zero(R::MAX) {
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

                        if as_repr_primitive::gt(min, Rn::MIN)
                            && Rn::MIN != 0
                            && min - 1 != 0
                        {
                            panic!(
                                "minimum must be lower or match or exclude \
                                 zero",
                            );
                        }

                        if as_repr_primitive::lt(max, Rn::MAX)
                            && Rn::MAX != 0
                            && max + 1 != 0
                        {
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

                        if as_repr_primitive::gt(R::MIN, min)
                            && Rn::MIN != 0
                            && !as_repr_primitive::is_one(R::MIN)
                        {
                            panic!(
                                "minimum must be lower or match or exclude \
                                 zero",
                            );
                        }

                        if as_repr_primitive::lt(R::MAX, max)
                            && Rn::MAX != 0
                            && !as_repr_primitive::is_minus_one(R::MAX)
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

to!(RangedNonZeroU8, RangedU8, u8, U8, I8);
to!(RangedNonZeroU16, RangedU16, u16, U16, I16);
to!(RangedNonZeroU32, RangedU32, u32, U32, I32);
to!(RangedNonZeroU64, RangedU64, u64, U64, I64);
to!(RangedNonZeroU128, RangedU128, u128, U128, I128);
to!(RangedNonZeroI8, RangedI8, i8, I8, U8);
to!(RangedNonZeroI16, RangedI16, i16, I16, U16);
to!(RangedNonZeroI32, RangedI32, i32, I32, U32);
to!(RangedNonZeroI64, RangedI64, i64, I64, U64);
to!(RangedNonZeroI128, RangedI128, i128, I128, U128);
