const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Clone, Copy)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub shape: [[bool; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            width: WIDTH,
            height: HEIGHT,
            shape: [[false; WIDTH]; HEIGHT],
        }
    }
}