use crate::bitboard::{BitBoard, BitBoardIter};
use crate::game::{Color, Piece};
use crate::squares::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    // White Pieces
    pub white_king  : BitBoard,
    pub white_queen : BitBoard,
    pub white_rook  : BitBoard,
    pub white_bishop: BitBoard,
    pub white_knight: BitBoard,
    pub white_pawn  : BitBoard,

    // Black Pieces
    pub black_king  : BitBoard,
    pub black_queen : BitBoard,
    pub black_rook  : BitBoard,
    pub black_bishop: BitBoard,
    pub black_knight: BitBoard,
    pub black_pawn  : BitBoard,
}

impl Board {
    pub fn empty() -> Self {
        Board {
            white_king  : 0u64,
            white_queen : 0u64,
            white_rook  : 0u64,
            white_bishop: 0u64,
            white_knight: 0u64,
            white_pawn  : 0u64,

            black_king  : 0u64,
            black_queen : 0u64,
            black_rook  : 0u64,
            black_bishop: 0u64,
            black_knight: 0u64,
            black_pawn  : 0u64,
        }
    }

    pub fn starting_position() -> Self {
        Board {
            white_king  : E1,
            white_queen : D1,
            white_rook  : A1 | H1,
            white_bishop: C1 | F1,
            white_knight: B1 | G1,
            white_pawn  : A2 | B2 | C2 | D2 | E2 | F2 | G2 | H2,

            black_king  : E8,
            black_queen : D8,
            black_rook  : A8 | H8,
            black_bishop: C8 | F8,
            black_knight: B8 | G8,
            black_pawn  : A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
        }
    }

    pub fn occupied(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_king
                | self.white_queen
                | self.white_rook
                | self.white_bishop
                | self.white_knight
                | self.white_pawn,
            Color::Black => self.black_king
                | self.black_queen
                | self.black_rook
                | self.black_bishop
                | self.black_knight
                | self.black_pawn,
        }
    }

    pub fn any_occupied(&self) -> BitBoard {
        self.occupied(Color::Black) | self.occupied(Color::White)
    }

    pub fn piece_board(&self, piece: Piece, color: Color) -> BitBoard {
        match (color, piece) {
            (Color::White, Piece::King)   => self.white_king,
            (Color::White, Piece::Queen)  => self.white_queen,
            (Color::White, Piece::Rook)   => self.white_rook,
            (Color::White, Piece::Bishop) => self.white_bishop,
            (Color::White, Piece::Knight) => self.white_knight,
            (Color::White, Piece::Pawn)   => self.white_pawn,
            (Color::Black, Piece::King)   => self.black_king,
            (Color::Black, Piece::Queen)  => self.black_queen,
            (Color::Black, Piece::Rook)   => self.black_rook,
            (Color::Black, Piece::Bishop) => self.black_bishop,
            (Color::Black, Piece::Knight) => self.black_knight,
            (Color::Black, Piece::Pawn)   => self.black_pawn,
        }
    }

    pub fn get_piece_positions(&self, piece: Piece, color: Color) -> BitBoardIter {
        get_bitmask_positions(self.piece_board(piece, color))
    }

    pub fn piece_board_mut(&mut self, piece: Piece, color: Color) -> &mut BitBoard {
        match (color, piece) {
            (Color::White, Piece::King)   => &mut self.white_king,
            (Color::White, Piece::Queen)  => &mut self.white_queen,
            (Color::White, Piece::Rook)   => &mut self.white_rook,
            (Color::White, Piece::Bishop) => &mut self.white_bishop,
            (Color::White, Piece::Knight) => &mut self.white_knight,
            (Color::White, Piece::Pawn)   => &mut self.white_pawn,
            (Color::Black, Piece::King)   => &mut self.black_king,
            (Color::Black, Piece::Queen)  => &mut self.black_queen,
            (Color::Black, Piece::Rook)   => &mut self.black_rook,
            (Color::Black, Piece::Bishop) => &mut self.black_bishop,
            (Color::Black, Piece::Knight) => &mut self.black_knight,
            (Color::Black, Piece::Pawn)   => &mut self.black_pawn,
        }
    }
}

pub fn get_bitmask_positions(bits: BitBoard) -> BitBoardIter {
    BitBoardIter { bits }
}

pub fn print_board(board: &Board) {
    for rank in (0..8).rev() {
        print!("{} ", rank+1);
        for file in 0..8 {
            match 1u64 << (8 * rank + file) {
                square if board.white_king   & square != 0 => print!("\u{2654} "),
                square if board.white_queen  & square != 0 => print!("\u{2655} "),
                square if board.white_rook   & square != 0 => print!("\u{2656} "),
                square if board.white_bishop & square != 0 => print!("\u{2657} "),
                square if board.white_knight & square != 0 => print!("\u{2658} "),
                square if board.white_pawn   & square != 0 => print!("\u{2659} "),

                square if board.black_king   & square != 0 => print!("\u{265A} "),
                square if board.black_queen  & square != 0 => print!("\u{265B} "),
                square if board.black_rook   & square != 0 => print!("\u{265C} "),
                square if board.black_bishop & square != 0 => print!("\u{265D} "),
                square if board.black_knight & square != 0 => print!("\u{265E} "),
                square if board.black_pawn   & square != 0 => print!("\u{265F} "),
                _ => print!(". "),
            }
        }
        println!();
    }
    println!("  A B C D E F G H");
}


