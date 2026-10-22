use crate::game::Square;

pub type BitBoard = u64;

#[derive(Clone)]
pub struct BitBoardIter {
    pub bits: BitBoard,
}

impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            return None;
        }

        let square = self.bits.trailing_zeros() as Square;
        self.bits &= self.bits - 1;
        Some(square)
    }
}

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
