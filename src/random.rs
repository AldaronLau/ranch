use crate::*;

/// Random ranged int generation trait
///
/// Can be implemented on RNGs.
pub trait RngRanged {
    /// Generate a random [`RangedU8`].
    fn ranged_u8<const MIN: u8, const MAX: u8>(&mut self)
    -> RangedU8<MIN, MAX>;

    /// Generate a random [`RangedU16`].
    fn ranged_u16<const MIN: u16, const MAX: u16>(
        &mut self,
    ) -> RangedU16<MIN, MAX>;

    /// Generate a random [`RangedU32`].
    fn ranged_u32<const MIN: u32, const MAX: u32>(
        &mut self,
    ) -> RangedU32<MIN, MAX>;

    /// Generate a random [`RangedU64`].
    fn ranged_u64<const MIN: u64, const MAX: u64>(
        &mut self,
    ) -> RangedU64<MIN, MAX>;

    /// Generate a random [`RangedU128`].
    fn ranged_u128<const MIN: u128, const MAX: u128>(
        &mut self,
    ) -> RangedU128<MIN, MAX>;

    /// Generate a random [`RangedI8`].
    fn ranged_i8<const MIN: i8, const MAX: i8>(&mut self)
    -> RangedI8<MIN, MAX>;

    /// Generate a random [`RangedI16`].
    fn ranged_i16<const MIN: i16, const MAX: i16>(
        &mut self,
    ) -> RangedI16<MIN, MAX>;

    /// Generate a random [`RangedI32`].
    fn ranged_i32<const MIN: i32, const MAX: i32>(
        &mut self,
    ) -> RangedI32<MIN, MAX>;

    /// Generate a random [`RangedI64`].
    fn ranged_i64<const MIN: i64, const MAX: i64>(
        &mut self,
    ) -> RangedI64<MIN, MAX>;

    /// Generate a random [`RangedI128`].
    fn ranged_i128<const MIN: i128, const MAX: i128>(
        &mut self,
    ) -> RangedI128<MIN, MAX>;

    /// Generate a random [`RangedNonZeroU8`].
    fn ranged_nonzero_u8<const MIN: u8, const MAX: u8>(
        &mut self,
    ) -> RangedNonZeroU8<MIN, MAX> {
        RangedNonZeroU8::from_ranged(self.ranged_u8::<MIN, MAX>())
    }

    /// Generate a random [`RangedNonZeroU16`].
    fn ranged_nonzero_u16<const MIN: u16, const MAX: u16>(
        &mut self,
    ) -> RangedNonZeroU16<MIN, MAX> {
        RangedNonZeroU16::from_ranged(self.ranged_u16::<MIN, MAX>())
    }

    /// Generate a random [`RangedNonZeroU32`].
    fn ranged_nonzero_u32<const MIN: u32, const MAX: u32>(
        &mut self,
    ) -> RangedNonZeroU32<MIN, MAX> {
        RangedNonZeroU32::from_ranged(self.ranged_u32::<MIN, MAX>())
    }

    /// Generate a random [`RangedNonZeroU64`].
    fn ranged_nonzero_u64<const MIN: u64, const MAX: u64>(
        &mut self,
    ) -> RangedNonZeroU64<MIN, MAX> {
        RangedNonZeroU64::from_ranged(self.ranged_u64::<MIN, MAX>())
    }

    /// Generate a random [`RangedNonZeroU128`].
    fn ranged_nonzero_u128<const MIN: u128, const MAX: u128>(
        &mut self,
    ) -> RangedNonZeroU128<MIN, MAX> {
        RangedNonZeroU128::from_ranged(self.ranged_u128::<MIN, MAX>())
    }

    /// Generate a random [`RangedNonZeroI8`].
    fn ranged_nonzero_i8<const MIN: i8, const MAX: i8>(
        &mut self,
    ) -> RangedNonZeroI8<MIN, MAX>;

    /// Generate a random [`RangedNonZeroI16`].
    fn ranged_nonzero_i16<const MIN: i16, const MAX: i16>(
        &mut self,
    ) -> RangedNonZeroI16<MIN, MAX>;

    /// Generate a random [`RangedNonZeroI32`].
    fn ranged_nonzero_i32<const MIN: i32, const MAX: i32>(
        &mut self,
    ) -> RangedNonZeroI32<MIN, MAX>;

    /// Generate a random [`RangedNonZeroI64`].
    fn ranged_nonzero_i64<const MIN: i64, const MAX: i64>(
        &mut self,
    ) -> RangedNonZeroI64<MIN, MAX>;

    /// Generate a random [`RangedNonZeroI128`].
    fn ranged_nonzero_i128<const MIN: i128, const MAX: i128>(
        &mut self,
    ) -> RangedNonZeroI128<MIN, MAX>;
}
