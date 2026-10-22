use::chess::game::{Chess, generate_pseudo_legal_moves, make_move_unsafe, undo_move, check};

fn main() {
    let mut chess = Chess::default();
    assert_eq!(perft(&mut chess, 5), 4865609);
}

fn perft(state: &mut Chess, depth: usize) -> u64 {
    let mut nodes = 0;
    if depth == 0 {
        return 1
    }
    let moves = generate_pseudo_legal_moves(state);
    let color = state.turn;

    for mv in moves {
        let undo = make_move_unsafe(state, mv);
        if !check(state, color) {
            nodes += perft(state, depth - 1);
        }
        undo_move(state, mv, undo);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use::chess::fen::parse_fen;
    // From https://github.com/official-stockfish/Stockfish/blob/master/tests/perft.sh

    #[test]
    #[cfg(not(debug_assertions))]
    fn perft_starting_position_depth_7() {
        let mut chess = Chess::default();
        assert_eq!(perft(&mut chess, 7), 3195901860);
    }

    #[test]
    fn perft_starting_position_depth_5() {
        let mut chess = Chess::default();
        assert_eq!(perft(&mut chess, 5), 4865609);
    }

    #[test]
    fn perft_fen_1() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut chess = parse_fen(fen).unwrap();

        assert_eq!(perft(&mut chess, 5), 193690690);
    }


    #[test]
    fn perft_fen_2() {
        let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        let mut chess = parse_fen(fen).unwrap();

        assert_eq!(perft(&mut chess, 7), 178633661);
    }


    #[test]
    fn perft_fen_3() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let mut chess = parse_fen(fen).unwrap();

        assert_eq!(perft(&mut chess, 6), 706045033);
    }


    #[test]
    fn perft_fen_4() {
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let mut chess = parse_fen(fen).unwrap();

        assert_eq!(perft(&mut chess, 5), 89941194);
    }

    #[test]
    fn perft_fen_5() {
        let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        let mut chess = parse_fen(fen).unwrap();

        assert_eq!(perft(&mut chess, 5), 164075551);
    }

    #[test]
    fn perft_fen_6() {
        let fen = "r7/4p3/5p1q/3P4/4pQ2/4pP2/6pp/R3K1kr w Q - 1 3";
        let mut chess = parse_fen(fen).unwrap();

        assert_eq!(perft(&mut chess, 5), 11609488);
    }
}
