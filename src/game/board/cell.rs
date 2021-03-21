use tetra::graphics::{ self, Texture, DrawParams };
use tetra::input::{ self, Key };
use tetra::math::Vec2;
use tetra::{ Context, ContextBuilder, State };

pub const WIDTH: f32 = 80.0;
pub const HEIGHT: f32 = 80.0;

#[derive(Copy, Clone)]
pub enum Color {
    NONE,
    BLACK,
    WHITE,
}

pub struct Cell {
    color: Color,
    texture_black: Texture,
    texture_white: Texture,
    texture_can_place: Texture,
    canPlace: bool,
    //grid_x_num: i32,
    //grid_y_num: i32,
    position: Vec2<f32>,
}
    
impl Cell {
    pub fn new( ctx: &mut Context, x: f32, y: f32, color: Color ) -> tetra::Result<Cell> {
        let texture_black = Texture::new( ctx, "./asset/piece_black.png" )?;
        let texture_white = Texture::new( ctx, "./asset/piece_white.png" )?;
        let texture_can_place = Texture::new( ctx, "./asset/can_place_cell.png" )?;
        let canPlace = false;
        //let grid_x_num = grid_x;
        //let grid_y_num = grid_y;
        let position = Vec2::new( x, y );
        Ok( Cell {
            color,
            texture_black,
            texture_white,
            texture_can_place,
            canPlace,
            //grid_x_num,
            //grid_y_num,
            position,
        } )
    }

    pub fn init(&mut self) {

    }

    pub fn update(&mut self) {
        
    }

    pub fn draw( &mut self, ctx: &mut Context, oriin: Vec2<f32> ) {
        match self.color {
            Color::NONE => {
                if self.canPlace {
                    self.texture_can_place.draw( ctx, self.position + oriin );
                }
            },
            Color::BLACK => self.texture_black.draw(ctx, self.position + oriin),
            Color::WHITE => self.texture_white.draw(ctx, self.position + oriin),
        }
    }

    pub fn get_piece( &mut self ) -> Color {
        return self.color;
    }

    pub fn set_piece( &mut self, pieceColor: Color ) {
        self.color = pieceColor;
    }

    pub fn set_can_place( &mut self, can_place: bool ) {
        self.canPlace = can_place;
    }
}