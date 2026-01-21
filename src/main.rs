use binbloom::AtomicBitMap;

fn main() {
    println!("Hello, world!");

    let mut bits = AtomicBitMap::new();

    bits.set(7);
    println!("Bit 7 set? {:?}", bits.get(7)); // Some(true)
    println!("Bit 4 set? {:?}", bits.get(4)); // Some(false)
    println!("Bit 1 set? {:?}", bits.get(1)); // Some(false)


    bits.clear(7);
    println!("Bit 7 set? {:?}", bits.get(7)); // Some(true)
    println!("Bit 4 set? {:?}", bits.get(4)); // Some(false)
    println!("Bit 1 set? {:?}", bits.get(1)); // Some(false)
    bits.set(7);


    println!("Len: {}", bits.x);

    bits.set(200);
    println!("Len: {}", bits.x);


    bits.clear(200);
    //bits.reclamation();
    println!("Len: {}", bits.x);


    bits.set(500);
    println!("Len: {}", bits.x);

    bits.clear(500);
    //bits.reclamation();
    println!("Len: {}", bits.x);

    println!("Bit 6000: {}", bits.get(6000));
    println!("Bit len: {}", bits.x);

    bits.set(10_000);
    println!("Bit 10_000: {}", bits.get(10_000));
    println!("Bit len: {}", bits.x);

    bits.toggle(10_000);
    println!("Bit len: {}", bits.x);


}

