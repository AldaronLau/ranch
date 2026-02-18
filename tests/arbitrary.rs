use arbitrary::Unstructured;
use fastrand::Rng;
use ranch::*;

#[test]
fn arbitrary() {
    let mut rng = Rng::with_seed(0x4d595df4d0f33173);
    let mut buf = Vec::from_iter([b'\0'; 128]);

    rng.fill(&mut buf);

    let mut unstructured = Unstructured::new(buf.as_slice());
    let a: RangedU8<0, 100> = unstructured.arbitrary().unwrap();
    let b: RangedNonZeroI32<-10, 10> = unstructured.arbitrary().unwrap();
    let c: RangedI16<0, 100> = unstructured.arbitrary().unwrap();
    let d: RangedNonZeroU64<1, 5> = unstructured.arbitrary().unwrap();
    // Validate values in range
    let _a: RangedU8<0, 100> = RangedU8::with_u8(a).unwrap();
    let _b: RangedNonZeroI32<-10, 10> =
        RangedNonZeroI32::with_i32(b).unwrap().unwrap();
    let _c: RangedI16<0, 100> = RangedI16::with_i16(c).unwrap();
    let _d: RangedNonZeroU64<1, 5> =
        RangedNonZeroU64::with_u64(d).unwrap().unwrap();
}
