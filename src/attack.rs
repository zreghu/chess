pub mod generated_magics;

use crate::bitboard::BitBoard;
use crate::game::{Color, Square};
use crate::attack::generated_magics::{ROOK_MAGICS, ROOK_OFFSET, ROOK_OCC_TABLE, BISHOP_MAGICS, BISHOP_OFFSET, BISHOP_OCC_TABLE};

/*
(a1 = LSB) | square = 8 * rank + file

* 8 | 56 57 58 59 60 61 62 63
* 7 | 48 49 50 51 52 53 54 55
* 6 | 40 41 42 43 44 45 46 47
* 5 | 32 33 34 35 36 37 38 39
* 4 | 24 25 26 27 28 29 30 31
* 3 | 16 17 18 19 20 21 22 23
* 2 | 8  9  10 11 12 13 14 15
* 1 | 0  1  2  3  4  5  6  7
*     A  B  C  D  E  F  G  H
*/

pub fn king_attacks(sq: Square) -> BitBoard {
    KING_ATTACKS[sq]
}

pub fn queen_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    rook_attacks(sq, occ) | bishop_attacks(sq, occ)
}

pub fn rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    let mask = ROOK_OCCUPANCY_MASKS[sq];
    let magic = ROOK_MAGICS[sq];
    let offset = ROOK_OFFSET[sq];
    let shift = 64 - mask.count_ones();
    
    let idx = (((occ & mask).wrapping_mul(magic) >> shift) as usize) + offset;
    ROOK_OCC_TABLE[idx]
}

pub fn bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    let mask = BISHOP_OCCUPANCY_MASKS[sq];
    let magic = BISHOP_MAGICS[sq];
    let offset = BISHOP_OFFSET[sq];
    let shift = 64 - mask.count_ones();
    
    let idx = (((occ & mask).wrapping_mul(magic) >> shift) as usize) + offset;
    BISHOP_OCC_TABLE[idx]
}

pub fn knight_attacks(sq: Square) -> BitBoard {
    KNIGHT_ATTACKS[sq]
}

/// All squares attacked diagonally by a pawn on `sq`, whether occupied or empty.
pub fn pawn_attack_squares(sq: Square, color: Color) -> BitBoard {
    match color {
        Color::White => WHITE_PAWN_ATTACKS[sq],
        Color::Black => BLACK_PAWN_ATTACKS[sq],
    }
}

/// Pawn attacks that land on an occupied square, suitable for capture generation.
pub fn pawn_attacks(sq: Square, color: Color, occ: BitBoard) -> BitBoard {
    pawn_attack_squares(sq, color) & occ
}

pub fn pawn_moves(sq: Square, color: Color, occ:BitBoard) -> BitBoard {
    let (rank, file) = (sq / 8, sq % 8);

    if (rank == 0 && color == Color::Black) || (rank == 7 && color == Color::White) { return 0u64; };
    
    let new_sq = if color == Color::White { 8*(rank+1)+file } else { 8*(rank-1)+file};
    let pmove = 1u64 << new_sq;
    if pmove & occ != 0u64 { return 0u64 };

    let double_move = match (rank, color) {
        (1, Color::White) => 1u64 << (8 * (rank + 2) + file),
        (6, Color::Black) => 1u64 << (8 * (rank - 2) + file),
        _ => return pmove,
    };

    if (double_move & occ) != 0u64 { return pmove };

    pmove | double_move
}


// moves is an array of relative (rank, file)
const fn generate_attack_table(moves : &[(i32, i32)]) -> [BitBoard; 64] {
    let mut attack_table = [0u64; 64];
    let (mut rank, mut file) = (0, 0);
    while rank < 8 {
        while file < 8 {
            let mut km = 0;
            while km < moves.len() {
                let (i, j) = moves[km];
                let (y, x) = (rank + i, file + j);
                if x >= 0 && x < 8 && y >= 0 && y < 8 {
                    attack_table[(8 * rank + file) as usize] |= 1u64 << (8 * y + x);
                }
                km += 1;
            }
            file += 1;
        }
        file = 0;
        rank += 1;
    }
    attack_table
}

pub const fn king_attack_table() -> [BitBoard; 64] {
    let king_moves: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    generate_attack_table(&king_moves)
}

pub const fn knight_attack_table() -> [BitBoard; 64] {
    let knight_moves: [(i32, i32); 8] = [(-2, -1), (-2, 1), (2, -1), (2, 1), (-1, -2), (1, -2), (-1, 2), (1, 2)];
    generate_attack_table(&knight_moves)
}

pub const fn pawn_attack_table(color: Color) -> [BitBoard; 64] {
    let pawn_moves: [(i32, i32); 2] =
        match color {
            Color::White => [(1, 1), (1, -1)],
            Color::Black => [(-1, -1), (-1, 1)],
        };
    generate_attack_table(&pawn_moves)
}

// moves in this case is the relative transformation to apply
pub const fn generate_occupancy_mask_table(moves : &[(i32, i32)]) -> [BitBoard; 64] {
    let mut occupancy_mask_table = [0u64; 64];
    let (mut rank, mut file) = (0, 0);
    while rank < 8 {
        while file < 8 {
            let mut km = 0;
            while km < moves.len() {
                let (i, j) = moves[km];
                let (mut curr_r, mut curr_f) = (rank + i, file + j);
                while curr_r + i >= 0 && curr_r + i < 8 && curr_f + j >= 0 && curr_f + j < 8 {
                    occupancy_mask_table[(8 * rank + file) as usize] |= 1u64 << (8 * curr_r + curr_f);
                    curr_r += i;
                    curr_f += j;
                }
                km += 1;
            }
            file += 1;
        }
        file = 0;
        rank += 1;
    }
    occupancy_mask_table
}

pub const fn rook_occupancy_mask_table() -> [BitBoard; 64] {
    let rook_moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    generate_occupancy_mask_table(&rook_moves)
}

pub const fn bishop_occupancy_mask_table() -> [BitBoard; 64] {
    let bishop_moves = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    generate_occupancy_mask_table(&bishop_moves)
}

// The builders above are const functions; these declarations evaluate them
// once at compile time so lookups don't rebuild a 64-entry table at runtime.
static KING_ATTACKS: [BitBoard; 64] = king_attack_table();
static KNIGHT_ATTACKS: [BitBoard; 64] = knight_attack_table();
static WHITE_PAWN_ATTACKS: [BitBoard; 64] = pawn_attack_table(Color::White);
static BLACK_PAWN_ATTACKS: [BitBoard; 64] = pawn_attack_table(Color::Black);
static ROOK_OCCUPANCY_MASKS: [BitBoard; 64] = rook_occupancy_mask_table();
static BISHOP_OCCUPANCY_MASKS: [BitBoard; 64] = bishop_occupancy_mask_table();


