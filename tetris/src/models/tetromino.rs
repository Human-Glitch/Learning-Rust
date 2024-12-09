use super::TetrominoType;

#[derive(Clone)]
pub struct Tetromino {
    pub shape: Vec<(i32, i32)>,
    pub x: i32,
    pub y: i32,
}

impl Tetromino {
    pub fn new(tetromino_type: TetrominoType, board_width: usize) -> Self {
        Tetromino { 
            shape: tetromino_type.shape(), 
            x: (board_width / 2) as i32, 
            y: 0 
        }
    }
}