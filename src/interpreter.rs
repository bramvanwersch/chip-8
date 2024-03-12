use std::collections::HashMap;
use crate::drivers::Line;
use crate::ram::{RAM, RAM_OFFSET};

pub struct Interpreter<'a> {
    // TODO see if we can escalate a panic up
    // TODO: make sure to place functions in memory
    ram: &'a mut RAM,
    offset: usize,
    current_definition: Option<String>,
    definition_map: HashMap<String, Vec<u16>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(ram: &'a mut RAM) -> Self {
        let definition_map = HashMap::new();
        Interpreter {
            ram,
            offset: 0,
            current_definition: None,
            definition_map,
        }
    }

    pub fn interpret_line(&mut self, line: &mut Line) {
        // ignore empty lines
        if line.value.eq("") {
            return;
        }
        // ignore comment lines
        if line.value.starts_with("//") {
            return;
        }
        let values: Vec<&str> = line.value.split(" ").collect();
        // ignore empty lines
        let command = match values.get(0) {
            Some(c) => *c,
            // line empty return --> probably does not work as expected
            None => return
        };
        if command.starts_with("#") {
            // is a special command like a function definition
            self.read_special_command(command, &values);
            return;
        }
        match command {
            "EXT" => self.add_instruction(0x0000), // exit
            "CLD" => self.add_instruction(0x000E), // clear display
            "RET" => self.add_ret_instruction(0x00EE), // return subroutine
            "JMP" => self.add_nnn_instruction(0x1, &values, line), // jump to memory location
            "CLL" => self.add_nnn_instruction(0x2, &values, line), // call subroutine at
            "SEV" => self.add_x_kk_instruction(0x3, &values, line), // Skip instruction if register x and kk are equal
            "SNEV" => self.add_x_kk_instruction(0x4, &values, line), // skip instruction if register x and kk are not equal
            "SER" => self.add_x_y_instruction(0x5, &values, 0x0, line), // skip instruction if register x and y are equal
            "STV" => self.add_x_kk_instruction(0x6, &values, line), // set kk into register x
            "ADDV" => self.add_x_kk_instruction(0x7, &values, line), // add kk to the value in register x
            "STR" => self.add_x_y_instruction(0x8, &values, 0x0, line), // set the value of register y into register x
            "OR" => self.add_x_y_instruction(0x8, &values, 0x1, line), // or the values of register y into x
            "AND" => self.add_x_y_instruction(0x8, &values, 0x2, line), // and the values of register y into x
            "XOR" => self.add_x_y_instruction(0x8, &values, 0x3, line), // xor the values of register y into x
            "ADD" => self.add_x_y_instruction(0x8, &values, 0x4, line), // add the values of register y into x
            "SUB" => self.add_x_y_instruction(0x8, &values, 0x5, line), // subtract the values of register y into x
            "RSH" => self.add_x_instruction(0x8, &values, 0x06, line), // right shift the register value of x
            "SUBR" => self.add_x_y_instruction(0x8, &values, 0x7, line), // subtract the values of register x from y and store in x
            "LSH" => self.add_x_instruction(0x8, &values, 0x0E, line), // left shift the register value of x
            "SNER" => self.add_x_y_instruction(0x9, &values, 0x0, line), // skip instruction if register x and y are not equal
            "STI" => self.add_nnn_instruction(0xA, &values, line), // set the value of register i
            "JMPR" => self.add_nnn_instruction(0xB, &values, line), // jump to location nnn plus the value of register 0
            "RND" => self.add_x_kk_instruction(0xC, &values, line), // get a random byte and AND with kk
            "DRW" => self.add_x_y_d_instruction(0xD, &values, line), // draw at coordinate of registers x, y for height d
            "SEP" => self.add_x_instruction(0xE, &values, 0x9E, line), // skip instruction of key is pressed
            "SENP" => self.add_x_instruction(0xE, &values, 0xAE, line),  // skip instruction if key is not pressed
            "STRD" => self.add_x_instruction(0xF, &values, 0x07, line), // set the value of register x to the remaining delay
            "WTP" => self.add_x_instruction(0xF, &values, 0x0A, line), // halt execution until a key is pressed. This key is stored in register x
            "STDR" => self.add_x_instruction(0xF, &values, 0x15, line), // set the delay to the value in register x
            "STRS" => self.add_x_instruction(0xF, &values, 0x18, line), // set the sound to value in register x
            "ADDI" => self.add_x_instruction(0xF, &values, 0x1E, line), // add the value in register x to register i
            "STIS" => self.add_x_instruction(0xF, &values, 0x29, line), // set register i to point to the memory where the sprite for value x is stored
            "BCD" => self.add_x_instruction(0xF, &values, 0x33, line), // set the bcd
            "CTR" => self.add_x_instruction(0xF, &values, 0x55, line), // copy values from register 0 to register x into ram starting at address i
            "CFR" => self.add_x_instruction(0xF, &values, 0x65, line), // copy values from ram into registers 0 to x starting at address i
            _ => { panic!("Invalid instruction {}", command); }
        }
    }

    fn read_special_command(&mut self, command: &str, values: &Vec<&str>) {
        match command {
            "#f" => {
                let name = match values.get(1){
                    Some(v) => v,
                    None => panic!("You need to provide a name when defining a function")
                };
                self.definition_map.insert(name.to_string(), Vec::new());
                self.current_definition = Some(name.to_string());
            }
            _ => panic!("Syntax error unknown special command {}", command)
        }
    }

    fn add_instruction(&mut self, instruction: u16) {
        match &self.current_definition {
            Some(name) => {
                match self.definition_map.get_mut(name){
                    Some(vec) => vec.push(instruction),
                    None => panic!("Failed to find current definition")
                }
            },
            None => {
                self.ram.set_u16(RAM_OFFSET + self.offset, instruction);
                self.offset += 2;
            }
        }
    }

    fn add_ret_instruction(&mut self, instruction: u16){
        // make sure to close the current definition
        self.ram.set_u16(RAM_OFFSET + self.offset, instruction);
        self.offset += 2;
        self.current_definition = None;
    }

    fn add_x_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, end_instruction: u16, current_line: &Line) {
        let x = self.get_u16_value(values, 1, current_line);
        let instruction = start_instruction << 12 | x << 8 | end_instruction;
        self.add_instruction(instruction);
    }

    fn add_x_y_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, end_instruction: u16, current_line: &Line) {
        let x = self.get_u16_value(values, 1, current_line);
        let y = self.get_u16_value(values, 2, current_line);
        let instruction = start_instruction << 12 | x << 8 | y << 4 | end_instruction;
        self.add_instruction(instruction);
    }

    fn add_x_y_d_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, current_line: &Line) {
        let x = self.get_u16_value(values, 1, current_line);
        let y = self.get_u16_value(values, 2, current_line);
        let d = self.get_u16_value(values, 3, current_line);
        let instruction = start_instruction << 12 | x << 8 | y << 4 | d;
        self.add_instruction(instruction);
    }

    fn add_x_kk_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, current_line: &Line) {
        let x = self.get_u16_value(values, 1, current_line);
        let kk = self.get_u16_value(values, 2, current_line);
        let instruction = start_instruction << 12 | x << 8 | kk;
        self.add_instruction(instruction)
    }

    fn add_nnn_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, current_line: &Line) {
        let nnn = self.get_u16_value(values, 1, current_line);
        let instruction = start_instruction << 12 | nnn;
        self.add_instruction(instruction);
    }

    fn get_u16_value(&self, values: &Vec<&str>, index: usize, current_line: &Line) -> u16 {
        let value = match values.get(index) {
            Some(c) => *c,
            // line empty return --> probably does not work as expected
            None => panic!("Incomplete instruction {}", current_line.panic_message())
        };
        u16::from_str_radix(value, 16).expect(format!("Invalid value for index {} {}", index, current_line.panic_message()).as_str())
    }
}


#[cfg(test)]
mod tests {
    use crate::drivers::Line;
    use crate::interpreter::Interpreter;
    use crate::ram::{RAM, RAM_OFFSET};

    #[test]
    fn test_ignore_comment_line() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        let mut line = Line::new("// some data", 1);
        interpreter.interpret_line(&mut line);
        assert_eq!(interpreter.offset, 0);
        assert_eq!(ram.get(RAM_OFFSET), 0);
    }

    #[test]
    fn test_add_x_instruction() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        values.push("A");
        let line = Line::new("test", 1);
        interpreter.add_x_instruction(0xF, &values, 0x29, &line);
        assert_eq!(interpreter.offset, 2);
        assert_eq!(ram.get(RAM_OFFSET), 0xFA);
        assert_eq!(ram.get(RAM_OFFSET + 1), 0x29);
    }

    #[test]
    fn get_u16_value() {
        let mut ram = RAM::new();
        let interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        values.push("A");
        let line = Line::new("test", 1);
        let result = interpreter.get_u16_value(&values, 1, &line);
        assert_eq!(result, 0xA);
    }

    #[test]
    #[should_panic(expected = "Incomplete instruction at line 1 for 'test'")]
    fn get_u16_value_fail1() {
        let mut ram = RAM::new();
        let interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        let line = Line::new("test", 1);
        interpreter.get_u16_value(&values, 1, &line);
    }

    #[test]
    #[should_panic(expected = "Invalid value for index 1 at line 1 for 'test'")]
    fn get_u16_value_fail2() {
        let mut ram = RAM::new();
        let interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        values.push("G");
        let line = Line::new("test", 1);
        interpreter.get_u16_value(&values, 1, &line);
    }

    #[test]
    fn test_interpret_ext() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        let mut line = Line::new("EXT", 1);
        interpreter.interpret_line(&mut line);
        assert_eq!(interpreter.offset, 2);
        assert_eq!(ram.get(RAM_OFFSET), 0x00);
        assert_eq!(ram.get(RAM_OFFSET + 1), 0x00);
    }

    #[test]
    fn test_interpret_stis() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        let mut line = Line::new("STIS 1", 1);
        interpreter.interpret_line(&mut line);
        assert_eq!(interpreter.offset, 2);
        assert_eq!(ram.get(RAM_OFFSET), 0xF1);
        assert_eq!(ram.get(RAM_OFFSET + 1), 0x29);
    }
}