extern crate rayon;
extern crate smallvec;

mod board;
mod coord;
mod game;
mod piece;
mod players;

use game::Game;
use players::*;

fn main() {
    let p1 = AlphaBetaPlayer::new(7);
    let p2 = Human;
    let mut game = Game::new(p1, p2);
    while {
        game.print();
        game.step()
    } {}
}
