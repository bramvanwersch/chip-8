use crate::ram::{RAM, RAM_OFFSET};

pub struct Interpreter{
    // TODO make reference with lifetime --> figure it out
    // ram: &RAM
    offset: usize
}

impl Interpreter{

    pub fn new(ram: &RAM) -> Self{
        Interpreter{
            // ram
            offset: 0
        }
    }

    pub fn interpret_line(&mut self, line: &str, ram: &mut RAM){
        if line.starts_with("#") {
            return;
        }
        let value = u16::from_str_radix(&line, 16).unwrap();
        ram.set_u16(RAM_OFFSET + self.offset, value);
        self.offset += 2;
    }
}