extern crate sdl2;
extern crate rand;

mod cpu;
mod ram;
mod drivers;

use cpu::CPU;
use drivers::{Display, Input};
use ram::RAM;
use crate::drivers::Cartridge;


fn main() {
    let file = "D:\\rust_projects\\chip_8\\test.txt";
    println!("{}", file);
    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    let mut cpu = CPU::new();
    let mut ram = RAM::new();
    Cartridge::read(file, &mut ram);

    loop {
        if !cpu.tick(&mut ram, None, Some(&mut display)) {
            break;
        }
    }
    display.refresh();

    while let Ok(keypad) = input.poll(){
    }
}
