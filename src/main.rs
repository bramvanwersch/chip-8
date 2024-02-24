// 512
const RAM_OFFSET: usize = 0x200;
// 4096
const RAM_SIZE: usize = 0x1000;
const STACK_SIZE: usize = 0x10;
// 16
const SPECIAL_REGISTER: usize = 0xF;

struct RAM {
    // 4096 bytes
    memory: [u8; RAM_SIZE],
    // 512 bytes reserved --> 0x200
}

fn create_ram() -> RAM{
    RAM {
        memory: [0; RAM_SIZE]
    }
}

impl RAM {
    fn set(&mut self, offset: usize, value: u8) {
        self.memory[RAM_OFFSET + offset] = value;
    }

    fn set_u16(&mut self, offset: usize, value: u16){
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xff) as u8;
        self.memory[RAM_OFFSET + offset] = high_byte;
        self.memory[RAM_OFFSET + offset + 1] = low_byte;
    }

    fn sets(&mut self, offset: usize, values: &[u8]) {
        let mut count: usize = 0;
        for val in values {
            self.memory[RAM_OFFSET + offset + count] = *val;
            count += 1;
        }
    }

    fn get(&self, offset: usize) -> u8 {
        self.memory[RAM_OFFSET + offset]
    }

    fn show(&self, from: usize, to: usize) {
        if from % 2 != 0 {
            panic!("From argument needs to be even");
        }
        if to % 2 != 0 {
            panic!("To argument needs to be even");
        }
        let mut current = from;
        while current < to {
            let number = ((self.memory[current] as u16) << 8) | self.memory[current + 1] as u16;
            println!("{}: {:04X}", current, number);
            current += 2;
        }
    }
}


struct CPU {
    registers: [u8; 16],
    // position in memory
    program_counter: usize,
    stack: [u16; STACK_SIZE],
    stack_pointer: usize,
    // special register used for memory addresses mainly
    i: u16,

    //TODO: add the time and sound registers, decreasing at 60 Hz
}

fn create_cpu() -> CPU{
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

    fn run(&mut self, ram: &mut RAM) {
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
            let _kk = (opcode & 0x00FF) as u8;
            println!("Instruction: {:04X}", opcode);

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0xA, _, _, _) => self.set_i(nnn),
                (0xF, _, 0x5, 0x5) => self.copy_to_memory(x, ram),
                _ => todo!("opcode {:04x}", opcode)
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
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

    fn call(&mut self, addr: u16) {
        // call a subroutine
        if self.stack_pointer >= self.stack.len() {
            panic!("Stack overflow, max stack is {}!", STACK_SIZE);
        }
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        // return from a system call
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
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
}

#[cfg(test)]
mod tests {
    use crate::{create_cpu, create_ram};

    #[test]
    fn test_set_i() {
        let mut cpu = create_cpu();
        let mut ram = create_ram();
        ram.set_u16(0, 0xA001);
        cpu.run(&mut ram);
        assert_eq!(cpu.i, 0x1);

    }
}


fn main() {
    let mut cpu = create_cpu();
    let mut ram = create_ram();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // function that adds a number 3 times
    let add_thrice: [u8; 8] = [
        0x80, 0x14,
        0x80, 0x14,
        0x80, 0x14,
        0x00, 0xEE
    ];

    ram.set(0, 0x20);
    ram.set(1, 0x06);
    ram.set(2, 0xF1);
    ram.set(3, 0x55);
    ram.set(4, 0x00);
    ram.set(5, 0x00);
    ram.sets(6, &add_thrice);

    ram.show(RAM_OFFSET, RAM_OFFSET + 2);

    cpu.run(&mut ram);
    ram.show(RAM_OFFSET, RAM_OFFSET + 2);
    println!("{}, {}", cpu.registers[0], cpu.registers[1]);
}