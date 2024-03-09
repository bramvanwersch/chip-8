extern crate sdl2;
extern crate rand;

mod cpu;
mod ram;
mod drivers;

use std::env;
use cpu::CPU;
use drivers::{Display, Input};
use ram::RAM;
use crate::drivers::Cartridge;


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = match args.get(1){
        Some(program) => program,
        None => panic!("No program file provided!")
    };
    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    let mut cpu = CPU::new();
    let mut ram = RAM::new();
    Cartridge::read(program, &mut ram);

    loop {
        if !cpu.tick(&mut ram, None, Some(&mut display)) {
            break;
        }
    }
    display.refresh();

    while let Ok(_keypad) = input.poll(){
    }
}
