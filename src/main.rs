mod cpu;
mod ram;


fn main() {
    let mut cpu = cpu::create_cpu();
    let mut ram = ram::create_ram();

    cpu.set_register(0, 5);
    cpu.set_register(1, 10);

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

    ram.show(0, 2);

    cpu.run(&mut ram);
    ram.show(0, 2);
    println!("{}, {}", cpu.read_register(0), cpu.read_register(1));
}