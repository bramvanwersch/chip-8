use std::collections::HashMap;
use crate::ram::{RAM, RAM_OFFSET};

pub struct Interpreter<'a> {
    // TODO: make sure to place functions in memory
    ram: &'a mut RAM,
    offset: usize,
    current_function: Option<String>,
    references: HashMap<String, Vec<usize>>,
    definition_map: HashMap<String, Vec<u16>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(ram: &'a mut RAM) -> Self {
        let definition_map = HashMap::new();
        let references = HashMap::new();
        Interpreter {
            ram,
            offset: RAM_OFFSET,
            current_function: None,
            references,
            definition_map,
        }
    }

    pub fn interpret_line(&mut self, line: &str) -> Result<bool, String> {
        // interpret a line and return an optional issue
        // ignore empty lines
        if line.eq("") {
            return Ok(true);
        }
        // ignore comment lines
        if line.starts_with("//") {
            return Ok(true);
        }
        let values: Vec<&str> = line.split(" ").collect();
        let command = match values.get(0) {
            Some(c) => *c,
            None => return Ok(true)
        };
        if command.starts_with("#") {
            // is a special command like a function definition
            return self.read_special_command(command, &values);
        }
        match command {
            "EXT" => self.add_instruction(0x0000), // exit
            "CLD" => self.add_instruction(0x000E), // clear display
            "RET" => self.add_ret_instruction(0x00EE), // return subroutine
            "JMP" => self.add_nnn_instruction(0x1, &values), // jump to memory location
            "CLL" => self.add_nnn_instruction(0x2, &values), // call subroutine at
            "SEV" => self.add_x_kk_instruction(0x3, &values), // Skip instruction if register x and kk are equal
            "SNEV" => self.add_x_kk_instruction(0x4, &values), // skip instruction if register x and kk are not equal
            "SER" => self.add_x_y_instruction(0x5, &values, 0x0), // skip instruction if register x and y are equal
            "STV" => self.add_x_kk_instruction(0x6, &values), // set kk into register x
            "ADDV" => self.add_x_kk_instruction(0x7, &values), // add kk to the value in register x
            "STR" => self.add_x_y_instruction(0x8, &values, 0x0), // set the value of register y into register x
            "OR" => self.add_x_y_instruction(0x8, &values, 0x1), // or the values of register y into x
            "AND" => self.add_x_y_instruction(0x8, &values, 0x2), // and the values of register y into x
            "XOR" => self.add_x_y_instruction(0x8, &values, 0x3), // xor the values of register y into x
            "ADD" => self.add_x_y_instruction(0x8, &values, 0x4), // add the values of register y into x
            "SUB" => self.add_x_y_instruction(0x8, &values, 0x5), // subtract the values of register y into x
            "RSH" => self.add_x_instruction(0x8, &values, 0x06), // right shift the register value of x
            "SUBR" => self.add_x_y_instruction(0x8, &values, 0x7), // subtract the values of register x from y and store in x
            "LSH" => self.add_x_instruction(0x8, &values, 0x0E), // left shift the register value of x
            "SNER" => self.add_x_y_instruction(0x9, &values, 0x0), // skip instruction if register x and y are not equal
            "STI" => self.add_nnn_instruction(0xA, &values), // set the value of register i
            "JMPR" => self.add_nnn_instruction(0xB, &values), // jump to location nnn plus the value of register 0
            "RND" => self.add_x_kk_instruction(0xC, &values), // get a random byte and AND with kk
            "DRW" => self.add_x_y_d_instruction(0xD, &values), // draw at coordinate of registers x, y for height d
            "SEP" => self.add_x_instruction(0xE, &values, 0x9E), // skip instruction of key is pressed
            "SENP" => self.add_x_instruction(0xE, &values, 0xAE),  // skip instruction if key is not pressed
            "STRD" => self.add_x_instruction(0xF, &values, 0x07), // set the value of register x to the remaining delay
            "WTP" => self.add_x_instruction(0xF, &values, 0x0A), // halt execution until a key is pressed. This key is stored in register x
            "STDR" => self.add_x_instruction(0xF, &values, 0x15), // set the delay to the value in register x
            "STRS" => self.add_x_instruction(0xF, &values, 0x18), // set the sound to value in register x
            "ADDI" => self.add_x_instruction(0xF, &values, 0x1E), // add the value in register x to register i
            "STIS" => self.add_x_instruction(0xF, &values, 0x29), // set register i to point to the memory where the sprite for value x is stored
            "BCD" => self.add_x_instruction(0xF, &values, 0x33), // set the bcd
            "CTR" => self.add_x_instruction(0xF, &values, 0x55), // copy values from register 0 to register x into ram starting at address i
            "CFR" => self.add_x_instruction(0xF, &values, 0x65), // copy values from ram into registers 0 to x starting at address i
            _ => self.set_reference(command, &values)
        }
    }

    fn read_special_command(&mut self, command: &str, values: &Vec<&str>) -> Result<bool, String> {
        match command {
            "#f" => {
                if self.current_function != None {
                    return Err("Cannot start a function in another function".to_string());
                }
                let name = match values.get(1) {
                    Some(v) => *v,
                    None => return Err("You need to provide a name when defining a function".to_string())
                };
                if self.definition_map.get(name).is_some() {
                    return Err(format!("Redeclared function name {}", name));
                }
                self.definition_map.insert(name.to_string(), Vec::new());
                self.current_function = Some(name.to_string());
                Ok(true)
            }
            _ => Err(format!("Syntax error unknown special command {}", command))
        }
    }

    pub fn resolve_references(&mut self) {
        // don't love this clone
        for (key, instructions) in self.definition_map.clone().iter() {
            let places = match self.references.get(key) {
                Some(v) => v,
                None => continue
            };
            let start_instruction = instructions.get(0).unwrap();
            for place in places {
                // call subroutine at correct place
                let full_instruction = 0x1u16 << 12 | *start_instruction;
                self.ram.set_u16(*place, full_instruction);
            }
            for value in instructions {
                self.add_instruction(*value).unwrap();
            }
        }
    }

    fn set_reference(&mut self, command: &str, _values: &Vec<&str>) -> Result<bool, String> {
        if !self.definition_map.get(command).is_some() {
            return Err(format!("Invalid instruction {}", command));
        }
        match self.references.get_mut(command) {
            Some(vec) => vec.push(self.offset),
            None => {
                let mut vec = Vec::new();
                vec.push(self.offset);
                self.references.insert(command.to_string(), vec);
            }
        };
        // placeholder --> invalid instruction
        self.add_instruction(0xFFFF)
    }

    fn add_instruction(&mut self, instruction: u16) -> Result<bool, String> {
        match &self.current_function{
            Some(name) => {
                self.definition_map.get_mut(name).unwrap().push(instruction);
            },
            None =>{
                self.ram.set_u16(self.offset, instruction);
                self.offset += 2;
            }
        }
        Ok(true)
    }

    fn add_ret_instruction(&mut self, instruction: u16) -> Result<bool, String> {
        // make sure to close the current definition
        self.ram.set_u16(self.offset, instruction);
        self.offset += 2;
        self.current_function = None;
        Ok(true)
    }

    fn add_x_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, end_instruction: u16) -> Result<bool, String> {
        let x = self.get_u16_value(values, 1)?;
        let instruction = start_instruction << 12 | x << 8 | end_instruction;
        self.add_instruction(instruction)
    }

    fn add_x_y_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, end_instruction: u16) -> Result<bool, String> {
        let x = self.get_u16_value(values, 1)?;
        let y = self.get_u16_value(values, 2)?;
        let instruction = start_instruction << 12 | x << 8 | y << 4 | end_instruction;
        self.add_instruction(instruction)
    }

    fn add_x_y_d_instruction(&mut self, start_instruction: u16, values: &Vec<&str>) -> Result<bool, String> {
        let x = self.get_u16_value(values, 1)?;
        let y = self.get_u16_value(values, 2)?;
        let d = self.get_u16_value(values, 3)?;
        let instruction = start_instruction << 12 | x << 8 | y << 4 | d;
        self.add_instruction(instruction)
    }

    fn add_x_kk_instruction(&mut self, start_instruction: u16, values: &Vec<&str>) -> Result<bool, String> {
        let x = self.get_u16_value(values, 1)?;
        let kk = self.get_u16_value(values, 2)?;
        let instruction = start_instruction << 12 | x << 8 | kk;
        self.add_instruction(instruction)
    }

    fn add_nnn_instruction(&mut self, start_instruction: u16, values: &Vec<&str>) -> Result<bool, String> {
        let nnn = self.get_u16_value(values, 1)?;
        let instruction = start_instruction << 12 | nnn;
        self.add_instruction(instruction)
    }

    fn get_u16_value(&self, values: &Vec<&str>, index: usize) -> Result<u16, String> {
        let value = match values.get(index) {
            Some(c) => *c,
            // line empty return --> probably does not work as expected
            None => return Err(format!("Invalid index '{}'", index))
        };
        match u16::from_str_radix(value, 16) {
            Ok(v) => Ok(v),
            Err(_) => Err(format!("Invalid u16 '{}'", value))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::ram::{RAM, RAM_OFFSET};

    #[test]
    fn test_ignore_comment_line() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        assert_eq!(interpreter.interpret_line("// some data"), Ok(true));
        assert_eq!(interpreter.offset, RAM_OFFSET);
        assert_eq!(ram.get(RAM_OFFSET), 0);
    }

    #[test]
    fn test_add_x_instruction() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        values.push("A");
        assert_eq!(interpreter.add_x_instruction(0xF, &values, 0x29), Ok(true));
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
        let result = interpreter.get_u16_value(&values, 1);
        assert_eq!(result, Ok(0xA));
    }

    #[test]
    fn get_u16_value_fail1() {
        let mut ram = RAM::new();
        let interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        assert_eq!(interpreter.get_u16_value(&values, 1).expect_err(""), "Invalid index '1'");
    }

    #[test]
    fn get_u16_value_fail2() {
        let mut ram = RAM::new();
        let interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        values.push("G");
        assert_eq!(interpreter.get_u16_value(&values, 1).expect_err(""), "Invalid u16 'G'");
    }

    #[test]
    fn test_interpret_ext() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        assert_eq!(interpreter.interpret_line("EXT"), Ok(true));
        assert_eq!(interpreter.offset, 2);
        assert_eq!(ram.get(RAM_OFFSET), 0x00);
        assert_eq!(ram.get(RAM_OFFSET + 1), 0x00);
    }

    #[test]
    fn test_interpret_stis() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        assert_eq!(interpreter.interpret_line("STIS 1"), Ok(true));
        assert_eq!(interpreter.offset, 2);
        assert_eq!(ram.get(RAM_OFFSET), 0xF1);
        assert_eq!(ram.get(RAM_OFFSET + 1), 0x29);
    }
}