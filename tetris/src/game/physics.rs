use crate::models::{Board, Tetromino};

pub struct Physics;

impl Physics {

    pub fn check_collision(self, board: &Board, tetromino: &Tetromino) -> bool {
        for &(dx, dy) in &tetromino.shape {
            let x = tetromino.x + dx;
            let y = tetromino.y + dy;
    
            if x < 0 || x >= board.width as i32 || y >= board.height as i32 {
                return true;
            }

            if y >= 0 && board.shape[y as usize][x as usize] {
                return true;
            }
        }
        false
    }
}