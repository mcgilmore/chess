use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::{Context, GameError, GameResult};

pub struct Pieces;

impl Pieces {
    pub fn new() -> Self {
        Pieces
    }

    // Placeholder: pieces made of simple shapes (circles, polygons, etc.)
    pub fn draw_piece(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        piece_color: crate::PieceColor,
        piece_type: crate::PieceType,
        x: f32,
        y: f32,
    ) -> GameResult<()> {
        let (fill_color, outline_color) = match piece_color {
            crate::PieceColor::White => (Color::from_rgb(220, 220, 220), Color::BLACK),
            crate::PieceColor::Black => (Color::from_rgb(50, 50, 50), Color::WHITE),
        };

        let mut mb = MeshBuilder::new();
        // Scaling factors based on TILE_SIZE
        let tile_size = crate::TILE_SIZE;
        let grid_square = tile_size / 10.0; // Each square will be a 10x10 grid
        let piece_radius = tile_size * 0.4;
        let piece_x_offset = tile_size * 0.2;
        let piece_color = match piece_color {
            crate::PieceColor::White => Color::from_rgb(240, 240, 240),
            crate::PieceColor::Black => Color::from_rgb(50, 50, 50),
        };
        
        // Each piece will be drawn on a 6x8 grid
        match piece_type {
            crate::PieceType::Pawn => {
                // Head and body
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 2.0,
                        y + grid_square * 2.0,
                        grid_square * 2.0, // width
                        grid_square * 6.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 3.0,
                        grid_square * 4.0, // width
                        grid_square * 2.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 1.5,
                        y + grid_square * 2.5,
                        grid_square * 3.0, // width
                        grid_square * 3.0, // height 
                    ),
                    piece_color,
                )?;
                // Base
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset,
                        y + grid_square * 7.0,
                        grid_square * 6.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 6.5,
                        grid_square * 4.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
            }
            crate::PieceType::Knight => {
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 2.0,
                        y + grid_square * 2.0,
                        grid_square * 2.0, // width
                        grid_square * 5.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 2.0,
                        y + grid_square * 2.5,
                        grid_square * 3.5, // width
                        grid_square * 2.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 2.0,
                        y + grid_square * 1.5,
                        grid_square * 0.5, // width
                        grid_square * 0.5, // height 
                    ),
                    piece_color,
                )?;
                // Base
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset,
                        y + grid_square * 7.0,
                        grid_square * 6.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 6.5,
                        grid_square * 4.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
            }
            crate::PieceType::Bishop => {
                // Body
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 2.0,
                        y + grid_square * 2.0,
                        grid_square * 2.0, // width
                        grid_square * 5.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 3.0,
                        y + grid_square,
                        grid_square * 0.5, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 2.5,
                        y + grid_square * 1.5,
                        grid_square * 1.5, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 2.5,
                        grid_square * 4.0, // width
                        grid_square * 0.5, // height 
                    ),
                    piece_color,
                )?;
                // Base
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset,
                        y + grid_square * 7.0,
                        grid_square * 6.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 6.5,
                        grid_square * 4.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
            }
            crate::PieceType::Rook => {
                // Body
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 1.5,
                        y + grid_square * 2.0,
                        grid_square * 3.0, // width
                        grid_square * 6.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 1.0,
                        y + grid_square * 1.0,
                        grid_square * 1.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 2.5,
                        y + grid_square * 1.0,
                        grid_square * 1.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square * 4.0,
                        y + grid_square * 1.0,
                        grid_square * 1.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                // Base
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset,
                        y + grid_square * 7.0,
                        grid_square * 6.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 6.5,
                        grid_square * 4.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
            }
            crate::PieceType::Queen => {
                
                // Base
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset,
                        y + grid_square * 7.0,
                        grid_square * 6.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 6.5,
                        grid_square * 4.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
            }
            crate::PieceType::King => {

                // Base
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset,
                        y + grid_square * 7.0,
                        grid_square * 6.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
                mb.rectangle(
                    DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        x + piece_x_offset + grid_square,
                        y + grid_square * 6.5,
                        grid_square * 4.0, // width
                        grid_square * 1.0, // height 
                    ),
                    piece_color,
                )?;
            }
        }

        let mesh_data = mb.build();
        let mesh = ggez::graphics::Mesh::from_data(ctx, mesh_data);
        canvas.draw(&mesh, DrawParam::default());

        Ok(())
    }
}