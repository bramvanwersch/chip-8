use crate::cpu::CPU;
use crate::drivers::{Display, Input};
use crate::ram::RAM;

mod cpu;
mod ram;
mod drivers;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let display = Display::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    let mut cpu = CPU::new();
    let mut ram = RAM::new();

    while let Ok(keypad) = input.poll(){

    }
}
