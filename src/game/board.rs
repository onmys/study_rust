pub mod cell;

use tetra::graphics::{ self, Color, Texture, DrawParams };
use tetra::input::{ self, Key };
use tetra::math::Vec2;
use tetra::{ Context, ContextBuilder, State };
use std::mem::MaybeUninit;

pub const WIDTH_NUM: usize = 8;     //  横の数
pub const HEIGHT_NUM: usize = 8;    //  縦の数
pub const DIR_X_MAX: usize = 8;     //  X方向の数
pub const DIR_Y_MAX: usize = 8;     //  Y方向の数

struct EnableInstallation {
    selected_position: Vec2<i32>,       //  ピース設置座標
    //color: cell::Color,
    reverse_position: Vec<Vec2<i32>>,   //  反転するマス座標
}

pub struct Board {
    texture: Texture,   //  ボード画像
    position: Vec2<f32>,    //  座標
    cells: [[cell::Cell; WIDTH_NUM]; HEIGHT_NUM],   //  マス
    can_select_cells: Vec<EnableInstallation>,  //  設置可能マス
}
    
impl Board {
    pub fn new( ctx: &mut Context ) -> tetra::Result<Board> {
        let texture = Texture::new( ctx, "./asset/board.png" )?;
        let position = Vec2::new( 32.0, 32.0 );
        let mut cells: [[MaybeUninit<cell::Cell>; WIDTH_NUM]; HEIGHT_NUM] = unsafe { MaybeUninit::uninit().assume_init() };
        let can_select_cells = Vec::with_capacity( WIDTH_NUM * HEIGHT_NUM );

        for i in 0 .. WIDTH_NUM {
            for k in 0 .. HEIGHT_NUM {
                let x = cell::WIDTH * i as f32 + 8.0;
                let y = cell::HEIGHT * k as f32 + 8.0;
                cells[i][k] = MaybeUninit::new( cell::Cell::new(ctx, x, y, cell::Color::NONE)? );
            }
        }
        
        let mut cells: [[cell::Cell; WIDTH_NUM]; HEIGHT_NUM] = unsafe { std::mem::transmute::<_, [[cell::Cell; WIDTH_NUM]; HEIGHT_NUM]>(cells) };

        Ok( Board {
            texture,
            position,
            cells,
            can_select_cells,
        } )
    }

    pub fn init( &mut self, ctx: &mut Context ) {
        for i in 0 .. WIDTH_NUM {
            for k in 0 .. HEIGHT_NUM {
                self.cells[i][k].set_piece( cell::Color::NONE );
            }
        }

        self.cells[3][3].set_piece( cell::Color::WHITE );
        self.cells[4][4].set_piece( cell::Color::WHITE );
        self.cells[3][4].set_piece( cell::Color::BLACK );
        self.cells[4][3].set_piece( cell::Color::BLACK );
    }

    pub fn update( &mut self, ctx: &mut Context ) {

    }

    pub fn draw( &mut self, ctx: &mut Context ) {
    self.texture.draw( ctx, self.position );

        for i in 0 .. WIDTH_NUM {
            for k in 0 .. HEIGHT_NUM {
                self.cells[i][k].draw( ctx, self.position );
            }
        }
    }

    //  ターン開始時の初期化
    pub fn init_turn( &mut self, color: cell::Color ) -> bool {
        //  TODO : 配置可能なセル
        self.can_select_cells.clear();

        for i in 0 .. WIDTH_NUM {
            for k in 0 .. HEIGHT_NUM {
                let can_place = self.checkEnableInstallation( i as i32, k as i32, color );
                self.cells[i][k].set_can_place( can_place );
            }
        }

        return true;
    }

    pub fn there_is_select_cells( &mut self ) -> bool {
        return self.can_select_cells.len() > 0;
    }

    //  マウス座標をマス座標に変換
    pub fn transform_board_num( &mut self, position: Vec2<f32> ) -> Vec2<i32> {
        let mut cell_num = Vec2::new( -1, -1 );
        let target_position = position - self.position;

        if target_position.x > 0.0 && target_position.x < self.texture.width() as f32 {
            cell_num.x = target_position.x as i32 / cell::WIDTH as i32;
        }

        if target_position.y > 0.0 && target_position.y < self.texture.height() as f32 {
            cell_num.y = target_position.y as i32 / cell::HEIGHT as i32;
        }

        return cell_num;
    }

    //  指定したマスがいくつあるか
    pub fn get_piece_count( &mut self, color: cell::Color ) -> i32 {
        let mut count = 0;

        for i in 0 .. WIDTH_NUM {
            for k in 0 .. HEIGHT_NUM {
                if color as i32 == self.cells[i][k].get_piece() as i32 {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn get_enable_installation( &mut self, x: i32, y: i32 ) -> Option<&EnableInstallation> {
        for iter in self.can_select_cells.iter() {
            if iter.selected_position.x == x && iter.selected_position.y == y {
                return Some( iter );
            }
        }

        return None;
    }

    //  指定マスに置けるか
    pub fn can_piece( &mut self, x: i32, y: i32, self_piece: cell::Color ) -> bool {
        if !self.is_inside( x, y ) {
            return false;
        }

        if !self.is_open( x, y ) {
            return false;
        }

        match self.get_enable_installation( x, y ) {
            Some( enable_installation ) => {
                return true;
            },
            None => {
                return false;
            }
        }
    }

    //  指定したマスに設置
    pub fn set_piece( &mut self, x: i32, y: i32, self_piece: cell::Color ) -> bool {
        if self.can_piece( x, y, self_piece) {
            match self.get_enable_installation( x, y ) {
                Some( enable_installation ) => {
                    let mut reverse_position: Vec<Vec2<i32>> = Vec::with_capacity( WIDTH_NUM * HEIGHT_NUM );
                    for iter in enable_installation.reverse_position.iter() {
                        reverse_position.push( *iter );
                    }

                    for iter in reverse_position.iter() {
                        self.cells[iter.x as usize][iter.y as usize].set_piece( self_piece );
                    }

                    self.cells[x as usize][y as usize].set_piece( self_piece );
                    return true;
                },
                None => {
                    return false;
                }
            }
        }
        
        return false;
    }

    //  指定したマスから一番近い自分のマスをチェック
    fn check_near_my_piece( &mut self, x: i32, y: i32, dir_x: i32, dir_y: i32, self_piece: cell::Color ) -> Vec<Vec2<i32>> {
        let mut reverse_position: Vec<Vec2<i32>> = Vec::with_capacity( WIDTH_NUM );
        let mut dir = 1;

        while self.is_inside( x + dir_x * dir, y + dir_y * dir ) {
            let offset_x = x + dir_x * dir;
            let offset_y = y + dir_y * dir;
            let offset_piece = self.cells[offset_x as usize][offset_y as usize].get_piece();

            if offset_piece as i32 == cell::Color::NONE as i32 {
                break;
            }

            if offset_piece as i32 == self_piece as i32 {
                return reverse_position;
            }

            //  裏返せるピース座標
            reverse_position.push( Vec2::new( offset_x, offset_y ) );
            dir += 1;
        }

        reverse_position.clear();
        return reverse_position;
    }

    //  指定したマスに設置可能かチェック
    fn checkEnableInstallation( &mut self, x: i32, y: i32, self_piece: cell::Color ) -> bool {
        if !self.is_inside( x, y ) {
            return false;
        }

        if !self.is_open( x, y ) {
            return false;
        }

        let dir_x = [ 0, 1, 1, 1, 0, -1, -1, -1 ];
        let dir_y = [ -1, -1, 0, 1, 1, 1, 0, -1 ];
        let reverse_position: Vec<Vec2<i32>> = Vec::with_capacity( WIDTH_NUM * HEIGHT_NUM );
        let selected_position = Vec2::new( -1, -1 );
        //self.can_select_cells.push( EnableInstallation{ selected_position, reverse_position } );

        let mut reverse_position: Vec<Vec2<i32>> = Vec::with_capacity( WIDTH_NUM * HEIGHT_NUM );

        for i in 0 .. dir_x.len() {
            for it in self.check_near_my_piece( x, y, dir_x[i], dir_y[i], self_piece ).iter() {
                reverse_position.push( *it );
            }
        }

        if reverse_position.len() > 0 {
            let selected_position = Vec2::new( x, y );
            self.can_select_cells.push( EnableInstallation{ selected_position, reverse_position } );
            return true;
        }

        return false;
    }

    //  指定したマスが内側かどうか
    fn is_inside( &mut self, x: i32, y: i32 ) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        if x as usize >= WIDTH_NUM || y as usize >= HEIGHT_NUM {
            return false;
        }

        return true;
    }

    //  指定したマスが空かどうか
    fn is_open( &mut self, x: i32, y: i32 ) -> bool {
        match self.cells[x as usize][y as usize].get_piece() {
            cell::Color::NONE => return true,
            cell::Color::BLACK => return false,
            cell::Color::WHITE => return false,
        }
    }

    //  相手のピース取得
    fn get_enemy_piece( &mut self, self_piece: cell::Color ) -> cell::Color {
        match self_piece {
            cell::Color::NONE => return cell::Color::NONE,
            cell::Color::BLACK => return cell::Color::WHITE,
            cell::Color::WHITE => return cell::Color::BLACK,
        }
    }
}