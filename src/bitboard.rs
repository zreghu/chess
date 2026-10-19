pub type BitBoard = u64;

pub fn print_bitboard(bitboard: BitBoard) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let occupied = bitboard & (1u64 << (8 * rank + file)) != 0;
            print!("{} ", if occupied { '1' } else { '0' });
        }
        println!();
    }
    println!("  A B C D E F G H");
}
