use core::ops::*;

pub struct BinBloom<N> {
    pub bitz: N
}

pub trait BitArena {
    type Repr;

    const BITZ: u32;


    fn set(&mut self, i: usize);
    fn clear(&mut self, i: usize);
    fn toggle(&mut self, i: usize);
    fn get(&self, i: usize) -> bool;
}


pub trait UnsignedInt:
    Copy
    + Default
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + From<u8>
    + PartialEq
    {
        const BITZ: u32;
    }


    impl UnsignedInt for u8 {
        const BITZ: u32 = u8::BITS; 
    }
    impl UnsignedInt for u16 {
        const BITZ: u32 = u16::BITS; 
    }
    impl UnsignedInt for u32 {
        const BITZ: u32 = u32::BITS; 
    }
    impl UnsignedInt for u64 {
        const BITZ: u32 = u64::BITS; 
    }
    impl UnsignedInt for u128 {
        const BITZ: u32 = u128::BITS; 
    }
    impl UnsignedInt for usize {
        const BITZ: u32 = usize::BITS; 
    }


    impl<N: UnsignedInt> BitArena for BinBloom<N> {
        type Repr = N;

        const BITZ: u32 = N::BITZ;
        
        #[inline]
        fn set(&mut self, i: usize) {
            debug_assert!(i < N::BITZ as usize);
            self.bitz = self.bitz | (N::from(1) << i as u32); // bitwise or: |, combining masks
        }

        #[inline]
        fn clear(&mut self, i: usize) {
            debug_assert!(i < N::BITZ as usize);
            self.bitz = self.bitz & !(N::from(1) << i as u32); // bitwise and not: inverted
        }

        #[inline]
        fn toggle(&mut self, i: usize) {
            debug_assert!(i < N::BITZ as usize);
            self.bitz = self.bitz ^ (N::from(1) << i as u32); // xor: ^ 
        }

        #[inline]
        fn get(&self, i: usize) -> bool {
            debug_assert!(i < N::BITZ as usize);
            (self.bitz & (N::from(1) << i as u32)) != N::default() // bitwise AND: &, Masking
        }

        
    }

