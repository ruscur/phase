extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2_image::LoadTexture;

use std::path::Path;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WHITE: Color = Color::RGB(255, 255, 255);
const RED:   Color = Color::RGB(255, 0, 0);
const GREEN: Color = Color::RGB(0, 255, 0);
const BLUE:  Color = Color::RGB(0, 0, 255);

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
    let image_rect = Rect::new(0, 0, SCREEN_WIDTH - 300, SCREEN_HEIGHT - 200);
    let red_rect = Rect::new((SCREEN_WIDTH/4) as i32, (SCREEN_HEIGHT/4) as i32,
                             SCREEN_WIDTH/2, SCREEN_HEIGHT/2);

    let mut event_pump = sdl_context.event_pump()
        .unwrap_or_else(|err| panic!("Couldn't get event pump: {}", err));

    'running : loop { for event in event_pump.poll_iter() { match event {
        Event::Quit {..} |
        Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            break 'running
        },
        _ => {
            renderer.set_draw_color(WHITE);
            renderer.clear();
            renderer.copy(&texture, None, Some(image_rect));
            renderer.set_draw_color(RED);
            renderer.fill_rect(red_rect).unwrap();
            renderer.set_draw_color(BLUE);
            renderer.draw_line(Point::new(0, (SCREEN_HEIGHT/2) as i32),
                Point::new(SCREEN_WIDTH as i32,
                    (SCREEN_HEIGHT/2) as i32)).unwrap();
            renderer.set_draw_color(GREEN);
            for i in 0..SCREEN_HEIGHT {
                renderer.draw_point(
                    Point::new((SCREEN_WIDTH/2) as i32, (i*4) as i32)).unwrap();
            }
            renderer.present();
        }
    }}}
}
