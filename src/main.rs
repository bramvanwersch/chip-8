extern crate sdl2;
extern crate rand;

mod cpu;
mod ram;
mod drivers;

use cpu::CPU;
use drivers::{Display, Input};
use ram::RAM;



fn main() {
    let sdl_context = sdl2::init().unwrap();
    let display = Display::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    let mut cpu = CPU::new();
    let mut ram = RAM::new();

    while let Ok(keypad) = input.poll(){
    }
}
