use core::{num::NonZero, ops::RangeInclusive};

use fastrand::Rng;

use crate::{
    range::{self, RngRanged},
    *,
};

impl RngRanged for Rng {
    /// Generate a random [`RangedU8`].
    fn ranged_u8<const MIN: u8, const MAX: u8>(
        &mut self,
    ) -> RangedU8<MIN, MAX> {
        RangedU8(self.u8(range::range_inclusive::<RangedU8<MIN, MAX>, u8>()))
    }

    /// Generate a random [`RangedU16`].
    fn ranged_u16<const MIN: u16, const MAX: u16>(
        &mut self,
    ) -> RangedU16<MIN, MAX> {
        RangedU16(
            self.u16(range::range_inclusive::<RangedU16<MIN, MAX>, u16>()),
        )
    }

    /// Generate a random [`RangedU32`].
    fn ranged_u32<const MIN: u32, const MAX: u32>(
        &mut self,
    ) -> RangedU32<MIN, MAX> {
        RangedU32(
            self.u32(range::range_inclusive::<RangedU32<MIN, MAX>, u32>()),
        )
    }

    /// Generate a random [`RangedU64`].
    fn ranged_u64<const MIN: u64, const MAX: u64>(
        &mut self,
    ) -> RangedU64<MIN, MAX> {
        RangedU64(
            self.u64(range::range_inclusive::<RangedU64<MIN, MAX>, u64>()),
        )
    }

    /// Generate a random [`RangedU128`].
    fn ranged_u128<const MIN: u128, const MAX: u128>(
        &mut self,
    ) -> RangedU128<MIN, MAX> {
        RangedU128(
            self.u128(range::range_inclusive::<RangedU128<MIN, MAX>, u128>()),
        )
    }

    /// Generate a random [`RangedI8`].
    fn ranged_i8<const MIN: i8, const MAX: i8>(
        &mut self,
    ) -> RangedI8<MIN, MAX> {
        RangedI8(self.i8(range::range_inclusive::<RangedI8<MIN, MAX>, i8>()))
    }

    /// Generate a random [`RangedI16`].
    fn ranged_i16<const MIN: i16, const MAX: i16>(
        &mut self,
    ) -> RangedI16<MIN, MAX> {
        RangedI16(
            self.i16(range::range_inclusive::<RangedI16<MIN, MAX>, i16>()),
        )
    }

    /// Generate a random [`RangedI32`].
    fn ranged_i32<const MIN: i32, const MAX: i32>(
        &mut self,
    ) -> RangedI32<MIN, MAX> {
        RangedI32(
            self.i32(range::range_inclusive::<RangedI32<MIN, MAX>, i32>()),
        )
    }

    /// Generate a random [`RangedI64`].
    fn ranged_i64<const MIN: i64, const MAX: i64>(
        &mut self,
    ) -> RangedI64<MIN, MAX> {
        RangedI64(
            self.i64(range::range_inclusive::<RangedI64<MIN, MAX>, i64>()),
        )
    }

    /// Generate a random [`RangedI128`].
    fn ranged_i128<const MIN: i128, const MAX: i128>(
        &mut self,
    ) -> RangedI128<MIN, MAX> {
        RangedI128(
            self.i128(range::range_inclusive::<RangedI128<MIN, MAX>, i128>()),
        )
    }

    /// Generate a random [`RangedNonZeroI8`].
    fn ranged_nonzero_i8<const MIN: i8, const MAX: i8>(
        &mut self,
    ) -> RangedNonZeroI8<MIN, MAX> {
        let range = const {
            if MIN == 0 || MAX == 0 {
                panic!("MIN or MAX cannot be zero for nonzero ints");
            }

            RangeInclusive::new(MIN, MAX)
        };

        RangedNonZeroI8(
            NonZero::new(if range.contains(&0) {
                let range = const { RangeInclusive::new(MIN + 1, MAX) };
                let value = self.i8(range);

                if value <= 0 { value - 1 } else { value }
            } else {
                self.i8(range::range_inclusive::<RangedI8<MIN, MAX>, i8>())
            })
            .unwrap(),
        )
    }

    /// Generate a random [`RangedNonZeroI16`].
    fn ranged_nonzero_i16<const MIN: i16, const MAX: i16>(
        &mut self,
    ) -> RangedNonZeroI16<MIN, MAX> {
        let range = const {
            if MIN == 0 || MAX == 0 {
                panic!("MIN or MAX cannot be zero for nonzero ints");
            }

            RangeInclusive::new(MIN, MAX)
        };

        RangedNonZeroI16(
            NonZero::new(if range.contains(&0) {
                let range = const { RangeInclusive::new(MIN + 1, MAX) };
                let value = self.i16(range);

                if value <= 0 { value - 1 } else { value }
            } else {
                self.i16(range::range_inclusive::<RangedI16<MIN, MAX>, i16>())
            })
            .unwrap(),
        )
    }

    /// Generate a random [`RangedNonZeroI32`].
    fn ranged_nonzero_i32<const MIN: i32, const MAX: i32>(
        &mut self,
    ) -> RangedNonZeroI32<MIN, MAX> {
        let range = const {
            if MIN == 0 || MAX == 0 {
                panic!("MIN or MAX cannot be zero for nonzero ints");
            }

            RangeInclusive::new(MIN, MAX)
        };

        RangedNonZeroI32(
            NonZero::new(if range.contains(&0) {
                let range = const { RangeInclusive::new(MIN + 1, MAX) };
                let value = self.i32(range);

                if value <= 0 { value - 1 } else { value }
            } else {
                self.i32(range::range_inclusive::<RangedI32<MIN, MAX>, i32>())
            })
            .unwrap(),
        )
    }

    /// Generate a random [`RangedNonZeroI64`].
    fn ranged_nonzero_i64<const MIN: i64, const MAX: i64>(
        &mut self,
    ) -> RangedNonZeroI64<MIN, MAX> {
        let range = const {
            if MIN == 0 || MAX == 0 {
                panic!("MIN or MAX cannot be zero for nonzero ints");
            }

            RangeInclusive::new(MIN, MAX)
        };

        RangedNonZeroI64(
            NonZero::new(if range.contains(&0) {
                let range = const { RangeInclusive::new(MIN + 1, MAX) };
                let value = self.i64(range);

                if value <= 0 { value - 1 } else { value }
            } else {
                self.i64(range::range_inclusive::<RangedI64<MIN, MAX>, i64>())
            })
            .unwrap(),
        )
    }

    /// Generate a random [`RangedNonZeroI128`].
    fn ranged_nonzero_i128<const MIN: i128, const MAX: i128>(
        &mut self,
    ) -> RangedNonZeroI128<MIN, MAX> {
        let range = const {
            if MIN == 0 || MAX == 0 {
                panic!("MIN or MAX cannot be zero for nonzero ints");
            }

            RangeInclusive::new(MIN, MAX)
        };

        RangedNonZeroI128(NonZero::new(if range.contains(&0) {
            let range = const { RangeInclusive::new(MIN + 1, MAX) };
            let value = self.i128(range);

            if value <= 0 { value - 1 } else { value }
        } else {
            self.i128(range::range_inclusive::<RangedI128<MIN, MAX>, i128>())
        }).unwrap())
    }
}
