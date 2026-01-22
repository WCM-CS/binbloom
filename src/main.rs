use std::sync::Arc;
use binbloom::AtomicBits;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use tokio::task;



#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    println!("Hello, world!");

    let bits = AtomicBits::new();

    bits.set(7).await;
    println!("Bit 7 set? {:?}", bits.read(7)); // Some(true)
    println!("Bit 4 set? {:?}", bits.read(4)); // Some(false)
    println!("Bit 1 set? {:?}", bits.read(1)); // Some(false)


    bits.clear(7).await;
    println!("Bit 7 set? {:?}", bits.read(7)); // Some(true)
    println!("Bit 4 set? {:?}", bits.read(4)); // Some(false)
    println!("Bit 1 set? {:?}", bits.read(1)); // Some(false)
    bits.set(7).await;


    println!("Len: {}", bits.len());

    bits.set(200).await;
    println!("Len: {}", bits.len());


    bits.clear(200).await;
    //bits.reclamation();
    println!("Len: {}", bits.len());


    bits.set(500).await;
    println!("Len: {}", bits.len());

    bits.clear(500).await;
    //bits.reclamation();
    println!("Len: {}", bits.len());

    println!("Bit 6000: {}", bits.read(6000));
    println!("Bit len: {}", bits.len());

    bits.set(10_000).await;
    println!("Bit 10_000: {}", bits.read(10_000));
    println!("Bit len: {}", bits.len());

    bits.clear(10_000).await;
    println!("Bit len: {}", bits.len());

    for idx in 0..100_000 {
        bits.set(idx).await;
    }

    for idx in 0..100_000 {
        bits.clear(idx).await;
    }

    println!("Bits opped")


}




