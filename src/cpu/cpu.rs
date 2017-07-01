use std::io::prelude::*;
use std::fs::File;
use cpu::opcode::*;
use std::time;
use super::time as ttime;
use std::thread;
#[allow(unused)]

pub struct Cpu {
    rom_buffer : Vec<u8>,
    pub opcode : u16,
    pub memory : [u8; 4096],
    pub v_regs : [u8; 16],
    // V0 - VE: general purpose registers,
    // VF: carry flag register
    pub i_reg : u16,                // Index register
    pub pc : u16,               // Program Counter
    pub v_memory : [u32; 64 * 32],   // Video memory
    pub stack : Vec<u16>,          // Stack memory
    pub sp : u16,                   // Stack Pointer
    pub dt : u8,
    pub keypad : [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {

        Cpu {
            rom_buffer : Vec::new(),
            opcode : 0,
            memory : [0; 4096],
            v_regs : [0; 16],
            i_reg : 0,
            pc : 0x200,
            sp : 0,
            stack : Vec::new(),
            v_memory : [0; 64 * 32],
            dt : 0,
            keypad : [0; 16],
        }
    } // fn new

    pub fn load_rom(&mut self, path: String) {

        let mut f = File::open(path)
        .expect("Error with file loading!");

        f.read_to_end(&mut self.rom_buffer)
        .expect("Error with file reading!");

        for i in &self.rom_buffer {
            print!("{:x} ", i);
        }
        println!("\nRom length (in bytes): {}", self.rom_buffer.len());

        // load the rom to the memory
        // the starting address is 0x200 (512)
        for i in 0..self.rom_buffer.len() {
            self.memory[i + 512] = self.rom_buffer[i];
        }

        let font_set : [u8; 80] = [
              0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
              0x20, 0x60, 0x20, 0x20, 0x70, // 1
              0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
              0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
              0x90, 0x90, 0xF0, 0x10, 0x10, // 4
              0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
              0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
              0xF0, 0x10, 0x20, 0x40, 0x40, // 7
              0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
              0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
              0xF0, 0x90, 0xF0, 0x90, 0x90, // A
              0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
              0xF0, 0x80, 0x80, 0x80, 0xF0, // C
              0xE0, 0x90, 0x90, 0x90, 0xE0, // D
              0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
              0xF0, 0x80, 0xF0, 0x80, 0x80]; // F

        for i in 0..80 {
            self.memory[i] = font_set[i];
        }

    } // load_rom

    pub fn cycle(&mut self) {
        let start = ttime::get_time();

        fetch(self);        // fetch opcode from rom_buffer
        execute(self);      // execute opcode
        if self.dt > 0 {
            self.dt -= 1;
        }

        let stop = ttime::get_time();
        let diff = 16 - ((stop.nsec - start.nsec)/1000) as i64;
        if 0 < diff && diff < 20 {
            thread::sleep(time::Duration::from_millis(diff as u64));
        }

    } // fn cycle


}
