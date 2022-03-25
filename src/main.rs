extern crate sdl2;

use array2d::Array2D;
use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: usize = 800;
const HEIGHT: usize = 640;
const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;
const TILE_SIZE: usize = 32;

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    Quit,
}

struct Player {
    x: i32,
    y: i32,
    velocity_x: f32,
    velocity_y: f32,
}

const PLAYER_WIDTH: u32 = 32;
const PLAYER_HEIGHT: u32 = 32;
const PLAYER_COLOR: Color = Color::RGB(255, 0, 0);

trait Sprite {
    fn draw(&self, canvas: &mut Canvas<Window>);
    fn change_velocity(&mut self, x: f32, y: f32);
    fn update_position(&mut self);
}

impl Sprite for Player {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let r = Rect::new(
            self.x + (PLAYER_WIDTH as i32 / 2),
            self.y - (PLAYER_HEIGHT as i32 / 2),
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );
        canvas.set_draw_color(PLAYER_COLOR);
        canvas.fill_rect(r).unwrap();
    }

    fn change_velocity(&mut self, x: f32, y: f32) {
        self.velocity_x = x;
        self.velocity_y = y;
        self.update_position();
    }

    fn update_position(&mut self) {
        self.x = (self.x as f32 + self.velocity_x).round() as i32;
        self.y = (self.y as f32 + self.velocity_y).round() as i32;
    }
}

#[derive(Clone, PartialEq)]
enum MapObject {
    Wall,
    Empty,
}

struct Cave {
    map: Array2D<MapObject>,
    front_column: usize,
    last_update: u32,
}

trait GameMap {
    fn new() -> Cave;
    fn draw(&self, canvas: &mut Canvas<Window>, time: u32);
    fn scroll(&mut self, time: u32);
}

const MAP_WIDTH: usize = WIDTH / TILE_SIZE;
const MAP_HEIGHT: usize = HEIGHT / TILE_SIZE;
const COLUMN_UPDATE_RATE: u32 = 1000;

impl GameMap for Cave {
    fn new() -> Cave {
        let mut map = Array2D::filled_with(MapObject::Empty, MAP_WIDTH + 1, MAP_HEIGHT);

        for col in 0..MAP_WIDTH + 1 {
            map[(col, 0)] = MapObject::Wall;
            map[(col, MAP_HEIGHT - 1)] = MapObject::Wall;
        }

        Cave {
            map,
            front_column: 0,
            last_update: 0,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, time: u32) {
        let mut left_pos: usize = 0;
        let slide_factor = (time - self.last_update) as f32 / COLUMN_UPDATE_RATE as f32;
        let slide_delta = (TILE_SIZE as f32 * slide_factor).round() as usize;

        let mut draw_column = |col| {
            for row in 0..MAP_HEIGHT {
                match self.map.get(col, row) {
                    None => {}
                    Some(map_obj) => {
                        if *map_obj == MapObject::Wall {
                            let r = Rect::new(
                                ((left_pos + 1) * TILE_SIZE - slide_delta) as i32,
                                (row * TILE_SIZE) as i32,
                                TILE_SIZE as u32,
                                TILE_SIZE as u32,
                            );
                            let c = match map_obj {
                                MapObject::Wall => Color::RGB(60, 60, 60),
                                _ => Color::RGB(0, 0, 0),
                            };
                            canvas.set_draw_color(c);
                            canvas.fill_rect(r).unwrap();
                        }
                    }
                }
            }
            left_pos += 1;
        };

        for col in self.front_column..MAP_WIDTH {
            draw_column(col);
        }
        if self.front_column > 1 {
            for col in 0..self.front_column - 1 {
                draw_column(col);
            }
        }
    }

    fn scroll(&mut self, time: u32) {
        if time - self.last_update < COLUMN_UPDATE_RATE {
            return;
        }
        self.last_update = time;

        let new_col: usize = if self.front_column > 1 {
            self.front_column - 1
        } else {
            MAP_WIDTH
        };

        self.map[(new_col, 0)] = MapObject::Wall;
        self.map[(new_col, MAP_HEIGHT - 1)] = MapObject::Wall;

        let stalactite: u8 = rand::random::<u8>() % 64u8;
        let stalagmite: u8 = rand::random::<u8>() % 64u8;

        if stalactite <= 8 && stalactite > 2 {
            for r in 1..stalactite as usize {
                self.map[(new_col, r)] = MapObject::Wall;
            }
        }

        if stalagmite <= 8 && stalagmite > 2 {
            for r in MAP_HEIGHT - stalagmite as usize..MAP_HEIGHT - 1 {
                self.map[(new_col, r)] = MapObject::Wall;
            }
        }

        if self.front_column >= MAP_WIDTH {
            self.front_column = 0;
        } else {
            self.front_column += 1;
        }
    }
}

fn update(time: u32, game_state: &mut GameState, player: &mut Player, cave: &mut Cave) {
    match *game_state {
        GameState::Playing => {
            player.update_position();
            cave.scroll(time);
        }
        _ => {}
    }
}

fn render(
    canvas: &mut Canvas<Window>,
    game_state: &mut GameState,
    player: &mut Player,
    cave: &mut Cave,
    time: u32,
) {
    canvas.clear();

    cave.draw(canvas, time);
    player.draw(canvas);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.present();
}

fn handle_events(event: sdl2::event::Event, game_state: &mut GameState, player: &mut Player) {
    match game_state {
        GameState::Menu => match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *game_state = GameState::Quit;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                *game_state = GameState::Playing;
            }
            _ => {}
        },
        GameState::Playing => match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *game_state = GameState::Menu;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                player.change_velocity(0.0, -1.8);
            }
            Event::KeyUp {
                keycode: Some(Keycode::Space),
                ..
            } => {
                player.change_velocity(0.0, 1.8);
            }
            _ => {}
        },
        GameState::Quit => {}
    }
}

fn game_loop(
    game_state: &mut GameState,
    player: &mut Player,
    cave: &mut Cave,
    canvas: &mut Canvas<Window>,
    timer: &mut sdl2::TimerSubsystem,
    events: &mut sdl2::EventPump,
) {
    loop {
        if *game_state == GameState::Quit {
            break;
        }

        let frame_start = timer.ticks();

        match events.poll_event() {
            Some(event) => {
                handle_events(event, game_state, player);
            }
            None => {}
        }

        update(frame_start, game_state, player, cave);
        render(canvas, game_state, player, cave, frame_start);

        let frame_time = timer.ticks() - frame_start;
        if frame_time < FRAME_DELAY {
            timer.delay(FRAME_DELAY - frame_time);
        } else {
            println!("Frame overrun");
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("Flappy Dragon", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    let mut game_state = GameState::Menu;
    let mut player = Player {
        x: 0,
        y: HEIGHT as i32 / 2,
        velocity_x: 0.0,
        velocity_y: 1.8,
    };

    let mut cave = Cave::new();

    game_loop(
        &mut game_state,
        &mut player,
        &mut cave,
        &mut canvas,
        &mut timer,
        &mut events,
    );
}
