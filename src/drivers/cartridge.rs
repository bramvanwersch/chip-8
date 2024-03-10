use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;
use crate::ram::{RAM};
use crate::interpreter::{Interpreter};

pub struct Cartridge {}

pub struct Line<'a>{
    pub value: &'a str,
    pub line_nr: usize
}

impl Cartridge {
    pub fn read(filename: &str, ram: &mut RAM) {
        let mut interpreter = Interpreter::new(ram);
        match read_lines(filename) {
            Ok(lines) => {
                for (index, line) in lines.flatten().enumerate() {
                    let mut line_obj = Line::new(&line, index + 1);
                    interpreter.interpret_line(&mut line_obj)
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
    }
}

impl<'a> Line<'a>{
    pub fn new(value: &'a str, line_nr: usize) -> Self{
        Line{
            value,
            line_nr
        }
    }

    pub fn panic_message(&self) -> String{
        format!("at line {} for '{}'", self.line_nr, self.value)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
