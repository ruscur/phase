extern crate sdl2;
extern crate sdl2_image;
#[macro_use]
extern crate phase;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2_image::LoadTexture;
use sdl2::pixels::Color;

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

const SCREEN_WIDTH:  u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WHITE: Color = Color::RGB(255, 255, 255);

const FRAME_RATE: u64 = 60;

fn main() {
    let frame_time: Duration = Duration::from_millis(1000/FRAME_RATE);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("phase", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap_or_else(|err| panic!("Couldn't make a window: {}", err));

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let texture = renderer.load_texture(
        &Path::new("assets/png/foo.png"))
        .unwrap_or_else(|err| panic!("Failed to open texture: {}", err));
    let mut event_pump = sdl_context.event_pump()
        .unwrap_or_else(|err| panic!("Couldn't get event pump: {}", err));

    let frames: Vec<Rect> = vec!(
        rect!(0, 0, 64, 205),
        rect!(64, 0, 64, 205),
        rect!(128, 0, 64, 205),
        rect!(196, 0, 64, 205));

    let mut frame_count = 0;

    'running : loop {
        let frame_start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running
                },
                _ => { }
            }
        }
        renderer.set_draw_color(WHITE);

        renderer.clear();
        renderer.copy(&texture, Some(frames[frame_count/4]), None);
        renderer.present();

        frame_count += 1;
        if (frame_count / 4) >= 4 {
            frame_count = 0;
        }

        if frame_start.elapsed() < frame_time {
                sleep(frame_time - frame_start.elapsed());
        }
    }
}
