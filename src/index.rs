use core::ops::{Index, IndexMut};

use crate::*;

macro_rules! index {
    ($unsigned:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p, const N: usize, T>
            Index<$unsigned<MIN, MAX>> for [T; N]
        {
            type Output = T;

            // Required method
            fn index(&self, index: $unsigned<MIN, MAX>) -> &Self::Output {
                const {
                    if (MAX as usize) >= N {
                        panic!("Maximum not equal to array length - 1")
                    }
                }

                Index::index(self, index.get() as usize)
            }
        }

        impl<const MIN: $p, const MAX: $p, const N: usize, T>
            IndexMut<$unsigned<MIN, MAX>> for [T; N]
        {
            // Required method
            fn index_mut(
                &mut self,
                index: $unsigned<MIN, MAX>,
            ) -> &mut Self::Output {
                const {
                    if (MAX as usize) >= N {
                        panic!("Maximum not equal to array length - 1")
                    }
                }

                IndexMut::index_mut(self, index.get() as usize)
            }
        }
    };
}

index!(RangedU8, u8);
index!(RangedU16, u16);
index!(RangedU32, u32);
index!(RangedU64, u64);
index!(RangedU128, u128);
index!(RangedNonZeroU8, u8);
index!(RangedNonZeroU16, u16);
index!(RangedNonZeroU32, u32);
index!(RangedNonZeroU64, u64);
index!(RangedNonZeroU128, u128);
