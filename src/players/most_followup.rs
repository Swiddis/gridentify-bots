use crate::gridentify;

use super::Player;

#[derive(Clone)]
pub struct MostFollowup;

// Greedily take whichever move makes the most valuable square
impl Player for MostFollowup {
    fn get_move(&self, game: &gridentify::Game) -> Vec<(usize, usize)> {
        let moves = game.moves();
        let mut max: (Vec<(usize, usize)>, usize) = (vec![], 0);
        for m in moves {
            let next = game.child(&m);
            let movecount = next.moves().len();
            if movecount > max.1 || max.0.len() == 0 {
                max = (m, movecount);
            }
        }
        max.0
    }
}
