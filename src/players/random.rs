use crate::gridentify;

use super::Player;
use rand::prelude::*;

struct Random;

impl Player for Random {
    fn get_move(game: &gridentify::Game) -> Vec<(usize, usize)> {
        let moves = game.moves();
        let mut rng = thread_rng();
        moves[rng.gen_range(1..moves.len())]
    }
}
