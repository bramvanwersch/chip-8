extern crate sdl2;

use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8 system", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        Display {
            canvas
        }
    }
}
