use crate::models::{Board,Tetromino};

#[derive(Clone, Copy)]
pub struct BoardManager {
    pub board: Board,
    pub score: i32,
    pub game_over: bool
}

impl BoardManager {

    pub fn new() -> BoardManager {
        BoardManager { 
            board: Board::new(),
            score: 0,
            game_over: false
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

    pub fn check_game_over(&mut self) -> bool {
        for x in 0..self.board.width {
            if self.board.shape[0][x] {
                self.game_over = true;
                return true;
            }
        }
        false
    }

}

#[cfg(test)]
mod tests {
    use crate::models::TetrominoType;

    use super::*;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;

    #[test]
    fn when_new_board_manager_is_created_then_it_has_default_values() {
        let manager = BoardManager::new();
        assert_eq!(manager.score, 0);
        assert!(!manager.game_over);
        assert_eq!(manager.board.shape.len(), HEIGHT);
        assert_eq!(manager.board.shape[0].len(), WIDTH);
    }

    #[test]
    fn given_tetromino_when_moved_then_position_updates_correctly() {
        let manager = BoardManager::new();
        let mut tetromino = Tetromino::new(TetrominoType::T, WIDTH);

        let initial_x = tetromino.x;
        let initial_y = tetromino.y;

        manager.move_tetromino(&mut tetromino, 1, 1);

        assert_eq!(tetromino.x, initial_x + 1);
        assert_eq!(tetromino.y, initial_y + 1);
    }

    #[test]
    fn given_tetromino_when_rotated_then_shape_is_updated() {
        let manager = BoardManager::new();
        let mut tetromino = Tetromino::new(TetrominoType::T, WIDTH);

        manager.rotate_tetromino(&mut tetromino);

        // Assert rotation is consistent with a 90-degree clockwise rotation
        assert_eq!(tetromino.shape, vec![(0, 0), (0, 1), (0, 2), (-1, 1)]);
    }

    #[test]
    fn given_tetromino_when_rotated_and_undone_then_shape_returns_to_original() {
        let manager = BoardManager::new();
        let mut tetromino = Tetromino::new(TetrominoType::T, WIDTH);

        manager.rotate_tetromino(&mut tetromino);
        manager.undo_rotate_tetromino(&mut tetromino);

        assert_eq!(tetromino.shape, TetrominoType::T.shape());
    }

    #[test]
    fn when_lines_are_cleared_then_score_increases() {
        let mut manager = BoardManager::new();

        manager.score(&2);
        assert_eq!(manager.score, 200);

        manager.score(&3);
        assert_eq!(manager.score, 500);
    }

    #[test]
    fn given_tetromino_when_fixed_to_board_then_board_is_updated() {
        let manager = BoardManager::new();
        let mut board = Board::new();
        let tetromino = Tetromino::new(TetrominoType::O, WIDTH);

        manager.fix_to_board(&mut board, &tetromino);

        for &(dx, dy) in &tetromino.shape {
            let x = (tetromino.x + dx) as usize;
            let y = (tetromino.y + dy) as usize;
            assert!(board.shape[y][x]);
        }
    }

    #[test]
    fn given_full_line_when_cleared_then_board_is_updated() {
        let mut manager = BoardManager::new();
        manager.board.shape[HEIGHT - 1] = [true; WIDTH]; // Simulate a full line

        let cleared_lines = manager.clear_full_lines();

        assert_eq!(cleared_lines, 1);
        assert!(manager.board.shape[HEIGHT - 1].iter().all(|&cell| !cell));
    }

    #[test]
    fn given_top_row_occupied_when_checked_then_game_over_is_true() {
        let mut manager = BoardManager::new();
        manager.board.shape[0][0] = true; // Simulate a block at the top row

        let game_over = manager.check_game_over();

        assert!(game_over);
        assert!(manager.game_over);
    }
}
