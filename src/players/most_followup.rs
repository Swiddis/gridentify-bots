use crate::gridentify;

use super::Player;

struct MostFollowup;

// Greedily take whichever move makes the most valuable square
impl Player for MostFollowup {
    fn get_move(game: &gridentify::Game) -> Vec<(usize, usize)> {
        let moves = game.moves();
        let mut max: (Vec<(usize, usize)>, u64) = (vec![], 0);
        for m in moves {
            let next = game.child(&m);
            let movecount = next.moves().len();
            if movecount > max.1 {
                max = (m, movecount);
            }
        }
        max.0
    }
}
