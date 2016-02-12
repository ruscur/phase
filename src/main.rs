extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2_image::LoadTexture;

use std::path::Path;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("phase", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap_or_else(|err| panic!("Couldn't make a window: {}", err));

    let mut renderer = window.renderer().build().unwrap();
    let texture = renderer.load_texture(
        &Path::new("/home/ruscur/Documents/phase/assets/png/hello.png"))
        .unwrap_or_else(|err| panic!("Failed to open texture: {}", err));

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    let rect = Rect::new(0, 0, SCREEN_WIDTH - 300, SCREEN_HEIGHT - 200).unwrap();
    renderer.copy(&texture, None, rect);
    renderer.present();

    let mut event_pump = sdl_context.event_pump()
        .unwrap_or_else(|err| panic!("Couldn't get event pump: {}", err));

    'running : loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
