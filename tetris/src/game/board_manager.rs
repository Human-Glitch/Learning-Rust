use crate::models::{Board,Tetromino};

#[derive(Clone, Copy)]
pub struct BoardManager {
    pub board: Board,
    pub score: i32,
}

impl BoardManager {

    pub fn new() -> BoardManager {
        BoardManager { 
            board: Board::new(),
            score: 0 
        }
    }


    pub fn move_tetromino(self, current_tetromino: &mut Tetromino, dx: i32, dy: i32) {
        current_tetromino.x += dx;
        current_tetromino.y += dy;
    }

   pub fn rotate_tetromino(self, current_tetromino: &mut Tetromino) {
        for point in &mut current_tetromino.shape {
            let (x, y) = *point;
            *point = (-y, x);
        }
    }

    pub fn undo_rotate_tetromino(self, current_tetromino: &mut Tetromino) {
        for point in &mut current_tetromino.shape {
            let (x, y) = *point;
            *point = (y, -x);
        }
    }

    pub fn score(&mut self, lines_cleared: &i32) {
       self.score += lines_cleared * 100;
    }

    pub fn fix_to_board(self, board: &mut Board, tetromino: &Tetromino) {
        for &(dx, dy) in &tetromino.shape {
            let x = (tetromino.x + dx) as usize;
            let y = (tetromino.y + dy) as usize;
            if y < board.height && x < board.width {
                board.shape[y][x] = true;
            }
        }
    }
    
    pub fn clear_full_lines(&mut self) -> i32{
        let mut new_board: Board = Board::new();
        let mut new_y = self.board.height - 1;
        let mut lines_cleared = 0;
    
        for y in (0..self.board.height).rev() {
            if self.board.shape[y].iter().all(|&cell| cell) {
                lines_cleared += 1;
                continue; // Skip full lines
            }

            new_board.shape[new_y] = self.board.shape[y];

            if new_y > 0 {
                new_y -= 1;
            }
        }
    
        self.board = new_board;
        lines_cleared
    }
}