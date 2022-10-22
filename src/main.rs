extern crate sdl2;
extern crate rand;

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::{thread, time};
use rand::Rng;

/**
 * A few constants that are used throughout the code
 */
const PIECE_COLOR: sdl2::pixels::Color = Color::RGB(255, 255, 255);
const BOARD_COLOR: sdl2::pixels::Color = Color::RGB(0, 0, 0);

const BLK_SIZE: usize = 30;
const ROWS: usize = 24;
const COLS: usize = 13;


/**
 * For the actual falling pieces
 */
#[allow(dead_code)]
enum TetrominoPieces {
    PieceI,
    PieceJ,
    PieceL,
    PieceO,
    PieceS,
    PieceT,
    PieceZ,
}
struct Tetromino {
    buffer : [[bool; 5]; 5] ,
    x_pos  : usize          ,
    y_pos  : usize          ,
}
impl Tetromino {
    fn new(piece_type: TetrominoPieces) -> Self {
        match piece_type {
            TetrominoPieces::PieceI => Self {
                buffer: [[ false, false, false, false, false ]  ,
                         [ false, false, false, false, false ]  ,
                         [ false, true , true , true , true  ]  ,
                         [ false, false, false, false, false ]  ,
                         [ false, false, false, false, false ]] ,
                x_pos: 6,
                y_pos: 3,
            },
            TetrominoPieces::PieceJ => Self {
                buffer: [[ false, false, false, false, false ]  ,
                         [ false, false, true , false, false ]  ,
                         [ false, false, true , false, false ]  ,
                         [ false, true , true , false, false ]  ,
                         [ false, false, false, false, false ]] ,
                x_pos: 6,
                y_pos: 3,
            },
            TetrominoPieces::PieceL => Self {
                buffer: [[ false, false, false, false, false ]  ,
                         [ false, false, true , false, false ]  ,
                         [ false, false, true , false, false ]  ,
                         [ false, false, true , true , false ]  ,
                         [ false, false, false, false, false ]] ,
                x_pos: 6,
                y_pos: 3,
            },
            TetrominoPieces::PieceO => Self {
                buffer: [[ false, false, false, false, false ]  ,
                         [ false, true , true , false, false ]  ,
                         [ false, true , true , false, false ]  ,
                         [ false, false, false, false, false ]  ,
                         [ false, false, false, false, false ]] ,
                x_pos: 6,
                y_pos: 3,
            },
            TetrominoPieces::PieceS => Self {
                buffer: [[ false, false, false, false, false ]  ,
                         [ false, false, true , true , false ]  ,
                         [ false, true , true , false, false ]  ,
                         [ false, false, false, false, false ]  ,
                         [ false, false, false, false, false ]] ,
                x_pos: 6,
                y_pos: 4,
            },
            TetrominoPieces::PieceT => Self {
                buffer: [[ false, false, false, false, false ]  ,
                         [ false, false, true , false, false ]  ,
                         [ false, true , true , true , false ]  ,
                         [ false, false, false, false, false ]  ,
                         [ false, false, false, false, false ]] ,
                x_pos: 6,
                y_pos: 3,
            },
            TetrominoPieces::PieceZ => Self {
                buffer: [[ false, false, false, false, false ]  ,
                         [ false, true , true , false, false ]  ,
                         [ false, false, true , true , false ]  ,
                         [ false, false, false, false, false ]  ,
                         [ false, false, false, false, false ]] ,
                x_pos: 6,
                y_pos: 3,
            },
        }
    }
    fn show(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(PIECE_COLOR);
        for r in 0..5 { for c in 0..5 {

            /* Drawing off the edge of the canvas isn't a big deal */
            let x_pos = match (self.x_pos + c).checked_sub(2) {
                Some(n) => {  n  }
                None => { continue; }
            };
            let y_pos = match (self.y_pos + r).checked_sub(2) {
                Some(n) => {  n  }
                None => { continue; }
            };

            if self.buffer[r][c] {
                let rect = Rect::new(
                    ((x_pos * BLK_SIZE)).try_into().unwrap(),
                    ((y_pos * BLK_SIZE)).try_into().unwrap(),
                    BLK_SIZE.try_into().unwrap(),
                    BLK_SIZE.try_into().unwrap());
                canvas.fill_rect(rect).expect("Oopsie");
            }
        }}
        canvas.set_draw_color(BOARD_COLOR);
    }
    fn colliding(&self, board: &Board) -> bool {

        for r in 0..5 {
            for c in 0..5 {

                /* Only check the cell if it's used */
                if !self.buffer[r][c] {
                    continue;
                }

                /* Some boundary checking (top and left) */
                let x_pos = match (self.x_pos + c).checked_sub(2) {
                    Some(n) => { n }
                    None => { return true }
                };
                let y_pos = match (self.y_pos + r).checked_sub(2) {
                    Some(n) => { n }
                    None => { return true }
                };

                /* Some boundary checking (top and right) */
                if x_pos >= COLS || y_pos >= ROWS {
                    return true
                }

                /* Collision with other squares */
                if board.buffer[y_pos][x_pos] {
                    return true
                }
            }
        }

        false
    }
    fn rotate(&mut self, board: &Board) {

        /* Make a new piece in place of there the real piece is
         *   trying to move to do some checks */
        let mut temp = Tetromino {
            buffer : self.buffer ,
            x_pos  : self.x_pos  ,
            y_pos  : self.y_pos  ,
        };

        /* Basically just a transpose then reverse rows */
        for r in 0..5 { for c in 0..5 {
            temp.buffer[r][c] = self.buffer[c][4-r];
        }}

        /* If there is no collision, we can copy the buffer over
         *   which dictates where the new squares are */
        if !temp.colliding(&board) {
            self.buffer = temp.buffer;
        }
    }
    fn mv_left(&mut self, board: &Board) {

        /* It kinda sucks, but we also need to check this to make sure
         *   it doesn't go beneath zero here */
        let new_x_pos = match (self.x_pos).checked_sub(1) {
            Some(n) => { n }
            None => { return }
        };

        /* Make a new piece in place of there the real piece is
         *   trying to move to do some checks */
        let temp = Tetromino {
            buffer : self.buffer    ,
            x_pos  : new_x_pos      ,
            y_pos  : self.y_pos     ,
        };

        if !temp.colliding(&board) {
            self.x_pos -= 1;
        }
    }
    fn mv_right(&mut self, board: &Board) {

        /* Make a new piece in place of there the real piece is
         *   trying to move to do some checks */
        let temp = Tetromino {
            buffer : self.buffer    ,
            x_pos  : self.x_pos + 1 ,
            y_pos  : self.y_pos     ,
        };

        if !temp.colliding(&board) {
            self.x_pos += 1;
        }
    }
    fn mv_down(&mut self, board: &Board) -> bool {

        /* Make a new piece in place of there the real piece is
         *   trying to move to do some checks */
        let temp = Tetromino {
            buffer : self.buffer    ,
            x_pos  : self.x_pos     ,
            y_pos  : self.y_pos + 1 ,
        };

        return if !temp.colliding(&board) {
            self.y_pos += 1; 
            false
        } else {
            true
        }
    }
}


/**
 * The structure containing all the information about each tile
 *   on the board where the tetrominos will be falling
 */
struct Board {
    buffer: [[bool; COLS]; ROWS]
}
impl Board {
    fn new() -> Self {
        Self {
            buffer: [[false; COLS]; ROWS]
        }
    }
    fn show(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(PIECE_COLOR);
        for r in 0..ROWS { for c in 0..COLS {
            if self.buffer[r][c] {
                let rect = Rect::new(
                    (c * BLK_SIZE).try_into().unwrap(),
                    (r * BLK_SIZE).try_into().unwrap(),
                    BLK_SIZE.try_into().unwrap(),
                    BLK_SIZE.try_into().unwrap());
                canvas.fill_rect(rect).expect("Oopsie");
            }
        }}
        canvas.set_draw_color(BOARD_COLOR);
    }
    fn emplace(&mut self, piece: &Tetromino) {
        for r in 0..5 {
            for c in 0..5 {
                if piece.buffer[r][c] {
                    self.buffer[piece.y_pos + r - 2][piece.x_pos + c - 2] = true;
                }
            }
        }
    }
    fn check_rows(&mut self) {
        for row in (1..ROWS).rev() {
            while !self.buffer[row].contains(&false) {
                for next_row in (1..row+1).rev() {
                    self.buffer[next_row] = self.buffer[next_row-1];
                }
            }
        }
    }
}

/* Just for generating random pieces */
fn rand_piece() -> TetrominoPieces {
    let num = rand::thread_rng().gen_range(0, 8);
    match num {
        0 => TetrominoPieces::PieceI,
        1 => TetrominoPieces::PieceJ,
        2 => TetrominoPieces::PieceL,
        3 => TetrominoPieces::PieceO,
        4 => TetrominoPieces::PieceS,
        5 => TetrominoPieces::PieceT,
        _ => TetrominoPieces::PieceZ,
    }
}

fn main() {

    let mut piece = Tetromino::new(rand_piece());
    let mut board = Board::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Blocc", 
                                    (COLS * BLK_SIZE).try_into().unwrap(),
                                    (ROWS * BLK_SIZE).try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(BOARD_COLOR);
    canvas.clear();
    canvas.present();

    let mut slowdown = 0;
    'running: loop {

        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    piece.rotate(&mut board);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    piece.mv_left(&mut board);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    piece.mv_right(&mut board);
                },
                _ => {}
            }
        }

        /* Display all layed down pieces */
        board.show(&mut canvas);

        /* Drop and show the current piece */
        slowdown += 1;
        if slowdown % 10 == 0 {
            if piece.mv_down(&board) {
                board.emplace(&piece);
                board.check_rows();
                piece = Tetromino::new(rand_piece());
            }
        }
        piece.show(&mut canvas);

        thread::sleep(time::Duration::from_millis(30));
        canvas.present();
    }
}
