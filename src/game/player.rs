use tetra::graphics::{ self, Texture, DrawParams };
use tetra::input::{ self, MouseButton };
use tetra::math::Vec2;
use tetra::{ Context, ContextBuilder, State };

pub enum Color {
    BLACK,
    WHITE,
}
    
pub struct Player {
    color: Color,
    param: DrawParams,
    selected: bool,
    texture: Texture,
}
    
impl Player {
    pub fn new( color: Color, ctx: &mut Context ) -> tetra::Result<Player> {
        //let color = Color::BLACK;
        let texture = Texture::new( ctx, "./asset/cursor.png" )?;
        let param = DrawParams::new()
            .position( Vec2::new( 32.0, 32.0 ) )
            .scale( Vec2::new( 0.5, 0.5 ) )
            .origin( Vec2::new( texture.width() as f32 / 2.0, texture.height() as f32 / 2.0 ) );
        let selected = false;
        Ok( Player {
            color,
            param,
            selected,
            texture
        } )
    }

    pub fn init( &mut self, ctx: &mut Context ) {
        self.selected = false;
    }

    pub fn update( &mut self, ctx: &mut Context ) {
        self.param.position = input::get_mouse_position( ctx );
        if input::is_mouse_button_released( ctx, MouseButton::Left ) {
            self.selected = true;
        }
    }

    pub fn draw( &mut self, ctx: &mut Context ) {
        let param = DrawParams::new()
            .position( self.param.position )
            .scale( self.param.scale )
            .origin( self.param.origin );
        self.texture.draw( ctx, param );
    }

    pub fn get_position( &mut self ) -> Vec2<f32> {
        return self.param.position;
    }

    pub fn get_selected( &mut self ) -> bool {
        return self.selected;
    }

    pub fn cancel( &mut self ) {
        self.selected = false;
    }
}