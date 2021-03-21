use tetra::{ Context };
use tetra::graphics::{ DrawParams };
use tetra::math::Vec2;
use tetra::input::{ self, MouseButton };

mod board;
mod player;
mod ui;

const PLAYER_NUM: usize = 2;

#[derive(Copy, Clone)]
enum PLAYER {
    ONE = 0,
    TWO = 1,
}

enum State {
    NONE,
    TURN_START,
    TURN_UPDATE,
    TURN_FINISH,
    RESULT,
    RESULT_WAIT,
    RESET,
}

pub struct Game {
    state: State,
    players: [player::Player; PLAYER_NUM],
    player_num: PLAYER,
    board: board::Board,
    ui: ui::UI,
    skip_count: i32,
}

impl Game {
    pub fn new( ctx: &mut Context ) -> tetra::Result<Game> {
        let state = State::TURN_START;
        let players: [player::Player; PLAYER_NUM] = 
        [ player::Player::new( player::Color::BLACK, ctx )?, 
          player::Player::new( player::Color::WHITE, ctx )?
        ];
        let player_num = PLAYER::ONE;
        let board = board::Board::new( ctx )?;
        let ui = ui::UI::new( ctx )?;
        let skip_count = 0;

        Ok( Game {
            state,
            players,
            player_num,
            board,
            ui,
            skip_count,
        } )
    }

    pub fn init( &mut self, ctx: &mut Context ) {
        self.player_num = PLAYER::ONE;
        self.skip_count = 0;
        self.players[PLAYER::ONE as usize].init( ctx );
        self.players[PLAYER::TWO as usize].init( ctx );
        self.board.init( ctx );
        self.ui.init( ctx );

        //let mut test = &self;
        //let mut func: fn( &mut Context ) = | ctx | {
        //    test.init( ctx );
        //    test.state = State::TURN_START;
        //};

        //let resetButtonUI = ui::ButtonUI::new( "reset".to_string(), "Reset".to_string(), 48.0,
        //DrawParams::new().position( Vec2::new( 0.0, 0.0 ) ), func, ctx );
        //self.ui.push_buttonUI( resetButtonUI.unwrap() );
    }

    pub fn update( &mut self, ctx: &mut Context ) {
        match self.state {
            State::NONE => {},
            //  ターン開始
            State::TURN_START => {
                self.turn_start( ctx );
            },
            //  ターン更新
            State::TURN_UPDATE => {
                self.turn_update( ctx );
            },
            //  ターン終了
            State::TURN_FINISH => {
                self.turn_finish();
            },
            //  リセット
            State::RESET => {
                self.reset( ctx );
            },
            //  リザルト
            State::RESULT => {
                self.result( ctx );
            },
            State::RESULT_WAIT => {
                self.result_wait( ctx );
            },
        }

        self.ui.update( ctx );
    }

    pub fn draw( &mut self, ctx: &mut Context ) {
        self.board.draw( ctx );
        self.players[self.player_num as usize].draw( ctx );
        self.ui.draw( ctx );
    }

    //  ターン開始
    fn turn_start( &mut self, ctx: &mut Context ) {
        self.board.init_turn(self.transform_color());
        self.players[self.player_num as usize].init( ctx );

        if self.board.there_is_select_cells() {
            self.skip_count = 0;

            self.ui.pop_textUI( "turn".to_string() );
            let mut text: String;
            match self.player_num {
                PLAYER::ONE => { text = "Black Turn".to_string() },
                PLAYER::TWO => { text = "White Turn".to_string() },
            }
            let turn_text = ui::TextUI::new( "turn".to_string(), 
            text, 32.0, DrawParams::new().position( Vec2::new( 720.0, 80.0 ) ), ctx );
            self.ui.push_textUI( turn_text.unwrap() );

            self.state = State::TURN_UPDATE;
            return;
        }

        //  置くところが無ければターン終了
        self.skip_count += 1;

        //  2回連続スキップはリザルトへ
        if self.skip_count >= 2 {
            self.state = State::RESULT;
        }

        self.state = State::TURN_FINISH;
    }

    //  ターン更新
    fn turn_update( &mut self, ctx: &mut Context ) {
        let player_num = self.player_num as usize;
        self.players[player_num].update( ctx );

        if self.players[player_num].get_selected() {
            let cell_position = self.board.transform_board_num(self.players[player_num].get_position());
            if self.board.set_piece(cell_position.x, cell_position.y, self.transform_color()) {
                self.state = State::TURN_FINISH;
            } else {
                self.players[player_num].cancel();
            }
        }
    }

    //  ターン終了
    fn turn_finish( &mut self ) {
        //  終了チェック
        if self.board.get_piece_count( board::cell::Color::NONE ) == 0 {
            self.state = State::RESULT;
            return;
        }

        //  プレイヤー交代
        match self.player_num {
            PLAYER::ONE => {
                self.player_num = PLAYER::TWO;
            },
            PLAYER::TWO => {
                self.player_num = PLAYER::ONE;
            },
        }

        self.state = State::TURN_START;
    }

    //  リセット
    fn reset( &mut self, ctx: &mut Context ) {
        self.init( ctx );
        self.state = State::TURN_START;
    }

    //  リザルト
    fn result( &mut self, ctx: &mut Context ) {
        let black_piece_num = self.board.get_piece_count( board::cell::Color::BLACK );
        let white_piece_num = self.board.get_piece_count( board::cell::Color::WHITE );
        let result_text: String;

        if black_piece_num == white_piece_num {
            result_text = "Draw".to_string();
        } else if black_piece_num < white_piece_num {
            result_text = "White Win".to_string();
        } else {
            result_text = "Black Win".to_string();
        }

        self.ui.push_textUI( ui::TextUI::new( "result".to_string(), result_text,
        64.0, DrawParams::new().position( Vec2::new( 800.0, 400.0 ) ), ctx ).unwrap() );

        self.state = State::RESULT_WAIT;
    }

    fn result_wait( &mut self, ctx: &mut Context ) {
        if input::is_mouse_button_released( ctx, MouseButton::Left ) {
            self.state = State::RESET;
        }
    }

    fn transform_color( &self ) -> board::cell::Color {
        match self.player_num {
            PLAYER::ONE => {
                return board::cell::Color::BLACK;
            },
            PLAYER::TWO => {
                return board::cell::Color::WHITE;
            },
        }
    }
}