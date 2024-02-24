const RAM_OFFSET: usize = 0x200;
// 4096
const RAM_SIZE: usize = 0x1000;


pub struct RAM {
    // 4096 bytes
    memory: [u8; RAM_SIZE],
    // 512 bytes reserved --> 0x200
}

pub fn create_ram() -> RAM{
    RAM {
        memory: [0; RAM_SIZE]
    }
}

impl RAM {
    pub fn set(&mut self, offset: usize, value: u8) {
        self.memory[RAM_OFFSET + offset] = value;
    }

    pub fn set_u16(&mut self, offset: usize, value: u16){
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xff) as u8;
        self.set(offset, high_byte);
        self.set(offset + 1, low_byte);
    }

    pub fn sets(&mut self, offset: usize, values: &[u8]) {
        let mut count: usize = 0;
        for val in values {
            self.memory[RAM_OFFSET + offset + count] = *val;
            count += 1;
        }
    }

    pub fn get(&self, offset: usize) -> u8 {
        self.memory[RAM_OFFSET + offset]
    }

    pub fn show(&self, from: usize, to: usize) {
        if from % 2 != 0 {
            panic!("From argument needs to be even");
        }
        if to % 2 != 0 {
            panic!("To argument needs to be even");
        }
        let mut current = RAM_OFFSET + from;
        while current < to {
            let number = ((self.memory[current] as u16) << 8) | self.memory[current + 1] as u16;
            println!("{}: {:04X}", current, number);
            current += 2;
        }
    }
}
