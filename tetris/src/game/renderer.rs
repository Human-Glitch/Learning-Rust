extern crate piston_window;
use piston_window::*;

use crate::models::{Board, Tetromino};

const BLOCK_SIZE: f64 = 20.0;

pub struct Renderer;

impl Renderer {

    pub fn render_board(self, board: &Board, tetromino: &Tetromino, c: Context, g: &mut G2d) {
        for y in 0..board.height {
            for x in 0..board.width {
                if board.shape[y][x] {
                    rectangle(
                        [0.0, 1.0, 0.0, 1.0],
                        [
                            x as f64 * BLOCK_SIZE,
                            y as f64 * BLOCK_SIZE,
                            BLOCK_SIZE,
                            BLOCK_SIZE,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        }
    
        for &(dx, dy) in &tetromino.shape {
            let x = (tetromino.x + dx) as usize;
            let y = (tetromino.y + dy) as usize;
            if x < board.width && y < board.height {
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [
                        x as f64 * BLOCK_SIZE,
                        y as f64 * BLOCK_SIZE,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }
        }
    }
    
    pub fn render_score(
        self, 
        board: &Board, 
        score: i32, 
        c: Context, 
        g: &mut G2d,
        glyphs: &mut Glyphs) {

        let transform = c.transform.trans(
            10.0, 
            board.height as f64 * BLOCK_SIZE + 30.0);

        let score_text = format!("Score: {}", score);

        text::Text::new_color(
            [1.0, 1.0, 1.0, 1.0], 
            20)
            .draw(
                &score_text,
                glyphs,
                &c.draw_state,
                transform,
                g)
            .unwrap_or_else(|e| println!("Failed to draw text: {}", e));
    }

}