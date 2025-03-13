use macroquad::prelude::*;
use ::rand::thread_rng;
use ::rand::Rng;

#[derive(Clone, Copy)]
enum PieceType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl PieceType {
    fn get_color(&self) -> Color {
        match self {
            PieceType::I => SKYBLUE,
            PieceType::J => BLUE,
            PieceType::L => ORANGE,
            PieceType::O => YELLOW,
            PieceType::S => GREEN,
            PieceType::T => PURPLE,
            PieceType::Z => RED,
        }
    }
}

struct Piece {
    shape: Vec<Vec<bool>>,
    x: i32,
    y: i32,
    piece_type: PieceType,
}

struct GameState {
    grid: Vec<Vec<Option<PieceType>>>,
    current_piece: Piece,
    last_fall: f64,
    block_size: f32,
    fall_speed: f64,  // Time between falls in seconds
}

impl GameState {
    fn get_piece_shape(piece_type: PieceType) -> Vec<Vec<bool>> {
        match piece_type {
            PieceType::I => vec![
                vec![true, true, true, true],
            ],
            PieceType::J => vec![
                vec![true, false, false],
                vec![true, true, true],
            ],
            PieceType::L => vec![
                vec![false, false, true],
                vec![true, true, true],
            ],
            PieceType::O => vec![
                vec![true, true],
                vec![true, true],
            ],
            PieceType::S => vec![
                vec![false, true, true],
                vec![true, true, false],
            ],
            PieceType::T => vec![
                vec![false, true, false],
                vec![true, true, true],
            ],
            PieceType::Z => vec![
                vec![true, true, false],
                vec![false, true, true],
            ],
        }
    }

    fn spawn_new_piece() -> Piece {
        let mut rng = thread_rng();
        let piece_type = match rng.gen_range(0..7) {
            0 => PieceType::I,
            1 => PieceType::J,
            2 => PieceType::L,
            3 => PieceType::O,
            4 => PieceType::S,
            5 => PieceType::T,
            _ => PieceType::Z,
        };

        Piece {
            shape: Self::get_piece_shape(piece_type),
            x: 4,
            y: 0,
            piece_type,
        }
    }

    fn rotate_piece(&mut self) {
        let old_shape = self.current_piece.shape.clone();
        let rows = old_shape.len();
        let cols = old_shape[0].len();
        
        // Create new rotated shape
        let mut new_shape = vec![vec![false; rows]; cols];
        
        // Rotate 90 degrees clockwise
        for i in 0..rows {
            for j in 0..cols {
                new_shape[j][rows - 1 - i] = old_shape[i][j];
            }
        }
        
        // Check if rotation is valid
        let old_shape = self.current_piece.shape.clone();
        self.current_piece.shape = new_shape;
        
        if !self.can_move(self.current_piece.x, self.current_piece.y) {
            // If rotation is invalid, revert back
            self.current_piece.shape = old_shape;
        }
    }

    fn clear_rows(&mut self) {
        let mut row = 19; // Start from bottom row
        
        while row > 0 {
            if self.grid[row].iter().all(|cell| cell.is_some()) {
                // Remove the completed row
                for r in (1..=row).rev() {
                    self.grid[r] = self.grid[r-1].clone();
                }
                // Add new empty row at top
                self.grid[0] = vec![None; 10];
            } else {
                row -= 1;
            }
        }
    }

    fn can_move(&self, new_x: i32, new_y: i32) -> bool {
        for (row_idx, row) in self.current_piece.shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell {
                    let grid_x = new_x + col_idx as i32;
                    let grid_y = new_y + row_idx as i32;
                    
                    if grid_x < 0 || grid_x >= 10 || grid_y >= 20 {
                        return false;
                    }
                    
                    if grid_y >= 0 && self.grid[grid_y as usize][grid_x as usize].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn lock_piece(&mut self) {
        for (row_idx, row) in self.current_piece.shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell {
                    let grid_x = self.current_piece.x + col_idx as i32;
                    let grid_y = self.current_piece.y + row_idx as i32;
                    
                    if grid_y >= 0 {
                        self.grid[grid_y as usize][grid_x as usize] = Some(self.current_piece.piece_type);
                    }
                }
            }
        }
        
        // Clear any completed rows
        self.clear_rows();
        
        // Spawn new piece
        self.current_piece = Self::spawn_new_piece();
    }

    fn new() -> Self {
        Self {
            grid: vec![vec![None; 10]; 20],
            current_piece: Self::spawn_new_piece(),
            last_fall: get_time(),
            block_size: 30.0,
            fall_speed: 0.5, // Normal fall speed
        }
    }
}

#[macroquad::main("Tetris")]
async fn main() {
    let mut game_state = GameState::new();
    
    // Calculate window size based on game grid
    let window_width = game_state.block_size * 12.0;
    let window_height = game_state.block_size * 22.0;
    
    request_new_screen_size(window_width, window_height);

    loop {
        clear_background(BLACK);

        // Handle input
        if is_key_pressed(KeyCode::Left) {
            let new_x = game_state.current_piece.x - 1;
            if game_state.can_move(new_x, game_state.current_piece.y) {
                game_state.current_piece.x = new_x;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            let new_x = game_state.current_piece.x + 1;
            if game_state.can_move(new_x, game_state.current_piece.y) {
                game_state.current_piece.x = new_x;
            }
        }
        if is_key_down(KeyCode::Down) {
            game_state.fall_speed = 0.05; // Fast fall speed
        } else {
            game_state.fall_speed = 0.5; // Normal fall speed
        }
        if is_key_pressed(KeyCode::R) {
            game_state.rotate_piece();
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Handle falling
        let current_time = get_time();
        if current_time - game_state.last_fall >= game_state.fall_speed {
            let new_y = game_state.current_piece.y + 1;
            if game_state.can_move(game_state.current_piece.x, new_y) {
                game_state.current_piece.y = new_y;
            } else {
                game_state.lock_piece();
            }
            game_state.last_fall = current_time;
        }

        // Draw border
        let border_color = DARKGRAY;
        for y in 0..22 {
            for x in 0..12 {
                if y == 0 || y == 21 || x == 0 || x == 11 {
                    draw_rectangle(
                        x as f32 * game_state.block_size,
                        y as f32 * game_state.block_size,
                        game_state.block_size,
                        game_state.block_size,
                        border_color
                    );
                }
            }
        }

        // Draw grid
        for y in 0..20 {
            for x in 0..10 {
                if let Some(piece_type) = game_state.grid[y][x] {
                    draw_rectangle(
                        (x as f32 + 1.0) * game_state.block_size,
                        (y as f32 + 1.0) * game_state.block_size,
                        game_state.block_size - 1.0,
                        game_state.block_size - 1.0,
                        piece_type.get_color()
                    );
                }
            }
        }

        // Draw current piece
        for (dy, row) in game_state.current_piece.shape.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell {
                    draw_rectangle(
                        ((game_state.current_piece.x + dx as i32 + 1) as f32) * game_state.block_size,
                        ((game_state.current_piece.y + dy as i32 + 1) as f32) * game_state.block_size,
                        game_state.block_size - 1.0,
                        game_state.block_size - 1.0,
                        game_state.current_piece.piece_type.get_color()
                    );
                }
            }
        }

        next_frame().await
    }
}