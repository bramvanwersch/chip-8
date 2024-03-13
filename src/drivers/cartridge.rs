use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;
use crate::ram::{RAM};
use crate::interpreter::{Interpreter};

pub struct Cartridge {}

impl Cartridge {
    pub fn read(filename: &str, ram: &mut RAM) {
        let mut interpreter = Interpreter::new(ram);
        match read_lines(filename) {
            Ok(lines) => {
                for (index, line) in lines.flatten().enumerate() {
                    match interpreter.interpret_line(&mut line.as_str()){
                        Ok(_) => (),
                        Err(message) => panic!("{} at line {} for {}", message, index + 1, line)
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
