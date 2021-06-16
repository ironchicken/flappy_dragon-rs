extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    Quit,
}

struct Player {
    x: u32,
    y: u32,
}

fn update(game_state: &mut GameState, player: &mut Player) {
}

fn render(canvas: &mut Canvas<Window>, game_state: &mut GameState, player: &mut Player) {
}

fn handle_events(event: sdl2::event::Event, game_state: &mut GameState, player: &mut Player) {
    match game_state {
	GameState::Menu => {
	    match event {
		Event::Quit {..} |
		Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
		    *game_state = GameState::Quit;
		},
		_ => {},
	    }
	},
	GameState::Playing => {},
	GameState::Quit => {},
    }
}

fn game_loop(sdl_context: sdl2::Sdl, canvas: &mut Canvas<Window>) {
    let mut timer = sdl_context.timer().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    let mut game_state = GameState::Menu;
    let mut player = Player { x: 0, y: WIDTH / 2 };

    loop {
	if game_state == GameState::Quit {
	    break;
	}

	let frame_start = timer.ticks();

	match events.poll_event() {
	    Some(event) => {
		handle_events(event, &mut game_state, &mut player);
	    }
	    None => {},
	}

	update(&mut game_state, &mut player);
	render(canvas, &mut game_state, &mut player);

	let frame_time = timer.ticks() - frame_start;
	if frame_time < FRAME_DELAY {
	    timer.delay(FRAME_DELAY - frame_time);
	}
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("Flappy Dragon", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    game_loop(sdl_context, &mut canvas);
}
