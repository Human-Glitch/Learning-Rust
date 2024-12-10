extern crate piston_window;
use piston_window::*;

use rand::Rng;
use std::time;
mod models;
mod game;

use game::{BoardManager, Physics, Renderer};
use models::{Tetromino, TetrominoType};

const FONT_PATH: &str = "/Library/Fonts/Arial Unicode.ttf";
const BOARD_OFFSET: f64 = 50.0;
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
            board_manager.board.height as f64 * BLOCK_SIZE + BOARD_OFFSET]
        )
        .exit_on_esc(true)
        .build()
        .expect("An unknown issue occurred while creating the window");

    let mut glyphs = window
        .load_font(FONT_PATH)
        .expect("Unable to find the specified font.");

    let mut last_update = time::Instant::now();

    // Game loop
    while let Some(event) = window.next() {

        // Handle keyboard events
        if let Some(Button::Keyboard(key)) = event.press_args() {
            handle_input(key, &mut board_manager, &mut current_tetromino, &mut rng);
        }

        // Update game state
        if last_update.elapsed() >= time::Duration::from_millis(MOVE_INTERVAL) {
            update_game_state(&mut board_manager, &mut current_tetromino, &mut rng);
            last_update = time::Instant::now();
        }

        // Render the game
        render_game(&mut window, &event, &mut board_manager, &current_tetromino, &mut glyphs);
    }

    fn spawn_new_tetromino(rng: &mut impl Rng, board_manager: &BoardManager) -> Tetromino {
        Tetromino::new(
            TETROMINO_TYPES[rng.gen_range(0..TETROMINO_TYPES.len())].clone(),
            board_manager.board.width,
        )
    }

    fn handle_input(
        key: Key,
        board_manager: &mut BoardManager,
        current_tetromino: &mut Tetromino,
        rng: &mut impl Rng,
    ) {
        match key {
            Key::Left => {
                board_manager.move_tetromino(current_tetromino, -1, 0);
                if Physics.check_collision(&board_manager.board, &current_tetromino) {
                    board_manager.move_tetromino(current_tetromino, 1, 0);
                }
            }
            Key::Right => {
                board_manager.move_tetromino(current_tetromino, 1, 0);
                if Physics.check_collision(&board_manager.board, &current_tetromino) {
                    board_manager.move_tetromino(current_tetromino, -1, 0);
                }
            }
            Key::Down => {
                board_manager.move_tetromino(current_tetromino, 0, 1);
                if Physics.check_collision(&board_manager.board, &current_tetromino) {
                    board_manager.move_tetromino(current_tetromino, 0, -1);
                    board_manager.fix_to_board(&mut board_manager.board, &current_tetromino);

                    let lines_cleared = &board_manager.clear_full_lines();
                    board_manager.score(lines_cleared);

                    *current_tetromino = spawn_new_tetromino(rng, board_manager);
                }
            }
            Key::Up => {
                board_manager.rotate_tetromino(current_tetromino);
                if Physics.check_collision(&board_manager.board, &current_tetromino) {
                    board_manager.undo_rotate_tetromino(current_tetromino);
                }
            }
            Key::R => {
                if board_manager.game_over {
                    board_manager.game_over = false;
                    board_manager.score = 0;
                    board_manager.board = models::Board::new();
                    *current_tetromino = spawn_new_tetromino(rng, board_manager);
                }
            }
            Key::Q => if board_manager.game_over { std::process::exit(0); },
            _ => {}
        }
    }

    fn update_game_state(
        board_manager: &mut BoardManager,
        current_tetromino: &mut Tetromino,
        rng: &mut impl Rng,
    ){
        board_manager.move_tetromino(current_tetromino , 0, 1);

        if Physics.check_collision(&board_manager.board, &current_tetromino) {

            board_manager.move_tetromino(current_tetromino, 0, -1);
            board_manager.fix_to_board(&mut board_manager.board, &current_tetromino);

            let lines_cleared = &board_manager.clear_full_lines();
            board_manager.score(lines_cleared);

            *current_tetromino = spawn_new_tetromino(rng, board_manager);
        }
    }

    fn render_game(
        window: &mut PistonWindow,
        event: &Event,
        board_manager: &mut BoardManager,
        current_tetromino: &Tetromino,
        glyphs: &mut Glyphs,
    ){
        window.draw_2d(event, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);
    
            if board_manager.check_game_over() {
                Renderer.render_game_over(&board_manager.board, c, g, glyphs);
            } else {
                Renderer.render_board(&board_manager.board, current_tetromino, c, g);
            }
    
            Renderer.render_score(&board_manager.board, board_manager.score, c, g, glyphs);
            glyphs.factory.encoder.flush(device);
        });
    }

}
