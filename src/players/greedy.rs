use crate::gridentify;

use super::Player;

struct Greedy;

// Greedily take whichever move makes the most valuable square
impl Player for Greedy {
    fn get_move(game: &gridentify::Game) -> Vec<(usize, usize)> {
        let moves = game.moves();
        let mut max: (Vec<(usize, usize)>, u64) = (vec![], 0);
        for m in moves {
            let mut sum = 0;
            for step in m {
                sum += game.board[step.0][step.1];
                if sum > max.1 {
                    max = (m, sum);
                }
            }
        }
        max.0
    }
}
