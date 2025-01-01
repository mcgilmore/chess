// src/pieces.rs
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::{Context, GameError, GameResult};

/// A simple shape-based piece drawer.
pub struct Pieces;

impl Pieces {
    /// Construct a dummy "Pieces" object. (We have no data to store, but in case
    /// you want to expand later, we keep it as a struct.)
    pub fn new() -> Self {
        Pieces
    }

    /// Draws a piece made of simple shapes (circles, polygons, etc.) at (x, y).
    pub fn draw_piece(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        piece_color: crate::PieceColor,
        piece_type: crate::PieceType,
        x: f32,
        y: f32,
    ) -> GameResult<()> {
        // Let’s define a shape for each piece type. 
        // color: White pieces => light color, Black => dark color
        let (fill_color, outline_color) = match piece_color {
            crate::PieceColor::White => (Color::from_rgb(240, 240, 240), Color::BLACK),
            crate::PieceColor::Black => (Color::from_rgb(50, 50, 50), Color::WHITE),
        };

        // We'll build a Mesh using MeshBuilder
        let mut mb = MeshBuilder::new();
        // Scaling factors based on TILE_SIZE
        let tile_size = crate::TILE_SIZE;
        let piece_radius = tile_size * 0.4;
        let piece_offset = tile_size * 0.1; // Offset to center pieces in tile
        let piece_color = match piece_color {
            crate::PieceColor::White => Color::from_rgb(240, 240, 240),
            crate::PieceColor::Black => Color::from_rgb(50, 50, 50),
        };

        match piece_type {
            crate::PieceType::Pawn => {
                mb.circle(
                    DrawMode::fill(),
                    [x + tile_size / 2.0, y + tile_size / 2.0], // Centered within tile
                    piece_radius,
                    0.5,
                    piece_color,
                )?;
            }
            crate::PieceType::Knight => {
                // Rectangle base and head circle for knight
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_offset,
                        y + tile_size / 2.0 - piece_radius / 2.0,
                        piece_radius * 1.2,
                        piece_radius,
                    ),
                    piece_color,
                )?;
                mb.circle(
                    DrawMode::fill(),
                    [x + tile_size / 2.0, y + tile_size / 3.0], // Positioned slightly upward
                    piece_radius / 1.5,
                    0.5,
                    piece_color,
                )?;
            }
            crate::PieceType::Bishop => {
                // Two stacked circles for the bishop
                mb.circle(
                    DrawMode::fill(),
                    [x + tile_size / 2.0, y + tile_size / 2.5],
                    piece_radius,
                    0.5,
                    piece_color,
                )?;
                mb.circle(
                    DrawMode::fill(),
                    [x + tile_size / 2.0, y + tile_size / 1.5],
                    piece_radius / 1.5,
                    0.5,
                    piece_color,
                )?;
            }
            crate::PieceType::Rook => {
                // Base rectangle and battlements for the rook
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_offset,
                        y + tile_size / 2.0 - piece_radius,
                        piece_radius * 2.0,
                        piece_radius * 1.2,
                    ),
                    piece_color,
                )?;
                for i in 0..3 {
                    mb.rectangle(
                        DrawMode::fill(),
                        ggez::graphics::Rect::new(
                            x + piece_offset + (i as f32 * piece_radius / 1.2),
                            y + tile_size / 2.0 - piece_radius * 1.2,
                            piece_radius / 2.0,
                            piece_radius / 2.0,
                        ),
                        piece_color,
                    )?;
                }
            }
            crate::PieceType::Queen => {
                // Large base circle and crown spikes for the queen
                mb.circle(
                    DrawMode::fill(),
                    [x + tile_size / 2.0, y + tile_size / 2.0],
                    piece_radius,
                    0.5,
                    piece_color,
                )?;
                for i in 0..5 {
                    let angle = i as f32 * 72.0_f32.to_radians();
                    let dx = angle.cos() * piece_radius * 0.7;
                    let dy = angle.sin() * piece_radius * 0.7;
                    mb.circle(
                        DrawMode::fill(),
                        [x + tile_size / 2.0 + dx, y + tile_size / 2.0 + dy],
                        piece_radius / 3.0,
                        0.5,
                        piece_color,
                    )?;
                }
            }
            crate::PieceType::King => {
                // Rectangle base with a cross for the king
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_offset,
                        y + tile_size / 2.0 - piece_radius,
                        piece_radius * 2.0,
                        piece_radius * 1.5,
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + tile_size / 2.0 - piece_radius / 4.0,
                        y + tile_size / 3.0,
                        piece_radius / 2.0,
                        piece_radius / 2.0,
                    ),
                    piece_color,
                )?;
            }
        }

        // Optional: Outline the shape
        //mb.circle(DrawMode::stroke(2.0), [16.0, 16.0], 15.0, 0.5, outline_color);

        let mesh_data = mb.build();
        let mesh = ggez::graphics::Mesh::from_data(ctx, mesh_data);
        canvas.draw(&mesh, DrawParam::default());

        Ok(())
    }
}