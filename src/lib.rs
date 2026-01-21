use std::sync::atomic::{AtomicU64, Ordering};

pub struct AtomicBitMap {
    arena: Vec<AtomicU64>,
    pub x: usize
}

impl AtomicBitMap {
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            x: 0
        }
    }

    // Reader
    pub fn get(&self, idx: usize) -> Option<bool> { // check if but is set, lock free, no contention
        if !self.check_bounds(idx) {
            return None
        }

        let i = self.arena[get_bit(idx)].load(Ordering::Relaxed);
        Some((i & (1 << (get_int(idx)))) != 0) // note some langauges dont like you calling nonatomic operation on atomic types,
    }

    // Writers 
    pub fn set(&mut self, idx: usize) {
        self.capacity_regulator(idx);
        self.arena[get_bit(idx)].fetch_or(1 << (get_int(idx)), Ordering::Relaxed);
    }

    pub fn clear(&mut self, idx: usize) {
        self.capacity_regulator(idx);
        self.arena[get_bit(idx)].fetch_and(!(1 << (get_int(idx))), Ordering::Relaxed);
        self.reclamation(); // remove any empty integers
    }

    pub fn toggle(&mut self, idx: usize) { // be careful, this does not ensure you are toggling with any safeties, ideally just use clear and set instead
        self.capacity_regulator(idx);
        self.arena[get_bit(idx)].fetch_xor(1 << (get_int(idx)), Ordering::Relaxed);
        self.reclamation();
    }

    // Utilities
    fn capacity_regulator(&mut self, idx: usize) {
        let i = idx / 64; 
        if i >= self.x {
            // scale arena aka, append a new Au64
            self.arena.resize_with(i + 1, || AtomicU64::new(0));
            self.x = i + 1;
        }
    }

    fn check_bounds(&self, idx: usize) -> bool {
        idx < self.x * 64
    }

    pub fn reclamation(&mut self) { // reclaim free memory seqentially
        while let Some(l) = self.arena.last() {
            if l.load(Ordering::Relaxed) != 0 {
                break;
            }
            self.arena.pop();
        }
        self.x = self.arena.len();
    }

}

fn get_int(idx: usize) -> usize {
    idx % 64
}

fn get_bit(idx: usize) -> usize {
    idx / 64
}




/*
//use core::ops::*;
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




*/

