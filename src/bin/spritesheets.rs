extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2_image::LoadTexture;
use sdl2::pixels::Color;

use std::path::Path;

const SCREEN_WIDTH:  u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WHITE: Color = Color::RGB(255, 255, 255);

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
        &Path::new("assets/png/sprites.png"))
        .unwrap_or_else(|err| panic!("Failed to open texture: {}", err));
    let mut event_pump = sdl_context.event_pump()
        .unwrap_or_else(|err| panic!("Couldn't get event pump: {}", err));

    let dots: Vec<Rect> = vec!(
        Rect::new(0, 0, 100, 100),
        Rect::new(100, 0, 100, 100),
        Rect::new(0, 100, 100, 100),
        Rect::new(100, 100, 100, 100));

    let dests: Vec<Rect> = vec!(
        Rect::new(0, 0, 100, 100),
        Rect::new((SCREEN_WIDTH - 100) as i32, 0, 100, 100),
        Rect::new(0, (SCREEN_HEIGHT - 100) as i32, 100, 100),
        Rect::new((SCREEN_WIDTH - 100) as i32, (SCREEN_HEIGHT - 100) as i32,
            100, 100));

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
                    renderer.set_draw_color(WHITE);
                    for i in 0..4 {
                        renderer.copy(&texture, Some(dots[i]), Some(dests[i]));
                    }
                    renderer.present();
                }
            }
        }
    }
}
