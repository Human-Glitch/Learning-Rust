#[derive(Clone)]
pub enum TetrominoType {
    T,
    O,
    I,
    L,
    S
}

impl TetrominoType {
    pub fn shape(&self) -> Vec<(i32, i32)> {
        match self {
            TetrominoType::T => vec![(0, 0), (1, 0), (2, 0), (1, 1)],
            TetrominoType::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            TetrominoType::I => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            TetrominoType::L => vec![(0, 0), (1, 0), (2, 0), (0, 1)],
            TetrominoType::S => vec![(1, 0), (2, 0), (0, 1), (1, 1)]
        }
    }
}
