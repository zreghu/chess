use crate::game::*;
use crate::board::Board;

use std::fmt;

#[derive(Debug)]
pub enum FenError {
    MissingField(&'static str),
    TooManyFields,
    BoardFileTooLarge,
    BoardInvalidChar(char),
    ColorInvalid(String),
    CastlingAvailabilityInvalidChar(char),
    CastlingAvailabilityTooMany(char),
    EnPassant(SquareError),
    HalfMoveNumber(String),
    FullMoveNumber(String),
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenError::MissingField(field)                => write!(f, "FEN: missing field {field}"),
            FenError::TooManyFields                      => write!(f, "FEN: too many fields"),
            FenError::BoardFileTooLarge                  => write!(f, "FEN: board field has a file that is too large"),
            FenError::BoardInvalidChar(c)                => write!(f, "FEN: board field invalid char: {c}"),
            FenError::ColorInvalid(s)                    => write!(f, "FEN: color field invalid string: {s}"),
            FenError::CastlingAvailabilityInvalidChar(c) => write!(f, "FEN: castling_availability field invalid char: {c}"),
            FenError::CastlingAvailabilityTooMany(c)     => write!(f, "FEN: castling_availability field char defined too many times: {c}"),
            FenError::EnPassant(error)                   => write!(f, "FEN: invalid en passant square: {error}"),
            FenError::HalfMoveNumber(s)                   => write!(f, "FEN: invalid half_move_number: {s}"),
            FenError::FullMoveNumber(s)                   => write!(f, "FEN: invalid full_move_number: {s}"),
        }
    }
}

#[derive(Debug)]
pub enum SquareError {
    TooLong(String),
    TooShort(String),
    InvalidRank(char),
    InvalidFile(char),
}

impl fmt::Display for SquareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquareError::TooLong(s)     => write!(f, "square {s} is too long"),
            SquareError::TooShort(s)    => write!(f, "square {s} is too short"),
            SquareError::InvalidRank(c) => write!(f, "invalid rank: {c}"),
            SquareError::InvalidFile(c) => write!(f, "invalid rank: {c}"),
        }
    }
}

pub fn parse_square(s: &str) -> Result<Square, SquareError> {
    let mut fields = s.chars();

    let fl_char = fields.next().ok_or(SquareError::TooShort(s.to_owned()))?;
    let rk_char = fields.next().ok_or(SquareError::TooShort(s.to_owned()))?;

    if fields.next().is_some() { return Err(SquareError::TooLong(s.to_owned())); }

    let fl = match fl_char {
        'a' => 0,  
        'b' => 1,  
        'c' => 2,  
        'd' => 3,  
        'e' => 4,  
        'f' => 5,  
        'g' => 6,  
        'h' => 7,  
        _   => return Err(SquareError::InvalidFile(rk_char)),
    };

    if !('1'..='8').contains(&rk_char) { return Err(SquareError::InvalidRank(rk_char)); }
    let rk = rk_char as usize - '0' as usize;

    Ok(8 * (rk - 1) + fl)
}

pub fn square_to_str(sq: Square) -> String {
    let mut out = String::new();
    let (rk, fl) = (sq / 8, sq % 8);
    match fl {
        0 => out.push('a'),
        1 => out.push('b'),
        2 => out.push('c'),
        3 => out.push('d'),
        4 => out.push('e'),
        5 => out.push('f'),
        6 => out.push('g'),
        7 => out.push('h'),
        _ => panic!("Something isn't right with this square {sq}"),
    }
    out.push_str(&(rk+1).to_string());

    out
}

pub fn parse_fen(fen: &str) -> Result<Chess, FenError> {
    // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    // half move clock
    // full move clock
    
    let mut fields = fen.split_whitespace();

    let board_field = fields.next().ok_or(FenError::MissingField("board"))?;
    let color_field = fields.next().ok_or(FenError::MissingField("color"))?;
    let castling_availability_field = fields.next().ok_or(FenError::MissingField("castling_availability"))?;
    let en_passant_field = fields.next().ok_or(FenError::MissingField("en_passant"))?;
    let half_move_number_field = fields.next().ok_or(FenError::MissingField("half_move_number"))?;
    let full_move_number_field = fields.next().ok_or(FenError::MissingField("full_move_number"))?;

    if fields.next().is_some() {
        return Err(FenError::TooManyFields);
    }

    // Parse Board
    let mut board : Board = Board::empty();
    let (mut rk, mut fl) = (7, 0);
    for c in board_field.chars() {
        match c {
            'k' => { board.black_king   |= 1u64 << (8 * rk + fl); fl += 1 },
            'q' => { board.black_queen  |= 1u64 << (8 * rk + fl); fl += 1 },
            'r' => { board.black_rook   |= 1u64 << (8 * rk + fl); fl += 1 },
            'b' => { board.black_bishop |= 1u64 << (8 * rk + fl); fl += 1 },
            'n' => { board.black_knight |= 1u64 << (8 * rk + fl); fl += 1 },
            'p' => { board.black_pawn   |= 1u64 << (8 * rk + fl); fl += 1 },
            'K' => { board.white_king   |= 1u64 << (8 * rk + fl); fl += 1 },
            'Q' => { board.white_queen  |= 1u64 << (8 * rk + fl); fl += 1 },
            'R' => { board.white_rook   |= 1u64 << (8 * rk + fl); fl += 1 },
            'B' => { board.white_bishop |= 1u64 << (8 * rk + fl); fl += 1 },
            'N' => { board.white_knight |= 1u64 << (8 * rk + fl); fl += 1 },
            'P' => { board.white_pawn   |= 1u64 << (8 * rk + fl); fl += 1 },
            '1'..='8' => fl += c as u32 - '0' as u32,
            '/' if fl == 7 => return Err(FenError::BoardInvalidChar(c)),
            '/' if fl != 7 => { fl = 0; rk -= 1; },
            _ => return Err(FenError::BoardInvalidChar(c)),
        }

        if fl > 8 { return Err(FenError::BoardFileTooLarge) };
    }

    // Parse Color
    let turn = match color_field {
        "w" => Color::White,
        "b" => Color::Black,
        _   => return Err(FenError::ColorInvalid(color_field.to_owned())),
    };

    // Parse Castling Availability
    let mut castling_availability = CastlingAvailability::empty();
    let (mut wk, mut bk, mut wq, mut bq, mut done) = (false, false, false, false, false);
    for c in castling_availability_field.chars() {
        if done { return Err(FenError::CastlingAvailabilityTooMany(c)) };
        match c {
            'K' if !wk => { castling_availability.white_king_side = true; wk = true; },
            'Q' if !wq => { castling_availability.white_queen_side = true; wq = true; },
            'k' if !bk => { castling_availability.black_king_side = true; bk = true; },
            'q' if !bq => { castling_availability.black_queen_side = true; bq = true },
            'K' if wk => return Err(FenError::CastlingAvailabilityTooMany(c)),
            'Q' if wq => return Err(FenError::CastlingAvailabilityTooMany(c)),
            'k' if bk => return Err(FenError::CastlingAvailabilityTooMany(c)),
            'q' if bq => return Err(FenError::CastlingAvailabilityTooMany(c)),
            '-' => done = true,
            _   => return Err(FenError::CastlingAvailabilityInvalidChar(c)),
        }
    }

    let en_passant = if en_passant_field == "-" { 
        None 
    } else { 
        Some(parse_square(en_passant_field).map_err(FenError::EnPassant)?)
    };

    let half_move_number: u16 = half_move_number_field
        .parse::<u16>()
        .map_err(|_| FenError::HalfMoveNumber(half_move_number_field.to_owned()))?;

    let full_move_number: u16 = full_move_number_field
        .parse::<u16>()
        .map_err(|_| FenError::FullMoveNumber(full_move_number_field.to_owned()))?;


    Ok(Chess {board, turn, castling_availability, en_passant, half_move_number, full_move_number})
}

pub fn chess_to_fen(state: &Chess) -> String {
    let mut fen = String::new();

    let mut board: [Option<char>; 64] = [None; 64];
    for (sq, ptr) in board.iter_mut().enumerate() {
        if state.board.white_king & 1u64 << sq != 0 { *ptr = Some('K') }
        else if state.board.white_queen  & 1u64 << sq != 0 { *ptr = Some('Q') }
        else if state.board.white_rook   & 1u64 << sq != 0 { *ptr = Some('R') }
        else if state.board.white_bishop & 1u64 << sq != 0 { *ptr = Some('B') }
        else if state.board.white_knight & 1u64 << sq != 0 { *ptr = Some('N') }
        else if state.board.white_pawn   & 1u64 << sq != 0 { *ptr = Some('P') }
        else if state.board.black_king   & 1u64 << sq != 0 { *ptr = Some('k') }
        else if state.board.black_queen  & 1u64 << sq != 0 { *ptr = Some('q') }
        else if state.board.black_rook   & 1u64 << sq != 0 { *ptr = Some('r') }
        else if state.board.black_bishop & 1u64 << sq != 0 { *ptr = Some('b') }
        else if state.board.black_knight & 1u64 << sq != 0 { *ptr = Some('n') }
        else if state.board.black_pawn   & 1u64 << sq != 0 { *ptr = Some('p') }
    };

    let mut cnt = 0;
    for rk in (0..8).rev() {
        for fl in 0..8 {
            match board[rk * 8 + fl] {
                Some(c) => {
                    if cnt > 0 { fen.push_str(&cnt.to_string()); cnt = 0; }
                    fen.push(c);
                },
                None => cnt += 1,
            }
        }
        if cnt > 0 { fen.push_str(&cnt.to_string()); cnt = 0; }
        if rk != 0 { fen.push('/'); }
    }
    fen.push(' ');

    if state.turn == Color::White {
        fen.push('w');
    } else {
        fen.push('b');
    }
    fen.push(' ');

    if !state.castling_availability.white_king_side
        && !state.castling_availability.white_queen_side 
        && !state.castling_availability.black_king_side  
        && !state.castling_availability.black_queen_side {
        fen.push('-');
    } else {
        if state.castling_availability.white_king_side { fen.push('K'); }
        if state.castling_availability.white_queen_side { fen.push('Q'); }
        if state.castling_availability.black_king_side { fen.push('k'); }
        if state.castling_availability.black_queen_side { fen.push('q'); }
    }
    fen.push(' ');

    if let Some(sq) = state.en_passant {
        fen.push_str(&square_to_str(sq))
    } else {
        fen.push('-');
    }
    fen.push(' ');

    fen.push_str(&state.half_move_number.to_string());
    fen.push(' ');

    fen.push_str(&state.full_move_number.to_string());

    fen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_starting_position() {
        let chess = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();

        assert_eq!(chess, Chess::default());
    }

    #[test]
    fn fen_e4() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let chess = parse_fen(fen)
            .unwrap();

        assert_eq!(chess_to_fen(&chess), fen);
    }

    #[test]
    fn fen_e4_c5_nf3() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let chess = parse_fen(fen)
            .unwrap();

        assert_eq!(chess_to_fen(&chess), fen);
    }
}
