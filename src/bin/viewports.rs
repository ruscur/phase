extern crate sdl2;
extern crate sdl2_image;
#[macro_use]
extern crate phase;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2_image::LoadTexture;

use std::path::Path;

const SCREEN_WIDTH:  u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("phase", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap_or_else(|err| panic!("Couldn't make a window: {}", err));

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let texture = renderer.load_texture(
        &Path::new("assets/png/hello.png"))
        .unwrap_or_else(|err| panic!("Failed to open texture: {}", err));
    let mut event_pump = sdl_context.event_pump()
        .unwrap_or_else(|err| panic!("Couldn't get event pump: {}", err));

    let top_left_viewport = rect!(0, 0, SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
    let top_right_viewport = rect!(SCREEN_WIDTH/2, 0,
                                   SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
    let bottom_viewport = rect!(0, SCREEN_HEIGHT/2,
                                SCREEN_WIDTH, SCREEN_HEIGHT/2);

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
                _ => {
                    renderer.clear();
                    renderer.set_viewport(Some(top_left_viewport));
                    renderer.copy(&texture, None, None);
                    renderer.set_viewport(Some(top_right_viewport));
                    renderer.copy(&texture, None, None);
                    renderer.set_viewport(Some(bottom_viewport));
                    renderer.copy(&texture, None, None);
                    renderer.set_viewport(None);
                    renderer.present();
                }
            }
        }
    }
}
