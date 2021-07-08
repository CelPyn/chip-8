use std::{thread, time};
use processor::CPU;

mod processor;
mod driver;
mod font;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;

fn main() {
    let wait_time = time::Duration::from_millis(2);
    let start_time = time::Instant::now();

    let mut cpu = CPU::new();

    println!("ram: {:?}", cpu.ram);

    for i in 0..cpu.ram.len() {
        println!("ram: index {} has value {}", i, cpu.ram[i]);
    }

    loop {

        println!("{}ms", start_time.elapsed().as_millis());
        thread::sleep(wait_time);
    }
}
