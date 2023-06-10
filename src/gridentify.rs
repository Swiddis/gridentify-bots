use rand::prelude::*;

const SIZE: usize = 5;

type Board = Box<[[u64; SIZE]; SIZE]>;

#[derive(Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut board: [[u64; SIZE]; SIZE] = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                board[i][j] = rng.gen_range(1..=3);
            }
        }
        Self {
            board: Box::new(board),
        }
    }

    pub fn score(&self) -> u64 {
        self.board.iter().map(|row| row.iter().sum::<u64>()).sum()
    }
}
