use crate::processor::Stack;
use crate::{CHIP8_RAM, CHIP8_WIDTH, CHIP8_HEIGHT};
use crate::font::FONT_SET;

const VARIABLE_REGISTER_SIZE: usize = 16;

pub struct CPU {

    pub ram: [u8; CHIP8_RAM],
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    program_counter: usize,
    index_register: u16,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; VARIABLE_REGISTER_SIZE]
}

impl CPU {

    pub fn new() -> Self {
        let mut ram = [0; CHIP8_RAM];

        for i in 0x50..0x50 + FONT_SET.len() {
            ram[i] = FONT_SET[i - 0x50];
        }

        CPU {
            ram,
            vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            program_counter: 0x200,
            index_register: 0,
            stack: Stack::new(),
            delay_timer: 255,
            sound_timer: 255,
            variable_registers: [0; VARIABLE_REGISTER_SIZE]
        }
    }
}