use tetra::graphics::{ self, Color, Texture, DrawParams };
use tetra::input::{ self, Key };
use tetra::math::Vec2;
use tetra::{ Context, ContextBuilder, State };

mod game;

const WINDOW_WIDTH: i32 = 950;
const WINDOW_HEIGHT: i32 = 720;

struct GameState
{
    game: game::Game,
}

impl State for GameState {
    fn update( &mut self, ctx: &mut Context ) -> tetra::Result {
        if input::is_key_pressed( ctx, Key::W ) {
            //self.board_position.y -= 10.0;
        }
        if input::is_key_pressed( ctx, Key::S ) {
            //self.board_position.y += 10.0;
        }
        if input::is_key_pressed( ctx, Key::A ) {
            //self.board_position.x -= 10.0;
        }
        if input::is_key_pressed( ctx, Key::D ) {
            //self.board_position.x += 10.0;
        }

        self.game.update( ctx );

        Ok(())
    }

    fn draw( &mut self, ctx: &mut Context ) -> tetra::Result {
        graphics::clear( ctx, Color::BLACK );
        self.game.draw( ctx );
        Ok(())
    }
}

impl GameState {
    fn new( ctx: &mut Context ) -> tetra::Result<GameState> {
        let mut game = game::Game::new( ctx )?;
        game.init( ctx );
        Ok( GameState {
            game,
        } )
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new( "Othello", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32 )
    .quit_on_escape( true) 
    .build()?
    .run( GameState::new )
}

fn add() -> bool {
    println!( "{}", 1 );
    return true;
}


//  構文の確認

fn test1( x: i32, y: i32, max :i32 ) -> i32 {
    //  変数宣言
    let mut sum: i32 = x + y;   //  : i32 は省略できる mutがないと定数になる

    //  if文
    if sum > max {
        sum = max;
    }

    sum = if sum > max {
        max
    } else {
        sum
    };

    return sum;
}

fn test2() {
    // プログラムが終了するまでループ
    let mut num = 0;
    loop {
        println!( "ループ数：{}", num );
        //  Rustでは++できない
        num += 1;

        if num > 5 {
            break;
        }
    }

    //  while文
    while num < 10 {
        num += 1;
    }

    //  for文
    num = 0;
    for num in ( 1..10 ).rev() {
        println!( "{}", num );
    }

    let array = [ 1, 2, 3, 4, 5 ];

    for value in array.iter() {
        println!( "{}", value );
    }
}