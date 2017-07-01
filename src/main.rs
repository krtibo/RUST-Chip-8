extern crate minifb;
pub mod cpu;
use cpu::Cpu;
use minifb::{Key, WindowOptions, Window, Scale};

#[allow(unused)]
fn main() {
    let mut chip8 : Cpu = Cpu::new();
    chip8.load_rom(String::from("./rom/INVADERS"));


    let mut window = Window::new("RUST Chip-8",
                                 64,
                                 32,
                                 WindowOptions {
                                     resize: true,
                                     scale: Scale::X4,
                                     ..WindowOptions::default()})
                                 .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape){

        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::Key1 => chip8.keypad[0] = 1,
                    Key::Key2 => chip8.keypad[1] = 1,
                    Key::Key3 => chip8.keypad[2] = 1,
                    Key::Key4 => chip8.keypad[3] = 1,
                    Key::Q => chip8.keypad[4] = 1,
                    Key::W => chip8.keypad[5] = 1,
                    Key::E => chip8.keypad[6] = 1,
                    Key::R => chip8.keypad[7] = 1,
                    Key::A => chip8.keypad[8] = 1,
                    Key::S => chip8.keypad[9] = 1,
                    Key::D => chip8.keypad[10] = 1,
                    Key::F => chip8.keypad[11] = 1,
                    Key::Y => chip8.keypad[12] = 1,
                    Key::X => chip8.keypad[13] = 1,
                    Key::C => chip8.keypad[14] = 1,
                    Key::V => chip8.keypad[15] = 1,
                    _ => (),
                }
            }
        });

        chip8.cycle();
        window.update_with_buffer(&chip8.v_memory);
        chip8.keypad = [0; 16];
    }
}
