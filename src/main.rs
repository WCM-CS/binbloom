use std::sync::Arc;
use binbloom::AtomicBits;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use tokio::task;

/*
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let bits = Arc::new(AtomicBits::new());

    const NUM_READERS: usize = 5000;
    const NUM_READS_PER_READER: usize = 100;
    const NUM_WRITES: usize = 20_000;

    // Spawn single writer
    let writer_bits = bits.clone();
    let writer_handle = task::spawn(async move {
        for idx in 0..NUM_WRITES {
            writer_bits.set(idx);
            // simulate some small delay so readers interleave
            tokio::task::yield_now().await;
        }
    });

    // Spawn multiple readers
    let mut reader_handles = Vec::new();
    for _ in 0..NUM_READERS {
        let bits_clone = bits.clone();
        let handle = task::spawn(async move {
        let mut rng = SmallRng::from_entropy(); // each task gets its own RNG
        for _ in 0..NUM_READS_PER_READER {
            let idx = rng.gen_range(0..NUM_WRITES);
            let _ = bits_clone.read(idx);
            tokio::task::yield_now().await;
        }
    });
        reader_handles.push(handle);
    }

    // Wait for writer
    writer_handle.await.unwrap();

    // Wait for all readers
    for handle in reader_handles {
        handle.await.unwrap();
    }

    // Print some summary
    println!("Stress test complete!");
    println!("Bit 7: {}", bits.read(7));
    println!("Bit 200: {}", bits.read(200));
    println!("Bit 500: {}", bits.read(500));
    println!("Bit 10_000: {}", bits.read(10_000));
    println!("Current len: {}", bits.len());
}

*/




fn main() {
    println!("Hello, world!");

    let mut bits = AtomicBits::new();

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
    //bits.reclamation();
    println!("Len: {}", bits.len());

    println!("Bit 6000: {}", bits.read(6000));
    println!("Bit len: {}", bits.len());

    bits.set(10_000);
    println!("Bit 10_000: {}", bits.read(10_000));
    println!("Bit len: {}", bits.len());

    bits.clear(10_000);
    println!("Bit len: {}", bits.len());


}



