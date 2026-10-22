use crate::board::*;
use crate::bitboard::BitBoard;
use crate::attack::*;
use crate::squares::*;
use crate::fen::chess_to_fen;

pub type Square = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::King,
        Piece::Queen,
        Piece::Rook,
        Piece::Bishop,
        Piece::Knight,
        Piece::Pawn,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Normal { piece: Piece, from: Square, to: Square },
    Castle { side: CastleSide },
    Promotion { from: Square, to: Square, promotion: Piece },
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingAvailability {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}


impl Default for CastlingAvailability {
    fn default() -> Self {
        CastlingAvailability {
            white_king_side:  true,
            white_queen_side: true,
            black_king_side:  true,
            black_queen_side: true,
        }
    }
}

impl CastlingAvailability {
    pub fn empty() -> Self {
        CastlingAvailability {
            white_king_side:  false,
            white_queen_side: false,
            black_king_side:  false,
            black_queen_side: false,
        }
    }

    pub fn available(&self, side: CastleSide, color: Color) -> bool {
        match (side, color) {
            (CastleSide::KingSide,  Color::White) => self.white_king_side,
            (CastleSide::QueenSide, Color::White) => self.white_queen_side,
            (CastleSide::KingSide,  Color::Black) => self.black_king_side,
            (CastleSide::QueenSide, Color::Black) => self.black_queen_side,
        }
    }

    pub fn available_mut(&mut self, side: CastleSide, color: Color) -> &mut bool {
        match (side, color) {
            (CastleSide::KingSide,  Color::White) => &mut self.white_king_side,
            (CastleSide::QueenSide, Color::White) => &mut self.white_queen_side,
            (CastleSide::KingSide,  Color::Black) => &mut self.black_king_side,
            (CastleSide::QueenSide, Color::Black) => &mut self.black_queen_side,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UndoState {
    capture_piece: Option<Piece>,
    en_passant: Option<Square>,
    half_move_number: u16,
    castling_availability: CastlingAvailability, 
}


// Based on FEN
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chess {
    pub board: Board,
    pub en_passant: Option<Square>,
    pub turn: Color,
    pub castling_availability: CastlingAvailability,
    pub half_move_number: u16,
    pub full_move_number: u16,
}

impl Default for Chess {
    fn default() -> Self {
        Chess {
            board: Board::starting_position(),
            en_passant: None,
            turn: Color::White,
            castling_availability: CastlingAvailability::default(),
            half_move_number: 0,
            full_move_number: 1,
        }
    }
}

fn piece_legal_move_bitboard(state: &Chess, piece: Piece, sq: Square, color: Color) -> BitBoard {
    let occ = state.board.any_occupied();
    !state.board.occupied(color) 
        & !state.board.piece_board(Piece::King, opposite_color(color))
        & match piece {
            Piece::King => king_attacks(sq),
            Piece::Queen => queen_attacks(sq, occ),
            Piece::Rook => rook_attacks(sq, occ),
            Piece::Bishop => bishop_attacks(sq, occ),
            Piece::Knight => knight_attacks(sq),
            Piece::Pawn => pawn_attacks(sq, color, occ | state.en_passant.map_or(0, |s| 1u64 << s)) 
                | pawn_moves(sq, color, occ),
        }
}

fn piece_legal_attack_bitboard(state: &Chess, piece: Piece, sq: Square, color: Color) -> BitBoard {
    let occ = state.board.any_occupied();
    match piece {
        Piece::King   => king_attacks(sq),
        Piece::Queen  => queen_attacks(sq, occ),
        Piece::Rook   => rook_attacks(sq, occ),
        Piece::Bishop => bishop_attacks(sq, occ),
        Piece::Knight => knight_attacks(sq),
        Piece::Pawn   => pawn_attack_squares(sq, color),
    }
}

fn sq_attacks(state: &Chess, sq: Square, color: Color) -> BitBoard {
    let occ = state.board.any_occupied();
    let rook_rays = rook_attacks(sq, occ);
    let bishop_rays = bishop_attacks(sq, occ);
    (pawn_attack_squares(sq, opposite_color(color)) & state.board.piece_board(Piece::Pawn, color))
        | (king_attacks(sq)   & state.board.piece_board(Piece::King, color))
        | ((rook_rays | bishop_rays) & state.board.piece_board(Piece::Queen, color))
        | (rook_rays & state.board.piece_board(Piece::Rook, color))
        | (bishop_rays & state.board.piece_board(Piece::Bishop, color))
        | (knight_attacks(sq) & state.board.piece_board(Piece::Knight, color))
}

fn all_attacks(state: &Chess, color: Color) -> BitBoard {
    let mut attacks = 0u64;
    for piece in Piece::ALL {
        

        for sq in state.board.get_piece_positions(piece, color) {
            attacks |= piece_legal_attack_bitboard(state, piece, sq, color);
        }
    }
    attacks
}

fn king_starting_pos(color: Color) -> BitBoard {
    match color {
        Color::White => E1,
        Color::Black => E8,
    }
}

fn castle_rook_pos(side: CastleSide, color: Color) -> BitBoard {
    match (side, color) {
        (CastleSide::KingSide, Color::White) => H1,
        (CastleSide::KingSide, Color::Black) => H8,
        (CastleSide::QueenSide, Color::White) => A1,
        (CastleSide::QueenSide, Color::Black) => A8,
    }
}

fn castle_occupancy_mask(side: CastleSide, color: Color) -> BitBoard {
    match (side, color) {
        (CastleSide::KingSide, Color::White) => F1 | G1,
        (CastleSide::KingSide, Color::Black) => F8 | G8,
        (CastleSide::QueenSide, Color::White) => B1 | C1 | D1,
        (CastleSide::QueenSide, Color::Black) => B8 | C8 | D8,
    }
}

fn castle_king_path_mask(side: CastleSide, color: Color) -> BitBoard {
    match (side, color) {
        (CastleSide::KingSide, Color::White) => E1 | F1 | G1,
        (CastleSide::KingSide, Color::Black) => E8 | F8 | G8,
        (CastleSide::QueenSide, Color::White) => E1 | D1 | C1,
        (CastleSide::QueenSide, Color::Black) => E8 | D8 | C8,
    }
}

fn castle_king_end(side: CastleSide, color: Color) -> BitBoard {
    match (side, color) {
        (CastleSide::KingSide, Color::White) => G1,
        (CastleSide::KingSide, Color::Black) => G8,
        (CastleSide::QueenSide, Color::White) => C1,
        (CastleSide::QueenSide, Color::Black) => C8,
    }
}

fn castle_rook_end(side: CastleSide, color: Color) -> BitBoard {
    match (side, color) {
        (CastleSide::KingSide, Color::White) => F1,
        (CastleSide::KingSide, Color::Black) => F8,
        (CastleSide::QueenSide, Color::White) => D1,
        (CastleSide::QueenSide, Color::Black) => D8,
    }
}

// Opposing en_passant pawn from en_passant square
const fn en_passant_pawn(sq: Square, color: Color) -> Square { 
    let (rank, file) = (sq / 8, sq % 8);
    match color {
        Color::White => 8 * (rank + 1) + file,
        Color::Black => 8 * (rank - 1) + file,
    }
}

// en_passant square that is calculated form pawn move
const fn en_passant_square(sq: Square, color: Color) -> Square { 
    let (rank, file) = (sq / 8, sq % 8);
    match color {
        Color::White => 8 * (rank - 1) + file,
        Color::Black => 8 * (rank + 1) + file,
    }
}

fn opposite_color(color: Color) -> Color {
    if color == Color::White {
        return Color::Black;
    }
    Color::White
}

fn captured_piece_at(state: &Chess, square: Square) -> Option<Piece> {
    let opp = opposite_color(state.turn);
    Piece::ALL.into_iter().find(|&piece| (1u64 << square) & state.board.piece_board(piece, opp) != 0)
}

fn is_castle_valid(state: &Chess, side: CastleSide) -> bool {
    if !state.castling_availability.available(side, state.turn) { return false; }
    if king_starting_pos(state.turn) & state.board.piece_board(Piece::King, state.turn) == 0 { return false; }
    if castle_rook_pos(side, state.turn) & state.board.piece_board(Piece::Rook, state.turn) == 0 { return false; }
    if castle_occupancy_mask(side, state.turn) & state.board.any_occupied() != 0 { return false; }
    if castle_king_path_mask(side, state.turn) & all_attacks(state, opposite_color(state.turn)) != 0 { return false; }
    true
}

fn castle_unsafe(state: &mut Chess, side: CastleSide) {
    *state.board.piece_board_mut(Piece::King, state.turn) = castle_king_end(side, state.turn);
    *state.board.piece_board_mut(Piece::Rook, state.turn) ^= castle_rook_pos(side, state.turn) | castle_rook_end(side, state.turn);
}

fn uncastle_unsafe(state: &mut Chess, side: CastleSide) {
    *state.board.piece_board_mut(Piece::King, state.turn) = king_starting_pos(state.turn);
    *state.board.piece_board_mut(Piece::Rook, state.turn) ^= castle_rook_pos(side, state.turn) | castle_rook_end(side, state.turn);
}

fn is_pseudo_legal(state: &Chess, mv: Move) -> bool {
    match mv {
        Move::Castle { side } => is_castle_valid(state, side),
        Move::Promotion { from, to, promotion } => {
            if from >= 64 || to >= 64 { return false; }
            if 1u64 << from & state.board.piece_board(Piece::Pawn, state.turn) == 0 { return false; }
            if (to / 8) != (if state.turn == Color::White { 7 } else { 0 }) { return false; }
            if (to / 8).abs_diff(from / 8) != 1 { return false; }
            if (1u64 << to) & piece_legal_move_bitboard(state, Piece::Pawn, from, state.turn) == 0 { return false; }
            if promotion != Piece::Queen && promotion != Piece::Rook && promotion != Piece::Bishop && promotion != Piece::Knight { return false; }
            true
        },
        Move::Normal { piece, from, to } => {
            if from >= 64 || to >= 64 { return false; }
            if piece == Piece::Pawn && (to / 8) == (if state.turn == Color::White { 7 } else { 0 }) { return false; }
            if (1u64 << from) & state.board.piece_board(piece, state.turn) == 0 { return false; }
            if (1u64 << to) & state.board.occupied(state.turn) != 0 { return false; }
            if (1u64 << to) & piece_legal_move_bitboard(state, piece, from, state.turn) == 0 { return false; } 
            if (1u64 << to) & state.board.piece_board(Piece::King, opposite_color(state.turn)) != 0 { return false; }
            true
       },
    }
}

pub fn make_move_unsafe(state: &mut Chess, mv: Move) -> UndoState {
    let mut undo_state = UndoState { 
        capture_piece: None, 
        en_passant : state.en_passant,
        half_move_number : state.half_move_number,
        castling_availability : state.castling_availability,
    };

    let mut new_en_passant = None;
    let mut pawn_move_or_capture = false;

    match mv {
        Move::Castle { side } => {
            castle_unsafe(state, side);
            *state.castling_availability.available_mut(CastleSide::KingSide, state.turn) = false;
            *state.castling_availability.available_mut(CastleSide::QueenSide, state.turn) = false;
        },
        Move::Promotion { from, to, promotion } => {
            *state.board.piece_board_mut(Piece::Pawn, state.turn) ^= 1u64 << from;
            *state.board.piece_board_mut(promotion, state.turn) |= 1u64 << to;

            let opp = opposite_color(state.turn);
            let capture_piece = captured_piece_at(state, to);
            if let Some(cp) = capture_piece {                             // remove captured piece
                *state.board.piece_board_mut(cp, opp) ^= 1u64 << to;
                undo_state.capture_piece = capture_piece;
            }

            if Some(Piece::Rook) == capture_piece {
                if 1u64 << to == castle_rook_pos(CastleSide::KingSide, opp) {
                    *state.castling_availability.available_mut(CastleSide::KingSide, opp) = false;
                } else if 1u64 << to == castle_rook_pos(CastleSide::QueenSide, opp) {
                    *state.castling_availability.available_mut(CastleSide::QueenSide, opp) = false;
                }
            }

            pawn_move_or_capture = true;
        },
        Move::Normal { piece, from, to } => {
            let capture_piece = captured_piece_at(state, to);
            if capture_piece.is_some() || piece == Piece::Pawn { pawn_move_or_capture = true };

            *state.board.piece_board_mut(piece, state.turn) ^= 1u64 << from | 1u64 << to;

            let opp = opposite_color(state.turn);
            if piece == Piece::Pawn && Some(to) == state.en_passant {            // en passant
                *state.board.piece_board_mut(Piece::Pawn, opp) ^= 1u64 << en_passant_pawn(to, opp);
            } else if piece == Piece::Pawn && (from / 8).abs_diff(to / 8) == 2 { // set en passant
                new_en_passant = Some(en_passant_square(to, state.turn));
            } else if let Some(cp) = capture_piece {                             // remove captured piece
                *state.board.piece_board_mut(cp, opp) ^= 1u64 << to;
                undo_state.capture_piece = capture_piece;
            }

            if piece == Piece::King {
                *state.castling_availability.available_mut(CastleSide::KingSide, state.turn) = false;
                *state.castling_availability.available_mut(CastleSide::QueenSide, state.turn) = false;
            }

            if piece == Piece::Rook {
                if 1u64 << from == castle_rook_pos(CastleSide::KingSide, state.turn) {
                    *state.castling_availability.available_mut(CastleSide::KingSide, state.turn) = false;
                } else if 1u64 << from == castle_rook_pos(CastleSide::QueenSide, state.turn) {
                    *state.castling_availability.available_mut(CastleSide::QueenSide, state.turn) = false;
                }
            }

            if Some(Piece::Rook) == capture_piece {
                if 1u64 << to == castle_rook_pos(CastleSide::KingSide, opp) {
                    *state.castling_availability.available_mut(CastleSide::KingSide, opp) = false;
                } else if 1u64 << to == castle_rook_pos(CastleSide::QueenSide, opp) {
                    *state.castling_availability.available_mut(CastleSide::QueenSide, opp) = false;
                }
            }
        },
    };

    state.en_passant = new_en_passant;

    if pawn_move_or_capture {
        state.half_move_number = 0;
    } else {
        state.half_move_number += 1;
    }

    if state.turn == Color::Black { state.full_move_number += 1 };
    state.turn = opposite_color(state.turn);

    undo_state
}

pub fn undo_move(state: &mut Chess, mv: Move, undo: UndoState) {
    state.castling_availability = undo.castling_availability;
    state.en_passant = undo.en_passant;
    state.half_move_number = undo.half_move_number;
    if state.turn == Color::White { state.full_move_number -= 1 };
    state.turn = opposite_color(state.turn);

    match mv {
        Move::Castle { side } => uncastle_unsafe(state, side),
        Move::Promotion { from, to, promotion } => {
            *state.board.piece_board_mut(Piece::Pawn, state.turn) ^= 1u64 << from;
            *state.board.piece_board_mut(promotion, state.turn) ^= 1u64 << to;

            let opp = opposite_color(state.turn);
            if let Some(cp) = undo.capture_piece {                             // remove captured piece
                *state.board.piece_board_mut(cp, opp) ^= 1u64 << to;
            }
        },
        Move::Normal { piece, from, to } => {
            *state.board.piece_board_mut(piece, state.turn) ^= 1u64 << from | 1u64 << to;

            let opp = opposite_color(state.turn);
            if piece == Piece::Pawn && Some(to) == undo.en_passant {            // en passant
                *state.board.piece_board_mut(Piece::Pawn, opp) ^= 1u64 << en_passant_pawn(to, opp);
            } else if let Some(cp) = undo.capture_piece {                             // remove captured piece
                *state.board.piece_board_mut(cp, opp) ^= 1u64 << to;
            }
        },
    };
}

pub fn check(state: &Chess, color: Color) -> bool {
    let king_board = state.board.piece_board(Piece::King, color);
    if king_board == 0 { return false; }
    let king_sq = king_board.trailing_zeros() as Square;
    sq_attacks(state, king_sq, opposite_color(color)) != 0
}

pub fn chess_move(state: &mut Chess, mv: Move) -> Option<UndoState> {
    let color = state.turn;

    if !is_pseudo_legal(state, mv) {
        return None;
    }

    let undo = make_move_unsafe(state, mv);

    if check(state, color) {
        undo_move(state, mv, undo);
        return None;
    }

    Some(undo)
}

pub fn generate_legal_moves(state: &mut Chess) -> Vec<Move> {
    let mut moves = Vec::with_capacity(256);
    let color = state.turn;

    if is_castle_valid(state, CastleSide::KingSide) {
        moves.push(Move::Castle { side: CastleSide::KingSide });
    }

    if is_castle_valid(state, CastleSide::QueenSide){
        moves.push(Move::Castle { side: CastleSide::QueenSide });
    }

    for piece in Piece::ALL {
        for from in state.board.get_piece_positions(piece, state.turn) {
            for to in get_bitmask_positions(piece_legal_move_bitboard(state, piece, from, state.turn)) {
                if piece == Piece::Pawn && (to / 8) == (if state.turn == Color::White { 7 } else { 0 }) {
                    for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                        let mv = Move::Promotion { from, to, promotion: piece };
                        let undo = make_move_unsafe(state, mv);
                        if !check(state, color) {
                            moves.push(mv);
                        }
                        undo_move(state, mv, undo);
                    }
                } else {
                    let mv = Move::Normal { piece, from, to };
                    let undo = make_move_unsafe(state, mv);
                    if !check(state, color) {
                        moves.push(mv);
                    }
                    undo_move(state, mv, undo);
                }
            }
        }
    }

    moves
}

pub fn generate_pseudo_legal_moves(state: &mut Chess) -> Vec<Move> {
    let mut moves = Vec::with_capacity(256);

    if is_castle_valid(state, CastleSide::KingSide) {
        moves.push(Move::Castle { side: CastleSide::KingSide });
    }

    if is_castle_valid(state, CastleSide::QueenSide){
        moves.push(Move::Castle { side: CastleSide::QueenSide });
    }

    for piece in Piece::ALL {
        for from in state.board.get_piece_positions(piece, state.turn) {
            for to in get_bitmask_positions(piece_legal_move_bitboard(state, piece, from, state.turn)) {
                if piece == Piece::Pawn && (to / 8) == (if state.turn == Color::White { 7 } else { 0 }) {
                    for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                        moves.push(Move::Promotion { from, to, promotion: piece });
                    }
                } else {
                    moves.push(Move::Normal { piece, from, to });
                }
            }
        }
    }

    moves
}

pub fn print_game(state: &Chess) {
    print_board(&state.board);
    println!("{}", chess_to_fen(state));
}

// Optimizations:
// Can't move unless resolve check
// Prune branches? E.g. piece pinned 

// Functionality
// Check + Checkmate + Stalemate

// perft
// benchmark for speed? 
