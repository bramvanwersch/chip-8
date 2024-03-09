pub const RAM_OFFSET: usize = 0x200;
// 4096
pub const RAM_SIZE: usize = 0x1000;

pub struct RAM {
    // 4096 bytes
    memory: [u8; RAM_SIZE],
    // 512 bytes reserved --> 0x200
}

impl RAM {
    pub fn new() -> Self {
        let memory = [0; RAM_SIZE];
        let mut ram = RAM { memory };
        // set all the letters in memory
        ram.sets(0, &LETTERS);
        ram
    }

    pub fn set(&mut self, offset: usize, value: u8) {
        self.memory[offset] = value;
    }

    pub fn set_u16(&mut self, offset: usize, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xff) as u8;
        self.set(offset, high_byte);
        self.set(offset + 1, low_byte);
    }

    pub fn sets(&mut self, offset: usize, values: &[u8]) {
        let mut count: usize = 0;
        for val in values {
            self.memory[offset + count] = *val;
            count += 1;
        }
    }

    pub fn get(&self, offset: usize) -> u8 {
        self.memory[offset]
    }

    pub fn _show(&self, from: usize, mut to: usize) {
        if from % 2 != 0 {
            panic!("From argument needs to be even");
        }
        if to % 2 != 0 {
            panic!("To argument needs to be even");
        }
        let mut current = RAM_OFFSET + from;
        to += RAM_OFFSET;
        while current < to {
            let number = ((self.memory[current] as u16) << 8) | self.memory[current + 1] as u16;
            println!("{}: {:04X}", current, number);
            current += 2;
        }
    }
}

pub const LETTER_SIZE: usize = 5;
// bytes
const LETTERS: [u8; 80] = [
    // 0
    0xF0,
    0x90,
    0x90,
    0x90,
    0xF0,

    // 1
    0x20,
    0x60,
    0x20,
    0x20,
    0x70,

    // 2
    0xF0,
    0x10,
    0xF0,
    0x80,
    0xF0,

    // 3
    0xF0,
    0x10,
    0xF0,
    0x10,
    0xF0,

    // 4
    0x90,
    0x90,
    0xF0,
    0x10,
    0x10,

    // 5
    0xF0,
    0x80,
    0xF0,
    0x10,
    0xF0,

    // 6
    0xF0,
    0x80,
    0xF0,
    0x90,
    0xF0,

    // 7
    0xF0,
    0x10,
    0x20,
    0x40,
    0x40,

    // 8
    0xF0,
    0x90,
    0xF0,
    0x90,
    0xF0,

    // 9
    0xF0,
    0x90,
    0xF0,
    0x10,
    0xF0,

    // A
    0xF0,
    0x90,
    0xF0,
    0x90,
    0x90,

    // B
    0xE0,
    0x90,
    0xE0,
    0x90,
    0xE0,

    // C
    0xF0,
    0x80,
    0x80,
    0x80,
    0xF0,

    // D
    0xE0,
    0x90,
    0x90,
    0x90,
    0xE0,

    // E
    0xF0,
    0x80,
    0xF0,
    0x80,
    0xF0,

    // F
    0xF0,
    0x80,
    0xF0,
    0x80,
    0x80
];