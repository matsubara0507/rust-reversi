use board::{Board, Move};
use game::Play;
use piece::Piece;

pub struct DumbPlayer;
impl Play for DumbPlayer {
    fn play(&mut self, piece: Piece, board: &Board) -> Option<Move> {
        board.moves(piece).into_iter().next()
    }
}
