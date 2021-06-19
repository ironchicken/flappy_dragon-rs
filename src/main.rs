extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 640;
const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;

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
        let r = sdl2::rect::Rect::new(
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

fn update(game_state: &mut GameState, player: &mut Player) {
    match *game_state {
        GameState::Playing => {
            player.update_position();
        }
        _ => {}
    }
}

fn render(canvas: &mut Canvas<Window>, game_state: &mut GameState, player: &mut Player) {
    canvas.clear();

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

        update(game_state, player);
        render(canvas, game_state, player);

        let frame_time = timer.ticks() - frame_start;
        if frame_time < FRAME_DELAY {
            timer.delay(FRAME_DELAY - frame_time);
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("Flappy Dragon", WIDTH, HEIGHT)
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

    game_loop(
        &mut game_state,
        &mut player,
        &mut canvas,
        &mut timer,
        &mut events,
    );
}
