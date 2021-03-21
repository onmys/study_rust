use tetra::graphics::{ self, DrawParams, Rectangle, Texture };
use tetra::graphics::text::{ Font, Text };
use tetra::input::{ self, Key, MouseButton };
use tetra::math::Vec2;
use tetra::{ Context, ContextBuilder, State };

struct ParamUI {
    piece_black_piece_num: i32, //  ���s�[�X�̐�
    piece_white_piece_num: i32, //  ���s�[�X�̐�
    is_first_strike: bool,      //  ��U��
}

pub struct TextureUI {
    key: String,
    texture: Texture,
    param: DrawParams,
}

impl TextureUI {
    pub fn new( _key: String, path: String, _param: DrawParams, ctx: &mut Context ) -> tetra::Result<TextureUI> {
        let key = _key;
        let texture = Texture::new( ctx, path )?;
        let param = _param;
        Ok( TextureUI {
            key,
            texture,
            param,
        } )
    }

    pub fn draw( &mut self, ctx: &mut Context ) {
        let param = DrawParams::new()
            .position( self.param.position )
            .scale( self.param.scale )
            .origin( self.param.origin );
        self.texture.draw( ctx, param );
    }
}

pub struct TextUI {
    key: String,
    text: Text,
    param: DrawParams,
}

impl TextUI {
    pub fn new( _key: String, _text: String, font_size: f32, _param: DrawParams, ctx: &mut Context ) -> tetra::Result<TextUI> {
        let key = _key;
        let text = Text::new( _text, Font::vector( ctx, "asset/DejaVuSansMono.ttf", font_size )? );
        let param = _param;
        Ok( TextUI {
            key,
            text,
            param,
        } )
    }

    pub fn draw( &mut self, ctx: &mut Context ) {
        let param = DrawParams::new()
            .position( self.param.position )
            .scale( self.param.scale )
            .origin( self.param.origin );
        self.text.draw( ctx, param );
    }
}

pub struct ButtonUI {
    key: String,
    text: Text,
    param: DrawParams,
    callback: fn( &mut Context ),
}

impl ButtonUI {
    pub fn new( _key: String, _text: String, font_size: f32, _param: DrawParams, _func: fn( &mut Context ), ctx: &mut Context ) -> tetra::Result<ButtonUI> {
        let key = _key;
        let text = Text::new( _text, Font::vector( ctx, "asset/DejaVuSansMono.ttf", font_size )? );
        let param = _param;
        let callback = _func;
        Ok( ButtonUI {
            key,
            text,
            param,
            callback,
        } )
    }

    pub fn update( &mut self, ctx: &mut Context ) {
        if input::is_mouse_button_released( ctx, MouseButton::Left ) {
            let mouse_position = input::get_mouse_position( ctx );
            match self.text.get_bounds( ctx ) {
                Some(rect) => {
                    if mouse_position.x > rect.x && mouse_position.y > rect.y && 
                    mouse_position.x < rect.width && mouse_position.y < rect.height {
                        ( self.callback )( ctx );
                    }
                },
                NONE => {},
            }
        }
    }

    pub fn draw( &mut self, ctx: &mut Context ) {
        let param = DrawParams::new()
            .position( self.param.position )
            .scale( self.param.scale )
            .origin( self.param.origin );
        self.text.draw( ctx, param );
    }
}

pub struct UI {
    textureUI: Vec<TextureUI>,
    textUI: Vec<TextUI>,
    buttonUI: Vec<ButtonUI>,
    //reset: ButtonUI,    //  ���Z�b�g�e�L�X�g
    //win: TextUI,        //  �����e�L�X�g
}

impl UI {
    pub fn new( ctx: &mut Context ) -> tetra::Result<UI> {
        let textureUI = Vec::with_capacity( 10 );
        let textUI = Vec::with_capacity( 10 );
        let buttonUI = Vec::with_capacity( 10 );
        
        Ok( UI {
            textureUI,
            textUI,
            buttonUI,
        } )
    }

    fn test() {

    }

    pub fn init( &mut self, ctx: &mut Context ) {
        self.textureUI.clear();
        self.textUI.clear();
        self.buttonUI.clear();

        let turn_text = TextUI::new( "turn".to_string(), 
        "1Pのターン".to_string(), 32.0, DrawParams::new().position( Vec2::new( 720.0, 80.0 ) ), ctx );
        //turn_text.unwrap().text.push_str( &"test".to_string() );        
        self.textUI.push( turn_text.unwrap() );
    }

    pub fn update( &mut self, ctx: &mut Context ) {
        for i in 0 .. self.buttonUI.len() {
            self.buttonUI[i].update( ctx );
        }
    }

    pub fn draw( &mut self, ctx: &mut Context ) {
        for i in 0 .. self.textureUI.len() {
            self.textureUI[i].draw( ctx );
        }

        for i in 0 .. self.textUI.len() {
            self.textUI[i].draw( ctx );
        }

        for i in 0 .. self.buttonUI.len() {
            self.buttonUI[i].draw( ctx );
        }
    }

    pub fn push_textureUI( &mut self, textureUI: TextureUI ) {
        self.textureUI.push( textureUI );
    }

    pub fn push_textUI( &mut self, textUI: TextUI ) {
        self.textUI.push( textUI );
    }

    pub fn push_buttonUI( &mut self, buttonUI: ButtonUI ) {
        self.buttonUI.push( buttonUI );
    }

    pub fn pop_textureUI( &mut self, key: String ) -> Option<TextureUI> {
        for i in 0 .. self.textureUI.len() {
            if self.textureUI[i].key == key {
                let textureUI = self.textureUI.remove( i );
                return Some( textureUI );
            }
        }

        return None;
    }

    pub fn pop_textUI( &mut self, key: String ) -> Option<TextUI> {
        for i in 0 .. self.textUI.len() {
            if self.textUI[i].key == key {
                let textUI = self.textUI.remove( i );
                return Some( textUI );
            }
        }

        return None;
    }

    pub fn pop_buttonUI( &mut self, key: String ) -> Option<ButtonUI> {
        for i in 0 .. self.buttonUI.len() {
            if self.buttonUI[i].key == key {
                return Some( self.buttonUI.remove( i ) );
            }
        }

        return None;
    }
}