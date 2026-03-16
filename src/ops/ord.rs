use core::cmp::Ordering;

use as_repr::AsRepr;

use crate::{
    multirange::{MultiRange, Ranged},
    num::rangeable_primitive::RangeablePrimitive,
    range::Range,
};

impl<P, R> Ord for Ranged<P, R>
where
    P: RangeablePrimitive,
    R: MultiRange<P::ZeroablePrimitive>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let this: P = as_repr::as_repr(*self);
        let other: P = as_repr::as_repr(*other);

        this.cmp(&other)
    }
}

impl<P, R, T> PartialOrd<T> for Ranged<P, R>
where
    P: RangeablePrimitive,
    R: MultiRange<P::ZeroablePrimitive>,
    T: AsRepr<P> + Copy + Clone,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let this: P = as_repr::as_repr(*self);
        let other: P = as_repr::as_repr(*other);

        this.partial_cmp(&other)
    }
}

impl<P, R> Eq for Ranged<P, R>
where
    P: RangeablePrimitive,
    R: MultiRange<P::ZeroablePrimitive>,
{
}

impl<P, R, T> PartialEq<T> for Ranged<P, R>
where
    P: RangeablePrimitive,
    R: MultiRange<P::ZeroablePrimitive>,
    T: AsRepr<P> + Copy + Clone,
{
    fn eq(&self, other: &T) -> bool {
        let this: P = as_repr::as_repr(*self);
        let other: P = as_repr::as_repr(*other);

        this.eq(&other)
    }
}
