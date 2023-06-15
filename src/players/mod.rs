pub mod random;
pub mod greedy;
pub mod most_followup;

trait Player {
    fn get_move(game: &gridentify::Game) -> Vec<(usize, usize)>;
}
