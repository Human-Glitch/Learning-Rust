extern crate piston_window;
use piston_window::*;

use rand::Rng;
use std::time;
mod models;
mod game;

use game::{BoardManager, Physics, Renderer};
use models::{Tetromino, TetrominoType};

const BLOCK_SIZE: f64 = 20.0;
const MOVE_INTERVAL: u64 = 500; // Interval in milliseconds for tetromino to move down

const TETROMINO_TYPES: [TetrominoType; 5] = [
    TetrominoType::T,
    TetrominoType::O, 
    TetrominoType::I,
    TetrominoType::L, 
    TetrominoType::S
];

fn main() {

    let mut board_manager = BoardManager::new();
    
    let mut rng = rand::thread_rng();
    let mut current_tetromino = models::Tetromino::new(
        TETROMINO_TYPES[rng.gen_range(0..TETROMINO_TYPES.len())].clone(), 
        board_manager.board.width);

    let mut window: PistonWindow = WindowSettings::new(
        "Tetris", 
        [
            board_manager.board.width as f64 * BLOCK_SIZE, 
            board_manager.board.height as f64 * BLOCK_SIZE + 50.0])
        .exit_on_esc(true)
        .build()
        .expect("An unknown issue occurred while creating the window");

    let mut glyphs = window
        .load_font("/Library/Fonts/Arial Unicode.ttf")
        .expect("Unable to find the specified font.");

    let mut last_update = time::Instant::now();

    // Game loop
    while let Some(event) = window.next() {
        
        // Handle keyboard events
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Left => {
                    board_manager.move_tetromino(&mut current_tetromino, -1, 0);
                    if Physics.check_collision(&board_manager.board, &current_tetromino) {
                        board_manager.move_tetromino(&mut current_tetromino, 1, 0);
                    }
                }
                Key::Right => {
                    board_manager.move_tetromino(&mut current_tetromino, 1, 0);
                    if Physics.check_collision(&board_manager.board, &current_tetromino) {
                        board_manager.move_tetromino(&mut current_tetromino, -1, 0);
                    }
                }
                Key::Down => {
                    board_manager.move_tetromino(&mut current_tetromino, 0, 1);
                    if Physics.check_collision(&board_manager.board, &current_tetromino) {
                        board_manager.move_tetromino(&mut current_tetromino, 0, -1);
                        board_manager.fix_to_board(&mut board_manager.board, &current_tetromino);

                        let lines_cleared = &board_manager.clear_full_lines();
                        board_manager.score(lines_cleared);

                        current_tetromino = Tetromino::new(
                            TETROMINO_TYPES[rng.gen_range(0..TETROMINO_TYPES.len())].clone(),
                            board_manager.board.width);
                    }
                }
                Key::Up => {
                    board_manager.rotate_tetromino(&mut current_tetromino);
                    if Physics.check_collision(&board_manager.board, &current_tetromino) {
                        board_manager.undo_rotate_tetromino(&mut current_tetromino);
                    }
                }
                _ => {}
            }
        }

        // Update game cadence
        if last_update.elapsed() >= time::Duration::from_millis(MOVE_INTERVAL) {
            board_manager.move_tetromino(&mut current_tetromino , 0, 1);

            if Physics.check_collision(&board_manager.board, &current_tetromino) {

                board_manager.move_tetromino(&mut current_tetromino, 0, -1);
                board_manager.fix_to_board(&mut board_manager.board, &current_tetromino);

                let lines_cleared = &board_manager.clear_full_lines();
                board_manager.score(lines_cleared);

                current_tetromino = Tetromino::new(
                    TETROMINO_TYPES[rng.gen_range(0..TETROMINO_TYPES.len())].clone(),
                    board_manager.board.width);
            }

            last_update = time::Instant::now();
        }

        // Render the game
        window.draw_2d(
            &event, |c, g, 
            device| { 
                clear([0.0, 0.0, 0.0, 1.0], g);
                Renderer.render_board(&board_manager.board, &current_tetromino, c, g);
                Renderer.render_score(&board_manager.board, board_manager.score, c, g, &mut glyphs);
                glyphs.factory.encoder.flush(device);
            });
    }

}
