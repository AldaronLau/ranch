use core::num::NonZero;

use ranch::*;

#[test]
fn casting() {
    as_repr::as_repr::<i32>(RangedI32::<0, 100>::new::<50>());
    as_repr::as_repr::<Option<NonZero<i32>>>(RangedI32::<0, 100>::new::<50>());
}
