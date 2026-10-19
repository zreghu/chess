use chess::board::*;
use chess::game::*;

fn main() {
    let game = Chess::new();
    print_board(&game.board); 
}
// Check + Checkmate + Stalemate
// Dead Position?
//
