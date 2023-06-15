use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const SIZE: usize = 5;

type Board = [[u64; SIZE]; SIZE];

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    rng: ChaCha8Rng
}

impl Game {
    pub fn new(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut board: [[u64; SIZE]; SIZE] = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                board[i][j] = rng.gen_range(1..=3);
            }
        }
        Self {
            board: board,
            rng: rng,
        }
    }

    pub fn score(&self) -> u64 {
        self.board.iter().map(|row| row.iter().sum::<u64>()).sum()
    }

    pub fn moves(&self) -> Vec<Vec<(usize, usize)>> {
        let mut all_moves = Vec::new();
        let mut visited = [[false; SIZE]; SIZE];

        for row in 0..SIZE {
            for col in 0..SIZE {
                self.generate_moves((row, col), &mut Vec::new(), &mut all_moves, &mut visited);
            }
        }

        all_moves
    }

    fn generate_moves(
        &self,
        start: (usize, usize),
        current_move: &mut Vec<(usize, usize)>,
        all_moves: &mut Vec<Vec<(usize, usize)>>,
        visited: &mut [[bool; SIZE]; SIZE],
    ) {
        let (row, col) = start;
    
        // If the square is already visited, return
        if visited[row][col] {
            return;
        }
    
        visited[row][col] = true;
        current_move.push(start);
    
        // Generate all possible directions (up, down, left, right)
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    
        for (dr, dc) in directions.iter() {
            let next_row = row as i32 + dr;
            let next_col = col as i32 + dc;
    
            if next_row >= 0 && next_row < SIZE as i32 && next_col >= 0 && next_col < SIZE as i32 {
                let next_pos = (next_row as usize, next_col as usize);
    
                if self.board[next_pos.0][next_pos.1] == self.board[row][col] && !visited[next_pos.0][next_pos.1] {
                    self.generate_moves(next_pos, current_move, all_moves, visited);
                }
            }
        }
    
        if current_move.len() >= 2 {
            all_moves.push(current_move.clone());
        }
    
        visited[row][col] = false;
        current_move.pop();
    }

    pub fn child(&self, steps: &Vec<(usize, usize)>) -> Self {
        let mut rng = self.rng.clone();
        let mut board = self.board.clone();
        let mut total = 0;
        for step in steps[..steps.len()-1].iter() {
            total += board[step.0][step.1];
            board[step.0][step.1] = rng.gen_range(1..=3);
        }
        let last = steps[steps.len()-1];
        board[last.0][last.1] += total;
        Self {
            board: board,
            rng: rng
        }
    }

    pub fn done(&self) -> bool {
        self.moves().len() == 0
    }

    pub fn reseed(&self) -> Self {
        Self {
            board: self.board,
            rng: ChaCha8Rng::from_seed(self.rng.clone().gen())
        }
    }
}
