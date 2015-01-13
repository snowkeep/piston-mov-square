//MOVE SQUARE
#![feature(globs)] //can use foo::*;
#![feature(default_type_params)]

extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;
extern crate input;

use std::cmp::{max, min}; //use for edge behav
use std::cell::RefCell;

use sdl2_window::Sdl2Window as Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL::_3_2;

use piston::RenderArgs;

use graphics::{
    Context,
    Rectangle,
};


use event::{
    Event,
    Events,
    RenderEvent,
    UpdateEvent,
    PressEvent,
    ReleaseEvent,
    WindowSettings
};

use input::Button;

use input::keyboard::Key::{
    Up, Down, Left, Right,
    W, J

};


//for random jitter
use std::rand;
use std::rand::Rng;

//pub static GRID_HEIGHT: int = 5;
//pub static GRID_WIDTH: int = 5;

//pub static BLOCK_SIZE: int = 100;

//pub static WINDOW_HEIGHT: int = GRID_HEIGHT * BLOCK_SIZE;
//pub static WINDOW_WIDTH: int = GRID_WIDTH * BLOCK_SIZE;
//pub static WINDOW_HEIGHT: int = 500;
//pub static WINDOW_WIDTH: int = 500;
const GRID_HEIGHT: int = 5;
const GRID_WIDTH: int = 5;

const BLOCK_SIZE: int = 100;

const WINDOW_HEIGHT: int = GRID_HEIGHT * BLOCK_SIZE;
const WINDOW_WIDTH: int = GRID_WIDTH * BLOCK_SIZE;

enum Direction {
    UpDir,
    DownDir,
    LeftDir,
    RightDir,
    Stop
}

struct GameState {
    gl: Gl,
    pub x: int, pub y: int,
    pub max_x: int, pub max_y: int,

    pub edge_behav: bool, //false-stop, true-wrap
    pub jitter_behav: bool, //true-jitters
    pub next_mov: Direction, //direction of movement in the next tick. Stop means no mov

    jitter_counter: uint,
    slide_counter: uint
}

impl GameState {
    pub fn new(gl: Gl, x: int, y: int, max_x: int, max_y: int, edge_behav: bool, jitter_behav: bool) -> GameState {
        GameState {
            gl: gl,
            x: x,
            y: y,
            max_x: max_x,
            max_y: max_y,
            edge_behav: edge_behav,
            jitter_behav: jitter_behav,
            next_mov: Direction::Stop,
            jitter_counter: 11,
            slide_counter: 11
        }
    }

    pub fn mov(&mut self, x: int, y: int) {
        match self.edge_behav {
            //stopping behavior. `self.max_x - 1` because range is (0, len-1)
            false => { self.x = min(max(self.x + x, 0), self.max_x - 1);
                       self.y = min(max(self.y + y, 0), self.max_y - 1);
            },
            //wrapping behavior
            true => {
                self.x += x;
                if self.x > self.max_x - 1 {self.x = 0}
                else if self.x < 0 {self.x = self.max_x - 1};
                self.y += y;
                if self.y > self.max_y - 1 {self.y = 0}
                else if self.y < 0 {self.y = self.max_x - 1};
            }
        }
    }
    // flip stopping/wrapping behavior
    pub fn change_edge_behav(&mut self) {self.edge_behav = !self.edge_behav}
    //start/stop jittering
    pub fn change_jitter_behav(&mut self) {self.jitter_behav = !self.jitter_behav}

    pub fn jitter(&mut self) {
        if self.jitter_behav {
            let mut rng = rand::thread_rng();
            let r = rng.gen::<uint>() % 4; // %4 trick to get range 0-3
            match r {
                0 => {self.mov(1, 0)},
                1 => {self.mov(-1, 0)},
                2 => {self.mov(0, 1)},
                3 => {self.mov(0, -1)},
                _ => {}
            }
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        let c = &Context::abs(args.width as f64, args.height as f64);
        graphics::clear(graphics::color::WHITE, &mut self.gl);
        Rectangle::border([1.0, 0.0, 0.0, 1.0], 10.0).draw([
            (self.x * BLOCK_SIZE) as f64,
            (self.y * BLOCK_SIZE) as f64,
            BLOCK_SIZE as f64,
            BLOCK_SIZE as f64
        ], c, &mut self.gl);
    }
    fn update(&mut self) {
        self.jitter_counter += 1;
        if self.jitter_counter == 12 {self.jitter_counter = 0; self.jitter()};

        self.slide_counter += 1;
        if self.slide_counter == 12 {
            self.slide_counter = 0;
            match self.next_mov {
                Direction::UpDir => {self.mov(0, -1)},
                Direction::DownDir => {self.mov(0, 1)},
                Direction::LeftDir => {self.mov(-1, 0)},
                Direction::RightDir => {self.mov(1,0)},
                _ => {}
            }
        }
    }

}

fn main() {
    let window = Window::new(
        shader_version::OpenGL::_3_2,
        WindowSettings {
            title: "moving square".to_string(),
            size: [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0
        }
    );

    let window = RefCell::new(window);

    let mut game = GameState::new(Gl::new(_3_2), GRID_WIDTH/2, GRID_HEIGHT/2, GRID_WIDTH, GRID_HEIGHT, false, false);

    for e in Events::new(&window) {
        let e: Event<input::Input> = e;
        e.press(|button| {
            match button {
                Button::Keyboard(key) => {
                    match key {
                        Up => {game.next_mov = Direction::UpDir},
                        Down => {game.next_mov = Direction::DownDir},
                        Left => {game.next_mov = Direction::LeftDir},
                        Right => {game.next_mov = Direction::RightDir},
                        W => {game.change_edge_behav()},
                        J => {game.change_jitter_behav()},
                        _ => {}
                    }
                },
                _ => {}
            }
        });
        e.release(|button| {
            match button {
               Button::Keyboard(key) => {
                    match key {
                          Up | Down | Left | Right => {game.next_mov = Direction::Stop},
                          _ => {}
                    }
                },
                _ => {}
            }
        });
        e.render(|r| game.render(r));
        e.update(|_| game.update());
    }
}
