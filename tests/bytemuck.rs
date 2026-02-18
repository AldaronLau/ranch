use bytemuck::checked;
use ranch::bitwise::U12;

#[test]
fn bytemuck() {
    let valid_slice: &[u16] = &[53, 200, 0, 512];
    let old_valid_slice = valid_slice;
    let invalid_slice: &[u16] = &[53, 200, 0, 5_012];

    let valid_slice: &[U12] = checked::try_cast_slice(valid_slice).unwrap();

    checked::try_cast_slice::<_, U12>(invalid_slice).unwrap_err();

    assert_eq!(old_valid_slice, bytemuck::cast_slice(valid_slice));
}
