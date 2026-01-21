use binbloom::{BinBloom, BitArena};

fn main() {
    println!("Hello, world!");


    let mut bloom = BinBloom::<u8> { bitz: 0 };

    bloom.set(7);

    println!("{:?}", bloom.get(7));
    println!("{:?}", bloom.get(4));
    println!("{:?}", bloom.get(1));



}


struct v {
    f: BinBloom::<u32>,
    k: i64
}