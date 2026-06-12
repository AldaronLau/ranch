//! Traits for working with different kinds of values
//!
//! Ranch defines a [`Value`] as a type with a minimum and a maximum.  As of
//! right now this includes two classes:
//!
//!  - Numbers (ints and floats)
//!  - [`Duration`]s

use core::{marker::PhantomData, range::RangeInclusive, time::Duration};

use as_repr::{cmp::Cmp, num::Number};

/// A type with an minimum and maximum value
pub trait Value:
    Sized + Copy + PartialOrd + Cmp + val::Sealed + 'static
{
    /// The smallest value that can be represented by this type
    const MIN: Self;
    /// The largest value that can be represented by this type
    const MAX: Self;
}

impl Value for Duration {
    const MAX: Self = Duration::MAX;
    const MIN: Self = Duration::ZERO;
}

impl<T> Value for T
where
    T: Sized + Copy + PartialOrd + Cmp + num::Num + 'static,
{
    const MAX: Self = <Self as Number>::REPR_MAX;
    const MIN: Self = <Self as Number>::REPR_MIN;
}

/// A value that cannot be constructed
pub trait Uninhabited<T> {}

impl<T, V> Uninhabited<T> for V
where
    V: Validate<T, ContiguousSets = sets::Sets<0>>,
    T: Value,
{
}

/// A type with a contiguous set of valid values
pub trait Contiguous<T> {}

impl<T, V> Contiguous<T> for V
where
    V: Validate<T, ContiguousSets = sets::Sets<1>>,
    T: Value,
{
}

/// A type with a list of valid inclusive ranges
pub trait Validate<T>: Sized
where
    T: Value,
{
    /// The number of contiguous sets
    type ContiguousSets: sets::ContiguousSets;

    /// List of inclusive ranges that are valid
    const VALID_FOR: &'static [RangeInclusive<T>];
    /// Proof that the number of contiguous sets matches the number of valid
    /// inclusive ranges
    const PROOF: proof::Proof<Self, T>;
}

mod proof {
    use super::*;

    #[derive(Debug)]
    pub struct Proof<T, V>(PhantomData<fn() -> (T, V)>);

    impl<T, V> Proof<T, V>
    where
        T: Validate<V>,
        V: Value,
    {
        pub const fn new() -> Self {
            if <T::ContiguousSets as sets::ContiguousSets>::COUNT
                != T::VALID_FOR.len()
            {
                panic!(
                    "Length of VALID_FOR must match the number of contiguous sets"
                );
            }

            Proof(PhantomData)
        }
    }
}

mod sets {
    /// Number of contiguous sets
    #[derive(Debug)]
    pub struct Sets<const N: usize>;

    /// A type that defines the number of contiguous sets
    pub trait ContiguousSets {
        /// The number of contiguous sets
        const COUNT: usize;
    }
}

mod num {
    use super::*;

    /// A numeric type
    pub trait Num: Number<ToRepr = Self> {}

    impl Num for f32 {}
    impl Num for f64 {}
    impl Num for i8 {}
    impl Num for i16 {}
    impl Num for i32 {}
    impl Num for i64 {}
    impl Num for i128 {}
    impl Num for u8 {}
    impl Num for u16 {}
    impl Num for u32 {}
    impl Num for u64 {}
    impl Num for u128 {}
}

mod val {
    use super::*;

    pub trait Sealed {}

    impl Sealed for Duration {}
    impl<T> Sealed for T where T: num::Num {}
}
