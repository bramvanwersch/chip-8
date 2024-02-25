use crate::ram::RAM;

const STACK_SIZE: usize = 0x10;
// 16
const SPECIAL_REGISTER: usize = 0xF;

pub struct CPU {
    registers: [u8; 16],
    // position in memory
    program_counter: usize,
    stack: [u16; STACK_SIZE],
    stack_pointer: usize,
    // special register used for memory addresses mainly
    i: u16,

    //TODO: add the time and sound registers, decreasing at 60 Hz
}

pub fn create_cpu() -> CPU {
    CPU {
        program_counter: 0,
        registers: [0; 16],
        stack: [0; STACK_SIZE],
        stack_pointer: 0,
        i: 0x0,
    }
}

impl CPU {
    fn read_opcode(&self, ram: &RAM) -> u16 {
        let p = self.program_counter;
        let op_byte1 = ram.get(p) as u16;
        let op_byte2 = ram.get(p + 1) as u16;
        op_byte1 << 8 | op_byte2
    }
    // convenience functions
    pub fn set_register(&mut self, nr: usize, value: u8) {
        self.registers[nr] = value;
    }

    pub fn read_register(&self, nr: usize) -> u8 {
        self.registers[nr]
    }

    // run all RAM instructions
    pub fn run(&mut self, ram: &mut RAM) {
        loop {
            let opcode = self.read_opcode(ram);
            self.program_counter += 2;

            // opcode group
            let c = ((opcode & 0xF000) >> 12) as u8;
            // register 1
            let x = ((opcode & 0x0F00) >> 8) as u8;
            // register 2
            let y = ((opcode & 0x00F0) >> 4) as u8;

            let d = (opcode & 0x000F) as u8;

            let nnn = opcode & 0x0FFF;
            let kk = (opcode & 0x00FF) as u8;
            // println!("Instruction: {:04X}", opcode);

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; }
                (0, 0, 0, 0xE) => self.clear_display(),
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jump(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x3, _, _, _) => self.skip_if_equal_value(x, kk),
                (0x4, _, _, _) => self.skip_if_not_equal_value(x, kk),
                (0x5, _, _, 0) => self.skip_if_equal_registers(x, y),
                (0x6, _, _, _) => self.put_in_register(x, kk),
                (0x8, _, _, 0x4) => self.add(x, y),
                (0xA, _, _, _) => self.set_i(nnn),
                (0xF, _, 0x5, 0x5) => self.copy_to_memory(x, ram),
                (0xF, _, 0x6, 0x5) => self.copy_from_memory(x, ram),
                _ => { return; }
                // _ => todo!("opcode {:04x}", opcode)
            }
        }
    }

    fn clear_display(&mut self) {
        todo!("Needs implementation");
    }

    fn ret(&mut self) {
        // return from a system call
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    fn jump(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }

    fn call(&mut self, addr: u16) {
        // call a subroutine
        if self.stack_pointer >= self.stack.len() {
            panic!("Stack overflow, max call stack is {}!", STACK_SIZE);
        }
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn skip_if_equal_value(&mut self, register: u8, value: u8) {
        if self.registers[register as usize] == value {
            self.program_counter += 2;
        }
    }

    fn skip_if_not_equal_value(&mut self, register: u8, value: u8){
        if self.registers[register as usize] != value {
            self.program_counter += 2;
        }
    }

    fn skip_if_equal_registers(&mut self, register1: u8, register2: u8){
        if self.registers[register1 as usize] == self.registers[register2 as usize]{
            self.program_counter += 2;
        }
    }

    fn put_in_register(&mut self, register: u8, value: u8){
        self.registers[register as usize] = value;
    }

    fn add(&mut self, x: u8, y: u8) {
        // add x and y together
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[SPECIAL_REGISTER] = 1;
        } else {
            self.registers[SPECIAL_REGISTER] = 0;
        }
    }

    fn set_i(&mut self, nnn: u16) {
        // set special register i to a given value often a memory adres
        self.i = nnn;
    }

    fn copy_to_memory(&mut self, x: u8, ram: &mut RAM) {
        // copies the values of registers V0 through Vx into memory, starting at the address in i
        for nr in 0..x as usize + 1 {
            ram.set(self.i as usize + nr, self.registers[nr]);
        }
    }

    fn copy_from_memory(&mut self, x: u8, ram: &mut RAM) {
        for nr in 0..x as usize + 1 {
            self.registers[nr] = ram.get((self.i as usize) + nr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{create_cpu, SPECIAL_REGISTER};
    use crate::ram::create_ram;

    #[test]
    fn test_set_i() {
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        ram.set_u16(0, 0xA001);
        cpu.run(&mut ram);
        assert_eq!(cpu.i, 0x1);
    }

    #[test]
    fn test_set_register() {
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        ram.set_u16(0, 0x6009);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 9);
    }

    #[test]
    fn test_jump() {
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 2);
        ram.set_u16(0, 0x1004);
        ram.set_u16(2, 0x8014);
        cpu.run(&mut ram);
        // make sure that add was not run
        assert_eq!(cpu.read_register(0), 1);
    }

    #[test]
    fn test_skip_if_equal_value1() {
        // check skip if equal
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 2);
        ram.set_u16(0, 0x3102);
        ram.set_u16(2, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 1);
    }

    #[test]
    fn test_skip_if_equal_value2() {
        // check not skip[ if not equal
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 2);
        ram.set_u16(0, 0x3103);
        ram.set_u16(2, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 3);
    }

    #[test]
    fn test_skip_if_not_equal_value1() {
        // check skip if equal
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 6);
        ram.set_u16(0, 0x4102);
        ram.set_u16(2, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 1);
    }

    #[test]
    fn test_skip_if_not_equal_value2() {
        // check not skip[ if not equal
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 2);
        cpu.set_register(1, 3);
        ram.set_u16(0, 0x4002);
        ram.set_u16(2, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 5);
    }

    #[test]
    fn test_skip_if_equal_registers1() {
        // check skip if equal
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 1);
        ram.set_u16(0, 0x5010);
        ram.set_u16(2, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 1);
    }

    #[test]
    fn test_skip_if_equal_registers2() {
        // check not skip[ if not equal
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 3);
        ram.set_u16(0, 0x5010);
        ram.set_u16(2, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 4);
    }

        #[test]
    fn test_call() {
        // test both call and ret
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 2);
        let add_call: [u8; 4] = [
            0x80, 0x14,
            0x00, 0xEE
        ];
        ram.set_u16(0, 0x2100);
        ram.sets(0x100, &add_call);

        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 3);
        assert_eq!(cpu.i, 0x0);
    }

    #[test]
    #[should_panic(expected = "Stack overflow, max call stack is 16!")]
    fn test_stack_overflow() {
        // test both call and ret
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 2);
        let add_call: [u8; 6] = [
            0x80, 0x14,
            0x21, 0x00,
            0x00, 0xEE
        ];
        ram.set_u16(0, 0x2100);
        ram.sets(0x100, &add_call);

        cpu.run(&mut ram);
    }

    #[test]
    fn test_add() {
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 2);
        ram.set_u16(0, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 3);
        assert_eq!(cpu.read_register(SPECIAL_REGISTER), 0);
    }

    #[test]
    fn test_add_overflow() {
        // integer simply overflows
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 255);
        ram.set_u16(0, 0x8014);
        cpu.run(&mut ram);
        assert_eq!(cpu.read_register(0), 0);
        assert_eq!(cpu.read_register(SPECIAL_REGISTER), 1);
    }

    #[test]
    fn test_copy_to_memory() {
        // integer simply overflows
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        cpu.set_register(0, 1);
        cpu.set_register(1, 255);
        cpu.set_register(2, 2);
        cpu.set_register(3, 100);

        ram.set_u16(0, 0xA010);
        ram.set_u16(2, 0xF455);
        cpu.run(&mut ram);
        assert_eq!(ram.get(0x10), 1);
        assert_eq!(ram.get(0x11), 255);
        assert_eq!(ram.get(0x12), 2);
        assert_eq!(ram.get(0x13), 100);
    }

    #[test]
    fn test_copy_from_memory() {
        // integer simply overflows
        let mut cpu = create_cpu();
        let mut ram = create_ram();

        ram.set_u16(0, 0xA010);
        ram.set_u16(2, 0xF265);
        ram.set(0x10, 1);
        ram.set(0x11, 52);
        cpu.run(&mut ram);
        ram.show(0, 20);
        assert_eq!(cpu.read_register(0), 1);
        assert_eq!(cpu.read_register(1), 52);
    }
}

