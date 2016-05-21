extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
    let mut texture = renderer.load_texture(
        &Path::new("assets/png/colors.png"))
        .unwrap_or_else(|err| panic!("Failed to open texture: {}", err));
    let mut event_pump = sdl_context.event_pump()
        .unwrap_or_else(|err| panic!("Couldn't get event pump: {}", err));

    let mut r = 255u8;
    let mut g = 255u8;
    let mut b = 255u8;

    'running : loop { for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown {
                keycode: Some(Keycode::Escape), ..
            } => {
                break 'running
            },
            Event::KeyDown {
                keycode: Some(Keycode::Q), ..
            } => {
                if r < 32 { r = 0; }
                else { r -= 32 };
            },
            Event::KeyDown {
                keycode: Some(Keycode::W), ..
            } => {
                if g < 32 { g = 0; }
                else { g -= 32 };
            },
            Event::KeyDown {
                keycode: Some(Keycode::E), ..
            } => {
                if b < 32 { b = 0; }
                else { b -= 32 };
            },
            Event::KeyDown {
                keycode: Some(Keycode::A), ..
            } => {
                if r > 223 { r = 255; }
                else { r += 32 };
            },
            Event::KeyDown {
                keycode: Some(Keycode::S), ..
            } => {
                if g > 223 { g = 255; }
                else { g += 32 };
            },
            Event::KeyDown {
                keycode: Some(Keycode::D), ..
            } => {
                if b > 223 { b = 255; }
                else { b += 32 };
            },
            _ => { }
        }
        renderer.clear();
        texture.set_color_mod(r, g, b);
        renderer.copy(&texture, None, None);

        renderer.present();
    }}
}
