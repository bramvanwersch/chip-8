extern crate sdl2;

use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;


const PIXEL_WIDTH: usize = 64;
const PIXEL_HEIGHT: usize = 32;
const SCALE: u32 = 20;
const WIDTH: u32 = (PIXEL_WIDTH as u32) * SCALE;
const HEIGHT: u32 = (PIXEL_HEIGHT as u32) * SCALE;
const COLORS: [pixels::Color; 2] = [
    pixels::Color::RGB(0, 0, 0),
    pixels::Color::RGB(0, 250, 0)
];


pub struct Display {
    canvas: Canvas<Window>,
    v_ram: [u8; PIXEL_WIDTH * PIXEL_HEIGHT]
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8 system", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let v_ram = [0; PIXEL_WIDTH * PIXEL_HEIGHT];
        Display {
            canvas,
            v_ram
        }
    }

    pub fn clear_display(&mut self){
        for index in 0..self.v_ram.len(){
            self.v_ram[index] = 0;
        }
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw(&mut self, x: u8, y: u8, value: u8){
        for bit in 0..8{
            // 1 or 0
            let color = (value >> (7 - bit)) & 1;
            self.v_ram[PIXEL_WIDTH * y as usize + x as usize + bit] ^= color;
        }
    }

    pub fn refresh(&mut self){
        for x in 0..PIXEL_WIDTH{
            for y in 0..PIXEL_HEIGHT{
                let color = self.v_ram[PIXEL_WIDTH * y + x];
                self.canvas.set_draw_color(self.get_color(color));
                let _ = self.canvas.fill_rect(Rect::new(x as i32 * SCALE as i32, y as i32 * SCALE as i32, SCALE, SCALE));
            }
        }
        self.canvas.present();
    }

    fn get_color(&self, value: u8) -> pixels::Color {
        COLORS[value as usize]
    }
}
