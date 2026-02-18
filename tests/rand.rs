use ranch::*;
use rand::RngExt;

#[test]
fn rand() {
    let mut rng = rand::rng();

    let a: RangedU8<0, 100> = rng.random();
    let b: RangedNonZeroI32<-10, 10> = rng.random();
    let c: RangedI16<0, 100> = rng.random();
    let d: RangedNonZeroU64<1, 5> = rng.random();
    // Validate values in range
    let _a: RangedU8<0, 100> = RangedU8::with_u8(a).unwrap();
    let _b: RangedNonZeroI32<-10, 10> =
        RangedNonZeroI32::with_i32(b).unwrap().unwrap();
    let _c: RangedI16<0, 100> = RangedI16::with_i16(c).unwrap();
    let _d: RangedNonZeroU64<1, 5> =
        RangedNonZeroU64::with_u64(d).unwrap().unwrap();
}
