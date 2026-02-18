use fastrand::Rng;
use ranch::{range::RngRanged, *};
use zeroize::Zeroize;

#[test]
fn zeroize() {
    let mut rng = Rng::with_seed(0x4d595df4d0f33173);

    let mut a: RangedU8<0, 100> = rng.ranged_u8();
    let mut b: RangedNonZeroI32<-10, 10> = rng.ranged_nonzero_i32();
    let mut c: RangedI16<0, 100> = rng.ranged_i16();
    let mut d: RangedNonZeroU64<1, 5> = rng.ranged_nonzero_u64();

    a.zeroize();
    b.zeroize();
    c.zeroize();
    d.zeroize();

    // Validate values in range
    let a: RangedU8<0, 100> = RangedU8::with_u8(a).unwrap();
    let b: RangedNonZeroI32<-10, 10> =
        RangedNonZeroI32::with_i32(b).unwrap().unwrap();
    let c: RangedI16<0, 100> = RangedI16::with_i16(c).unwrap();
    let d: RangedNonZeroU64<1, 5> =
        RangedNonZeroU64::with_u64(d).unwrap().unwrap();

    insta::assert_json_snapshot!((a, b, c, d));
}
