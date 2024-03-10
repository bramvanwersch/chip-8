use crate::drivers::Line;
use crate::ram::{RAM, RAM_OFFSET};

pub struct Interpreter<'a> {
    // TODO make reference with lifetime --> figure it out
    ram: &'a mut RAM,
    offset: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(ram: &'a mut RAM) -> Self {
        Interpreter {
            ram,
            offset: 0
        }
    }

    pub fn interpret_line(&mut self, line: &mut Line) {
        // ignore comment lines
        if line.value.starts_with("#") {
            return;
        }
        let values: Vec<&str> = line.value.split(" ").collect();
        // ignore empty lines
        let command = match values.get(0) {
            Some(c) => *c,
            // line empty return --> probably does not work as expected
            None => return
        };
        match command {
            "EXT" => self.add_instruction(0x0000),
            "STIS" => self.add_x_instruction(0xF, &values, 0x29, line),
            // (0, 0, 0, 0xE) => self.clear_display(display.unwrap()),
            // (0, 0, 0xE, 0xE) => self.ret(),
            // (0x1, _, _, _) => self.jump(nnn),
            // (0x2, _, _, _) => self.call(nnn),
            // (0x3, _, _, _) => self.skip_if_equal_x_to_kk(x, kk),
            // (0x4, _, _, _) => self.skip_if_not_equal_x_to_kk(x, kk),
            // (0x5, _, _, 0x0) => self.skip_if_equal_registers(x, y),
            // (0x6, _, _, _) => self.put_value_in_register(x, kk),
            // (0x7, _, _, _) => self.add_kk_to_x(x, kk),
            // (0x8, _, _, 0x0) => self.put_y_in_x(x, y),
            // (0x8, _, _, 0x1) => self.or_y_in_x(x, y),
            // (0x8, _, _, 0x2) => self.and_y_in_x(x, y),
            // (0x8, _, _, 0x3) => self.xor_y_in_x(x, y),
            // (0x8, _, _, 0x4) => self.add_y_to_x(x, y),
            // (0x8, _, _, 0x5) => self.sub_y_from_x(x, y),
            // (0x8, _, 0x0, 0x6) => self.rshift_x(x),
            // (0x8, _, _, 0x7) => self.sub_x_from_y(x, y),
            // (0x8, _, 0x0, 0xE) => self.lshift_x(x),
            // (0x9, _, _, 0x0) => self.skip_if_not_equal_registers(x, y),
            // (0xA, _, _, _) => self.set_i(nnn),
            // (0xB, _, _, _) => self.jump_plus_v0(nnn),
            // (0xC, _, _, _) => self.random_and_value(x, kk),
            // (0xD, _, _, _) => self.draw(ram, x, y, d, display.unwrap()),
            // (0xE, _, 0x9, 0xE) => self.skip_if_key_pressed(x, keypad.unwrap()),
            // (0xE, _, 0xA, 0xE) => self.skip_if_key_not_pressed(x, keypad.unwrap()),
            // (0xF, _, 0x0, 0x7) => self.set_register_to_delay(x),
            // (0xF, _, 0x0, 0xA) => self.await_key_press(x, keypad.unwrap()),
            // (0xF, _, 0x1, 0x5) => self.set_delay_to_register(x),
            // (0xF, _, 0x1, 0x8) => self.set_sound_to_register(x),
            // (0xF, _, 0x1, 0xE) => self.add_register_to_i(x),
            // (0xF, _, 0x2, 0x9) => self.set_i_to_char_loc(x),
            // (0xF, _, 0x3, 0x3) => self.register_to_bcd(x, ram),
            // (0xF, _, 0x5, 0x5) => self.copy_x_to_ram(x, ram),
            // (0xF, _, 0x6, 0x5) => self.copy_ram_to_x(x, ram),
            _ => { panic!("Invalid instruction {}", command); }
        }
    }

    fn add_instruction(&mut self, instruction: u16) {
        self.ram.set_u16(RAM_OFFSET + self.offset, instruction);
        self.offset += 2;
    }

    fn add_x_instruction(&mut self, start_instruction: u16, values: &Vec<&str>, end_instruction: u16, current_line: &Line) {
        let x = match values.get(1) {
            Some(c) => *c,
            // line empty return --> probably does not work as expected
            None => panic!("Incomplete instruction {}", current_line.panic_message())
        };
        let nx = x.parse::<u16>().expect(format!("Invalid value for x {}", current_line.panic_message()).as_str());
        let instruction = start_instruction << 12 | nx << 8 | end_instruction;
        self.add_instruction(instruction);
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
        let mut line = Line::new("# some data", 1);
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
        assert_eq!(ram.get(RAM_OFFSET), 0xFF);
        assert_eq!(ram.get(RAM_OFFSET + 2), 0x29);
    }

    #[test]
    #[should_panic(expected = "Invalid value for x at line 1 for 'test'")]
    fn test_add_x_instruction_fail() {
        let mut ram = RAM::new();
        let mut interpreter = Interpreter::new(&mut ram);
        let mut values: Vec<&str> = Vec::new();
        values.push("0");
        values.push("H");
        let line = Line::new("test", 1);
        interpreter.add_x_instruction(0xF, &values, 0x29, &line);
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