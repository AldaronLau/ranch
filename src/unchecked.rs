#![allow(unsafe_code)]

use as_repr::AsRepr;

use crate::*;

macro_rules! unchecked {
    ($ranged:ident, $from:ident, $primitive:ty) => {
        impl<const MIN: $primitive, const MAX: $primitive> $ranged<MIN, MAX> {
            /// Create a new ranged integer, without checking if the value is in
            /// range.
            ///
            /// # Safety
            ///
            ///  - This method alone will never cause undefined behavior
            ///  - This results in undefined behavior when used by a safe method
            ///    that assumes the value is within range or else produces
            ///    undefined behavior
            #[must_use]
            pub const unsafe fn $from(value: $primitive) -> Self {
                Self::from_unchecked(value)
            }

            /// Compute `self + rhs`, assuming overflow and out-of-range cannot
            /// occur.
            ///
            /// # Safety
            ///
            ///  - This results in undefined behavior when overflow occurs
            ///  - This results in undefined behavior when used by a safe method
            ///    that assumes the value is within range or else produces
            ///    undefined behavior
            #[must_use]
            pub const unsafe fn unchecked_add(
                self,
                rhs: impl AsRepr<$primitive>,
            ) -> Self {
                let this: $primitive = as_repr::as_repr(self);
                let rhs: $primitive = as_repr::as_repr(rhs);

                unsafe { Self::$from(this.unchecked_add(rhs)) }
            }

            /// Compute `self - rhs`, assuming overflow and out-of-range cannot
            /// occur.
            ///
            /// # Safety
            ///
            ///  - This results in undefined behavior when overflow occurs
            ///  - This results in undefined behavior when used by a safe method
            ///    that assumes the value is within range or else produces
            ///    undefined behavior
            #[must_use]
            pub const unsafe fn unchecked_sub(
                self,
                rhs: impl AsRepr<$primitive>,
            ) -> Self {
                let this: $primitive = as_repr::as_repr(self);
                let rhs: $primitive = as_repr::as_repr(rhs);

                unsafe { Self::$from(this.unchecked_sub(rhs)) }
            }

            /// Compute `self * rhs`, assuming overflow and out-of-range cannot
            /// occur.
            ///
            /// # Safety
            ///
            ///  - This results in undefined behavior when overflow occurs
            ///  - This results in undefined behavior when used by a safe method
            ///    that assumes the value is within range or else produces
            ///    undefined behavior
            #[must_use]
            pub const unsafe fn unchecked_mul(
                self,
                rhs: impl AsRepr<$primitive>,
            ) -> Self {
                let this: $primitive = as_repr::as_repr(self);
                let rhs: $primitive = as_repr::as_repr(rhs);

                unsafe { Self::$from(this.unchecked_mul(rhs)) }
            }
        }
    };
}

unchecked!(RangedU8, from_u8_unchecked, u8);
unchecked!(RangedU16, from_u16_unchecked, u16);
unchecked!(RangedU32, from_u32_unchecked, u32);
unchecked!(RangedU64, from_u64_unchecked, u64);
unchecked!(RangedU128, from_u128_unchecked, u128);
unchecked!(RangedI8, from_i8_unchecked, i8);
unchecked!(RangedI16, from_i16_unchecked, i16);
unchecked!(RangedI32, from_i32_unchecked, i32);
unchecked!(RangedI64, from_i64_unchecked, i64);
unchecked!(RangedI128, from_i128_unchecked, i128);
