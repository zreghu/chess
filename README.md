# Chess Move Generator in Rust

Back when I was in undergrad, I created a chess move generator and UI in Java as the final project for the introductory course.
The requirements were to use classes and polymorphism (you had to create some number of classes). Because of this I created a monstrosity.
I created a Piece class and each chess piece inherited from that piece class and the UI was in JavaFX. It worked but it was very slow, particularly
checkmate detection.

This is my attempt to right the atrocities I committed in 2018 and atone.

In this repo, I use rust. Piece positions are tracked with `u64` bitboards. For sliding attack pieces, I generate my own magic bitboards taking inspiration
from https://chessprogramming.org/Magic_Bitboards.

I also implement `perft` and a FEN translator for testing.

Improvements can still be made with legal move generation for perft, particularly with pin detection and pawn shifting to avoid make and undo to check
legal moves.
