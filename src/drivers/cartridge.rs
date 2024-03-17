use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use crate::ram::{RAM, RAM_OFFSET, RAM_SIZE};
use crate::interpreter::{Interpreter};

pub struct Cartridge {}

impl Cartridge {
    pub fn read(filename: &str, ram: &mut RAM) {
        let mut interpreter = Interpreter::new(ram);
        match read_lines(filename) {
            Ok(lines) => {
                for (index, line) in lines.flatten().enumerate(){
                    match interpreter.interpret_line(&line.trim()){
                        Ok(_) => (),
                        Err(message) => panic!("{} at line {} for {}", message, index + 1, line)
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
        interpreter.resolve_references();
        let mut compiled = File::create(filename.to_owned() + ".cmp").expect("Failed to create compiled file");
        let mut current = RAM_OFFSET;
        let mut value: u16;
        let mut val = String::new();
        while current < RAM_SIZE{
            value = ram.get_u16(current);
            val.push_str(format!("{:04X}\n", value).as_str());
            current += 2;
        }
        compiled.write_all(&val.as_bytes()).expect("Failed to write values to file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
