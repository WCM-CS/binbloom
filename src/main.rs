use binbloom::asyncronous::AtomicBits;
use binbloom::syncronous::AtomicBits as sbits;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use tokio::task;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    println!("Hello, world!");

    let start = std::time::Instant::now();

    let bits = sbits::new();

    bits.set(7);
    println!("Bit 7 set? {:?}", bits.read(7)); // Some(true)
    println!("Bit 4 set? {:?}", bits.read(4)); // Some(false)
    println!("Bit 1 set? {:?}", bits.read(1)); // Some(false)

    bits.clear(7);
    println!("Bit 7 set? {:?}", bits.read(7)); // Some(true)
    println!("Bit 4 set? {:?}", bits.read(4)); // Some(false)
    println!("Bit 1 set? {:?}", bits.read(1)); // Some(false)
    bits.set(7);

    println!("Len: {}", bits.len());

    bits.set(200);
    println!("Len: {}", bits.len());

    bits.clear(200);
    //bits.reclamation();
    println!("Len: {}", bits.len());

    bits.set(500);
    println!("Len: {}", bits.len());

    bits.clear(500);
    //bits.reclamation(
    println!("Len: {}", bits.len());

    println!("Bit 6000: {}", bits.read(6000));
    println!("Bit len: {}", bits.len());

    bits.set(10_000);
    println!("Bit 10_000: {}", bits.read(10_000));
    println!("Bit len: {}", bits.len());

    bits.clear(10_000);
    println!("Bit len: {}", bits.len());

    for idx in 0..1_000_000 {
        bits.set(idx);
    }

    for idx in 0..1_000_000 {
        bits.clear(idx);
    }

    let elapsed = std::time::Instant::elapsed(&start);

    println!("Bits opped: {:?}", elapsed)
}
