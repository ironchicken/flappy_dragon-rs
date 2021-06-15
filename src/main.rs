extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;

fn game_loop(mut timer: sdl2::TimerSubsystem, canvas: &Canvas<Window>) {
    loop {
	let frame_start = timer.ticks();

	let frame_time = timer.ticks() - frame_start;
	if frame_time < FRAME_DELAY {
	    timer.delay(FRAME_DELAY - frame_time);
	}
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let timer = sdl_context.timer().unwrap();

    let window = video.window("Flappy Dragon", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    game_loop(timer, &canvas);
}
