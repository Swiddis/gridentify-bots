use crate::gridentify;

pub mod greedy;
pub mod most_followup;
pub mod random;

pub trait Player {
    fn get_move(&self, game: &gridentify::Game) -> Vec<(usize, usize)>;
}
