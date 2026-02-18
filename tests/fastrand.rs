use fastrand::Rng;
use ranch::{range::RngRanged, *};

#[test]
fn fastrand() {
    let mut rng = Rng::with_seed(0x4d595df4d0f33173);

    let a: RangedU8<0, 100> = rng.ranged_u8();
    let b: RangedNonZeroI32<-10, 10> = rng.ranged_nonzero_i32();
    let c: RangedI16<0, 100> = rng.ranged_i16();
    let d: RangedNonZeroU64<1, 5> = rng.ranged_nonzero_u64();
    // Validate values in range
    let _a: RangedU8<0, 100> = RangedU8::with_u8(a).unwrap();
    let _b: RangedNonZeroI32<-10, 10> =
        RangedNonZeroI32::with_i32(b).unwrap().unwrap();
    let _c: RangedI16<0, 100> = RangedI16::with_i16(c).unwrap();
    let _d: RangedNonZeroU64<1, 5> =
        RangedNonZeroU64::with_u64(d).unwrap().unwrap();
}
