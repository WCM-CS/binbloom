use std::sync::atomic::{AtomicU64, Ordering};

pub struct AtomicBitMap {
    arena: Vec<AtomicU64>,
    pub u64_count: usize
}

/* // if the bitmap were used more heavily and scaling in size frequently this would be better
    arena: &'a [AtomicU64] // Overallocate initially, built once, reset occasionally, use a slice, backed by a custom bump allocator,

*/

impl AtomicBitMap {
    pub fn new() -> Self { Self { arena: Vec::new(), u64_count: 0 } }

    // Reader
    pub fn get(&self, idx: usize) -> bool { // check if but is set, lock free, no contention
        if let Some(i) = self.arena.get(get_index(idx)) { // dont raw index here, cant ensure bounds, get returns an option
            (i.load(Ordering::Relaxed) & (1 << (get_offset(idx)))) != 0  // note some langauges dont like you calling nonatomic operation on atomic types,
        } else {
            false
        }
    }

    // Writers 
    pub fn set(&mut self, idx: usize) {
        self.capacity_regulator(idx);
        self.arena[get_index(idx)].fetch_or(1 << (get_offset(idx)), Ordering::Relaxed);
    }

    pub fn clear(&mut self, idx: usize) {
        self.capacity_regulator(idx);
        self.arena[get_index(idx)].fetch_and(!(1 << (get_offset(idx))), Ordering::Relaxed);
        self.reclamation(); // remove any empty integers
    }

    pub fn toggle(&mut self, idx: usize) { // be careful, this does not ensure you are toggling with any safeties, ideally just use clear and set instead
        self.capacity_regulator(idx);
        self.arena[get_index(idx)].fetch_xor(1 << (get_offset(idx)), Ordering::Relaxed);
        self.reclamation();
    }

    // Utilities
    fn capacity_regulator(&mut self, idx: usize) {
        let i = idx / 64; 
        if i >= self.u64_count {
            // scale arena aka, append a new Au64
            self.arena.resize_with(i + 1, || AtomicU64::new(0));
            self.u64_count = i + 1;
        }
    }

    fn _check_bounds(&self, idx: usize) -> bool {
        idx < self.u64_count * 64  // buggy if valid range is unititialzied and accessed, will cause Undefined behavior, should return false but can return None instead
    }

    pub fn reclamation(&mut self) { // reclaim free memory seqentially
        while let Some(l) = self.arena.last() {
            if l.load(Ordering::Relaxed) != 0 {
                break;
            }
            self.arena.pop();
        }
        self.u64_count = self.arena.len();
    }

}

fn get_index(idx: usize) -> usize {
    idx / 64
}

fn get_offset(idx: usize) -> usize {
    idx % 64
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

