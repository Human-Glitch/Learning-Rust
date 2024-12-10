use crate::models::{Board, Tetromino};

pub struct Physics;

impl Physics {

    pub fn check_collision(self, board: &Board, tetromino: &Tetromino) -> bool {
        for &(dx, dy) in &tetromino.shape {
            let x = tetromino.x + dx;
            let y = tetromino.y + dy;
    
            if 
                x < 0 || 
                x >= board.width as i32 || 
                y >= board.height as i32 || 
                y < 0  {
                return true;
            }

            if y >= 0 && board.shape[y as usize][x as usize] {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEIGHT: usize = 20;

    #[test]
    fn given_tetromino_out_of_bounds_when_checked_then_collision_detected() {
        let board = Board::new();
        let tetromino = Tetromino {
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            x: -1,
            y: 0,
        };

        let physics = Physics;
        assert!(physics.check_collision(&board, &tetromino));
    }

    #[test]
    fn given_tetromino_within_bounds_when_checked_then_no_collision() {
        let board = Board::new();
        let tetromino = Tetromino {
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            x: 4,
            y: 0,
        };

        let physics = Physics;
        assert!(!physics.check_collision(&board, &tetromino));
    }

    #[test]
    fn given_tetromino_on_filled_cell_when_checked_then_collision_detected() {
        let mut board = Board::new();
        board.shape[1][4] = true; // Simulate a filled cell

        let tetromino = Tetromino {
            shape: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            x: 4,
            y: 1,
        };

        let physics = Physics;
        assert!(physics.check_collision(&board, &tetromino));
    }

    #[test]
    fn given_tetromino_outside_top_boundary_when_checked_then_collision_detected() {
        let board = Board::new();
        let tetromino = Tetromino {
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            x: 0,
            y: -1,
        };

        let physics = Physics;
        assert!(physics.check_collision(&board, &tetromino));
    }

    #[test]
    fn given_tetromino_outside_bottom_boundary_when_checked_then_collision_detected() {
        let board = Board::new();
        let tetromino = Tetromino {
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            x: 0,
            y: (HEIGHT as i32),
        };

        let physics = Physics;
        assert!(physics.check_collision(&board, &tetromino));
    }
}