use ggez::event::{self, EventHandler, MouseButton};
use ggez::glam::Vec2;
use ggez::graphics::{
    self, Canvas, Color, DrawMode, DrawParam, Mesh, PxScale, Rect, Text, TextFragment,
};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::conf::{WindowMode, WindowSetup};

use std::path::PathBuf;

use clap::Parser;

mod pieces;
use pieces::Pieces;

/// Command-line arguments for the chess game.
#[derive(Parser)]
#[command(name = "chess")]
#[command(author = "Dr. Hugh Jass")]
#[command(version = "0.1")]
#[command(about = "Simple chess in rust with ggez")]
struct Args {
    /// Optional FEN string to initialize the game state
    #[arg(short, long)]
    fen: Option<String>,
}

const BOARD_SIZE: usize = 8;
const TILE_SIZE: f32 = 150.0;

#[derive(Copy, Clone, PartialEq, Debug)]
enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Piece {
    piece_type: PieceType,
    color: PieceColor,
    has_moved: bool,
}

#[derive(Copy, Clone, Debug)]
struct Square {
    occupant: Option<Piece>,
}

struct ChessBoard {
    squares: [[Square; BOARD_SIZE]; BOARD_SIZE],
}

impl ChessBoard {
    fn empty() -> Self {
        ChessBoard {
            squares: [[Square { occupant: None }; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn new_standard() -> Self {
        let mut board = Self::empty();
    
        // Place pawns
        for file in 0..BOARD_SIZE {
            board.squares[6][file].occupant = Some(Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                has_moved: false,
            });
            board.squares[1][file].occupant = Some(Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                has_moved: false,
            });
        }
    
        // Place back ranks with `has_moved` set to false
        fn place_back_rank(row: usize, color: PieceColor, board: &mut ChessBoard) {
            board.squares[row][0].occupant = Some(Piece {
                piece_type: PieceType::Rook,
                color,
                has_moved: false,
            });
            board.squares[row][7].occupant = Some(Piece {
                piece_type: PieceType::Rook,
                color,
                has_moved: false,
            });
            board.squares[row][1].occupant = Some(Piece {
                piece_type: PieceType::Knight,
                color,
                has_moved: false,
            });
            board.squares[row][6].occupant = Some(Piece {
                piece_type: PieceType::Knight,
                color,
                has_moved: false,
            });
            board.squares[row][2].occupant = Some(Piece {
                piece_type: PieceType::Bishop,
                color,
                has_moved: false,
            });
            board.squares[row][5].occupant = Some(Piece {
                piece_type: PieceType::Bishop,
                color,
                has_moved: false,
            });
            board.squares[row][3].occupant = Some(Piece {
                piece_type: PieceType::Queen,
                color,
                has_moved: false,
            });
            board.squares[row][4].occupant = Some(Piece {
                piece_type: PieceType::King,
                color,
                has_moved: false,
            });
        }
    
        place_back_rank(0, PieceColor::Black, &mut board);
        place_back_rank(7, PieceColor::White, &mut board);
    
        board
    }
} 

struct ChessGame {
    board: ChessBoard,
    selected: Option<(usize, usize)>,
    pieces: Pieces,
    turn: PieceColor,
    needs_redraw: bool,
    castling_rights: String, // e.g., "KQkq", "-" if none
    en_passant_target: Option<(usize, usize)>, // Square where en passant is possible
    halfmove_clock: u32, // Number of halfmoves since the last capture or pawn move
    fullmove_number: u32, // Fullmove count (increments after Black's turn)
}

impl ChessGame {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let pieces = Pieces::new(); // Initialize the Pieces struct
        Ok(Self {
            board: ChessBoard::new_standard(),
            selected: None,
            turn: PieceColor::White,
            needs_redraw: true,
            castling_rights: "KQkq".to_string(),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            pieces,
        })
    }

    fn coords_to_square(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let col = (x / TILE_SIZE) as usize;
        let row = (y / TILE_SIZE) as usize;
        if row < BOARD_SIZE && col < BOARD_SIZE {
            Some((row, col))
        } else {
            None
        }
    }

    // Checks if a move is valid based on piece type, turn, and rules.
    fn validate_move(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        // Ensure both squares are on the board
        if start_row >= BOARD_SIZE || start_col >= BOARD_SIZE ||
           end_row >= BOARD_SIZE || end_col >= BOARD_SIZE {
            return false;
        }

        let start_square = self.board.squares[start_row][start_col];
        let end_square = self.board.squares[end_row][end_col];

        let piece = match start_square.occupant {
            Some(p) => p,
            None => return false,
        };

        // Ensure it's the correct player's turn
        if piece.color != self.turn {
            return false;
        }

        // Ensure the end square is not occupied by a friendly piece
        if let Some(occupant) = end_square.occupant {
            if occupant.color == piece.color {
                return false;
            }
        }

        // Validate movement based on piece type
        match piece.piece_type {
            PieceType::Pawn => self.validate_pawn_move(start, end, piece.color),
            PieceType::Knight => self.validate_knight_move(start, end),
            PieceType::Bishop => self.validate_bishop_move(start, end),
            PieceType::Rook => self.validate_rook_move(start, end),
            PieceType::Queen => self.validate_queen_move(start, end),
            PieceType::King => self.validate_king_move(start, end),
        }
    }

    fn validate_pawn_move(&self, start: (usize, usize), end: (usize, usize), color: PieceColor) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;
    
        let direction = if color == PieceColor::White { -1 } else { 1 };
        let start_row = start_row as isize;
        let start_col = start_col as isize;
        let end_row = end_row as isize;
        let end_col = end_col as isize;
    
        // One-square forward move
        if end_row == start_row + direction && end_col == start_col {
            return self.board.squares[end_row as usize][end_col as usize].occupant.is_none();
        }
    
        // Two-square forward move (if pawn hasn't moved yet)
        if end_row == start_row + 2 * direction && end_col == start_col {
            if let Some(piece) = self.board.squares[start_row as usize][start_col as usize].occupant {
                if !piece.has_moved && self.board.squares[(start_row + direction) as usize][start_col as usize].occupant.is_none()
                    && self.board.squares[end_row as usize][end_col as usize].occupant.is_none()
                {
                    return true;
                }
            }
        }
    
        // Diagonal capture
        if end_row == start_row + direction && (end_col == start_col + 1 || end_col == start_col - 1) {
            return self.board.squares[end_row as usize][end_col as usize].occupant.is_some();
        }

        if let Some((target_row, target_col)) = self.en_passant_target {
            if end_row == target_row as isize
                && end_col == target_col as isize
                && (end_col as isize - start_col as isize).abs() == 1
            {
                return true;
            }
        }
    
        false
    }

    fn validate_knight_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;
        let row_diff = (start_row as isize - end_row as isize).abs();
        let col_diff = (start_col as isize - end_col as isize).abs();

        (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)
    }

    fn validate_bishop_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        // Check if the move is diagonal
        let row_diff = (start_row as isize - end_row as isize).abs();
        let col_diff = (start_col as isize - end_col as isize).abs();

        if row_diff == col_diff {
            // Ensure no pieces block the path
            self.path_is_clear(start, end)
        } else {
            false
        }
    }

    fn validate_rook_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        // Check if the move is horizontal or vertical
        if start_row == end_row || start_col == end_col {
            self.path_is_clear(start, end)
        } else {
            false
        }
    }

    fn validate_queen_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        self.validate_rook_move(start, end) || self.validate_bishop_move(start, end)
    }

    fn validate_king_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;
        let row_diff = (start_row as isize - end_row as isize).abs();
        let col_diff = (start_col as isize - end_col as isize).abs();
    
        if row_diff <= 1 && col_diff <= 1 {
            return true;
        }
    
        // Check for castling
        self.validate_king_castling(start, end)
    }

    fn validate_king_castling(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        // King-side castling
        if start_row == end_row && (end_col as isize - start_col as isize).abs() == 2 {
            let is_king_side = end_col > start_col;
            let castling_right = if self.turn == PieceColor::White {
                if is_king_side { "K" } else { "Q" }
            } else {
                if is_king_side { "k" } else { "q" }
            };

            if !self.castling_rights.contains(castling_right) {
                return false;
            }

            // Ensure squares between king and rook are empty
            let rook_col = if is_king_side { 7 } else { 0 };
            let step = if is_king_side { 1 } else { -1 };

            for col in (start_col as isize + step..end_col as isize).map(|c| c as usize) {
                if self.board.squares[start_row][col].occupant.is_some() {
                    return false;
                }
            }

            // Ensure rook is in the correct position
            if let Some(piece) = self.board.squares[start_row][rook_col].occupant {
                if piece.piece_type == PieceType::Rook && piece.color == self.turn {
                    return true;
                }
            }
        }

        false
    }

    fn perform_castling(&mut self, start: (usize, usize), end: (usize, usize)) {
        let (start_row, start_col) = start;
        let is_king_side = end.1 > start_col;

        // Move the rook
        let rook_start_col = if is_king_side { 7 } else { 0 };
        let rook_end_col = if is_king_side { end.1 - 1 } else { end.1 + 1 };

        let rook = self.board.squares[start_row][rook_start_col].occupant.take();
        self.board.squares[start_row][rook_end_col].occupant = rook;
    }

    fn path_is_clear(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        let row_step = (end_row as isize - start_row as isize).signum();
        let col_step = (end_col as isize - start_col as isize).signum();

        let mut current_row = start_row as isize + row_step;
        let mut current_col = start_col as isize + col_step;

        while current_row != end_row as isize || current_col != end_col as isize {
            if self.board.squares[current_row as usize][current_col as usize].occupant.is_some() {
                return false;
            }

            current_row += row_step;
            current_col += col_step;
        }

        true
    }

    fn update_castling_rights(&mut self, start: (usize, usize)) {
        // Remove castling rights if king moves
        if let Some(piece) = self.board.squares[start.0][start.1].occupant {
            if piece.piece_type == PieceType::King {
                if piece.color == PieceColor::White {
                    self.castling_rights = self.castling_rights.replace("K", "").replace("Q", "");
                } else {
                    self.castling_rights = self.castling_rights.replace("k", "").replace("q", "");
                }
            }
            // Remove castling rights if rook moves
            if piece.piece_type == PieceType::Rook {
                if piece.color == PieceColor::White {
                    if start == (7, 0) {
                        self.castling_rights = self.castling_rights.replace("Q", "");
                    } else if start == (7, 7) {
                        self.castling_rights = self.castling_rights.replace("K", "");
                    }
                } else {
                    if start == (0, 0) {
                        self.castling_rights = self.castling_rights.replace("q", "");
                    } else if start == (0, 7) {
                        self.castling_rights = self.castling_rights.replace("k", "");
                    }
                }
            }
        }
    }

    fn is_king_in_check(&self, color: PieceColor) -> bool {
        let (king_row, king_col) = self.find_king(color).unwrap();
    
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board.squares[row][col].occupant {
                    if piece.color != color && self.validate_move((row, col), (king_row, king_col)) {
                        return true;
                    }
                }
            }
        }
    
        false
    }

    fn find_king(&self, color: PieceColor) -> Option<(usize, usize)> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board.squares[row][col].occupant {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Some((row, col));
                    }
                }
            }
        }
        None
    }

    fn is_checkmate(&self, color: PieceColor) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board.squares[row][col].occupant {
                    if piece.color == color {
                        for target_row in 0..BOARD_SIZE {
                            for target_col in 0..BOARD_SIZE {
                                if self.validate_move((row, col), (target_row, target_col)) {
                                    // Clone the game to simulate the move
                                    let mut cloned_game = self.clone();
                                    cloned_game.board.squares[target_row][target_col].occupant =
                                        cloned_game.board.squares[row][col].occupant.take();
                                    if !cloned_game.is_king_in_check(color) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }

    fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Convert board to FEN
        for row in (0..BOARD_SIZE).rev() {
            let mut empty_count = 0;

            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board.squares[row][col].occupant {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece_to_fen_char(piece));
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }

            if row > 0 {
                fen.push('/');
            }
        }

        // Add active color
        fen.push(' ');
        fen.push(if self.turn == PieceColor::White { 'w' } else { 'b' });

        // Add castling rights
        fen.push(' ');
        if self.castling_rights.is_empty() {
            fen.push('-');
        } else {
            fen.push_str(&self.castling_rights);
        }

        // Add en passant target square
        fen.push(' ');
        if let Some((row, col)) = self.en_passant_target {
            fen.push_str(&square_to_algebraic(row, col));
        } else {
            fen.push('-');
        }

        // Add halfmove clock and fullmove number
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    fn from_fen(&mut self, fen: &str) -> Result<(), String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 6 {
            return Err("Invalid FEN: Missing fields".to_string());
        }

        // Parse board layout
        let rows: Vec<&str> = parts[0].split('/').collect();
        if rows.len() != BOARD_SIZE {
            return Err("Invalid FEN: Incorrect number of rows".to_string());
        }

        for (row, row_data) in rows.iter().rev().enumerate() {
            let mut col = 0;
            for ch in row_data.chars() {
                if ch.is_digit(10) {
                    let empty_count = ch.to_digit(10).unwrap() as usize;
                    for _ in 0..empty_count {
                        self.board.squares[row][col] = Square { occupant: None };
                        col += 1;
                    }
                } else {
                    let piece = char_to_piece(ch).ok_or_else(|| format!("Invalid FEN: Unknown piece '{ch}'"))?;
                    self.board.squares[row][col] = Square { occupant: Some(piece) };
                    col += 1;
                }
            }

            if col != BOARD_SIZE {
                return Err("Invalid FEN: Row length mismatch".to_string());
            }
        }

        // Parse active color
        self.turn = match parts[1] {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => return Err("Invalid FEN: Invalid active color".to_string()),
        };

        // Parse castling rights
        self.castling_rights = parts[2].to_string();

        // Parse en passant target square
        self.en_passant_target = if parts[3] == "-" {
            None
        } else {
            algebraic_to_square(parts[3])
        };

        // Parse halfmove clock
        self.halfmove_clock = parts[4].parse().map_err(|_| "Invalid FEN: Invalid halfmove clock".to_string())?;

        // Parse fullmove number
        self.fullmove_number = parts[5].parse().map_err(|_| "Invalid FEN: Invalid fullmove number".to_string())?;

        Ok(())
    }
}

fn square_to_algebraic(row: usize, col: usize) -> String {
    let file = (b'a' + col as u8) as char;
    let rank = (8 - row) as u8;
    format!("{file}{rank}")
}

fn algebraic_to_square(pos: &str) -> Option<(usize, usize)> {
    if pos.len() != 2 {
        return None;
    }
    let file = pos.chars().nth(0)?;
    let rank = pos.chars().nth(1)?.to_digit(10)?;

    let col = (file as u8).wrapping_sub(b'a') as usize;
    let row = 8 - rank as usize;

    if col < BOARD_SIZE && row < BOARD_SIZE {
        Some((row, col))
    } else {
        None
    }
}

fn char_to_piece(ch: char) -> Option<Piece> {
    let color = if ch.is_uppercase() {
        PieceColor::White
    } else {
        PieceColor::Black
    };

    let piece_type = match ch.to_ascii_lowercase() {
        'p' => PieceType::Pawn,
        'n' => PieceType::Knight,
        'b' => PieceType::Bishop,
        'r' => PieceType::Rook,
        'q' => PieceType::Queen,
        'k' => PieceType::King,
        _ => return None,
    };

    Some(Piece {
        piece_type,
        color,
        has_moved: false, // Assumption: FEN doesn't track this explicitly
    })
}

fn piece_to_fen_char(piece: Piece) -> char {
    let ch = match piece.piece_type {
        PieceType::Pawn => 'p',
        PieceType::Knight => 'n',
        PieceType::Bishop => 'b',
        PieceType::Rook => 'r',
        PieceType::Queen => 'q',
        PieceType::King => 'k',
    };

    if piece.color == PieceColor::White {
        ch.to_ascii_uppercase()
    } else {
        ch
    }
}

impl Clone for ChessGame {
    fn clone(&self) -> Self {
        ChessGame {
            board: ChessBoard {
                squares: self.board.squares,
            },
            selected: self.selected,
            pieces: Pieces::new(), // Pieces doesn't need to carry state
            turn: self.turn,
            needs_redraw: self.needs_redraw,
            castling_rights: self.castling_rights.clone(),
            en_passant_target: self.en_passant_target,
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.fullmove_number,
        }
    }
}

impl Clone for ChessBoard {
    fn clone(&self) -> Self {
        ChessBoard {
            squares: self.squares,
        }
    }
}

impl EventHandler<GameError> for ChessGame {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        // No special logic, yet
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if !self.needs_redraw {
            return Ok(());
        }

        // Create a canvas, clear to green
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(34, 139, 34));

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let is_light = (row + col) % 2 == 0;
                let color = if is_light {
                    Color::from_rgb(238, 238, 210)
                } else {
                    Color::from_rgb(118, 150, 86)
                };

                let rect = Rect::new(
                    col as f32 * TILE_SIZE,
                    row as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                );
                let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }

        if let Some((r, c)) = self.selected {
            let rect = Rect::new(
                c as f32 * TILE_SIZE,
                r as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
            );
            let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(3.0), rect, Color::RED)?;
            canvas.draw(&mesh, DrawParam::default());
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board.squares[row][col].occupant {
                    let x = col as f32 * TILE_SIZE;
                    let y = row as f32 * TILE_SIZE;
                    self.pieces.draw_piece(ctx, &mut canvas, piece.color, piece.piece_type, x, y)?;
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        if button == MouseButton::Left {
            if let Some((row, col)) = self.coords_to_square(x, y) {
                if let Some(selected) = self.selected {
                    if selected == (row, col) {
                        // Unselect the currently selected square
                        self.selected = None;
                        self.needs_redraw = true;
                    } else if self.validate_move(selected, (row, col)) {
                        let mut piece = self.board.squares[selected.0][selected.1].occupant.take().unwrap();
                        piece.has_moved = true;

                        // Update en passant target for pawns moving two squares
                        if piece.piece_type == PieceType::Pawn && (selected.0 as isize - row as isize).abs() == 2 {
                            self.en_passant_target = Some(((selected.0 + row) / 2, col));
                        } else {
                            self.en_passant_target = None;
                        }

                        if piece.piece_type == PieceType::Pawn && Some((row, col)) == self.en_passant_target {
                            let captured_pawn_row = if piece.color == PieceColor::White { row + 1 } else { row - 1 };
                            self.board.squares[captured_pawn_row][col].occupant = None;
                        }

                        // Update castling rights (if a rook or king moves)
                        if piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::King {
                            self.update_castling_rights(selected);
                        }

                        // Update move counters
                        if piece.piece_type == PieceType::Pawn || self.board.squares[row][col].occupant.is_some() {
                            self.halfmove_clock = 0;
                        } else {
                            self.halfmove_clock += 1;
                        }
                        if self.turn == PieceColor::Black {
                            self.fullmove_number += 1;
                        }

                        if piece.piece_type == PieceType::King && (selected.1 as isize - col as isize).abs() == 2 {
                            self.perform_castling(selected, (row, col));
                        }
                        
                        

                        self.board.squares[row][col].occupant = Some(piece);
                        self.turn = match self.turn {
                            PieceColor::White => PieceColor::Black,
                            PieceColor::Black => PieceColor::White,
                        };
                        self.selected = None;
                        self.needs_redraw = true;
                        } else {
                            // Invalid move, clear selection
                            self.selected = None;
                            self.needs_redraw = true;
                        }
                        } else {
                            // Select a square if it has a piece belonging to the current player
                            if let Some(piece) = self.board.squares[row][col].occupant {
                                if piece.color == self.turn {
                                    self.selected = Some((row, col));
                                    self.needs_redraw = true;
                                }
                            }
                        }
            } else {
                // Clicked outside the board, clear selection
                self.selected = None;
                self.needs_redraw = true;
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    // Parse command-line arguments
    let args = Args::parse();

    let (mut ctx, event_loop) = ContextBuilder::new("chess", "YourName")
        .window_setup(WindowSetup::default().title("Chess"))
        .window_mode(WindowMode::default().dimensions((TILE_SIZE * 8.0), (TILE_SIZE * 8.0))) //Window size based on tile sizes
        .build()?;

    let mut game = ChessGame::new(&mut ctx)?;

    if let Some(fen) = args.fen {
        match game.from_fen(&fen) {
            Ok(_) => println!("Loaded FEN: {}", fen),
            Err(err) => {
                eprintln!("Failed to load FEN: {}", err);
                return Err(GameError::CustomError(err));
            }
        }
    }

    event::run(ctx, event_loop, game)
}