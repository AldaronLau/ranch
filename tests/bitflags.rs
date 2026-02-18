use bitflags::{Flag, Flags};
use ranch::bitwise::U3;

/// Represents a set of flags.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CustomFlags(U3);

impl Flags for CustomFlags {
    type Bits = U3;

    const FLAGS: &'static [Flag<Self>] = &[
        Flag::new("A", Self::A),
        Flag::new("B", Self::B),
        Flag::new("C", Self::C),
    ];

    fn bits(&self) -> Self::Bits {
        self.0
    }

    fn from_bits_retain(bits: Self::Bits) -> Self {
        Self(bits)
    }
}

impl CustomFlags {
    /// The value `A`, at bit position `0`.
    pub const A: Self = Self(U3::new::<0b001>());
    /// The value `B`, at bit position `1`.
    pub const B: Self = Self(U3::new::<0b010>());
    /// The value `C`, at bit position `2`.
    pub const C: Self = Self(U3::new::<0b100>());
}

impl CustomFlags {
    /// The combination of `A`, `B`, and `C`.
    pub const ABC: Self =
        Self(Self::A.0.bitor_ranged(Self::B.0).bitor_ranged(Self::C.0));
}

#[test]
fn bitflags() {
    let e1 = CustomFlags::A.union(CustomFlags::C);
    let e2 = CustomFlags::B.union(CustomFlags::C);

    assert_eq!(e1.union(e2), CustomFlags::ABC);
    assert_eq!(e1.intersection(e2), CustomFlags::C);
    assert_eq!(e1.difference(e2), CustomFlags::A);
    assert_eq!(e2.complement(), CustomFlags::A);
}
