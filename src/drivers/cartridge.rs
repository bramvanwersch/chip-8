use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;
use crate::ram::{RAM, RAM_OFFSET};

pub struct Cartridge{
}

impl Cartridge{

    pub fn read(filename: &str, ram: &mut RAM){
        match read_lines(filename) {
            Ok(lines) => {
                for (index, line) in lines.flatten().enumerate() {
                    let value = u16::from_str_radix(&line, 16).unwrap();
                    ram.set_u16(RAM_OFFSET + index * 2, value);
                }
            },
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
