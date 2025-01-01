use ggez::event::{self, EventHandler, MouseButton};
use ggez::glam::Vec2;
use ggez::graphics::{
    self, Canvas, Color, DrawMode, DrawParam, Mesh, PxScale, Rect, Text, TextFragment,
};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use std::path::PathBuf;

mod pieces;
use pieces::Pieces;

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
}

impl ChessGame {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let pieces = Pieces::new();
        Ok(Self {
            board: ChessBoard::new_standard(),
            selected: None,
            turn: PieceColor::White, // White starts
            pieces,
            needs_redraw: true,
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

        row_diff <= 1 && col_diff <= 1
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
                        // Perform the move
                        let mut piece = self.board.squares[selected.0][selected.1].occupant.take().unwrap();
                        piece.has_moved = true; // Mark the piece as having moved
                        self.board.squares[row][col].occupant = Some(piece);
    
                        // End turn
                        self.turn = match self.turn {
                            PieceColor::White => PieceColor::Black,
                            PieceColor::Black => PieceColor::White,
                        };
    
                        self.selected = None; // Clear selection after the move
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
    let (mut ctx, event_loop) = ContextBuilder::new("chess", "YourName")
        .window_setup(WindowSetup::default().title("Chess"))
        .window_mode(WindowMode::default().dimensions((TILE_SIZE * 8.0), (TILE_SIZE * 8.0))) //Window size based on tile sizes
        .build()?;

    let game = ChessGame::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}