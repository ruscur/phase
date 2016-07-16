extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
        &Path::new("assets/png/arrow.png"))
        .unwrap_or_else(|err| panic!("Failed to open texture: {}", err));
    let mut event_pump = sdl_context.event_pump()
        .unwrap_or_else(|err| panic!("Couldn't get event pump: {}", err));

    let mut degrees: f64 = 0.0;
    let mut h_flip = false;
    let mut v_flip = false;

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
                Event::KeyDown {
                    keycode: Some(Keycode::A), ..
                } => {
                    degrees -= 60.0;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::D), ..
                } => {
                    degrees += 60.0;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Q), ..
                } => {
                    h_flip = true;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::W), ..
                } => {
                    h_flip = false;
                    v_flip = false;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::E), ..
                } => {
                    v_flip = true;
                },
                _ => { }
            }
        }
        renderer.set_draw_color(WHITE);

        renderer.clear();
        renderer.copy_ex(&texture, None, None, degrees, None,
            h_flip, v_flip).unwrap();
        renderer.present();

        if frame_start.elapsed() < frame_time {
                sleep(frame_time - frame_start.elapsed());
        }
    }
}
