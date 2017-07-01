pub use cpu::Cpu;
use super::byteorder::{ByteOrder, BigEndian};
use super::colored::*;
use super::rand::Rng;

#[allow(unused)]
pub fn fetch(cpu : &mut Cpu) {

/*
 * During this step, the system will fetch one opcode from the
 * memory at the location specified by the program counter (pc).
 * In our Chip 8 emulator, data is stored in an array in which
 * each address contains one byte. As one opcode is 2 bytes long,
 * we will need to fetch two successive bytes and merge them to
 * get the actual opcode.
 */
    let buffer = [cpu.memory[cpu.pc as usize],
                  cpu.memory[(cpu.pc+1) as usize]];

    cpu.opcode = BigEndian::read_u16(&buffer);

}

pub fn execute(cpu : &mut Cpu) {
// TODO: implement

/*
 * Now that we know what to do with the opcode, we can execute
 * the opcode in our emulator. Because every instruction is 2 bytes
 * long, we need to increment the program counter by two after every
 * executed opcode. This is true unless you jump to a certain address
 * in the memory or if you call a subroutine (in which case you need
 * to store the program counter in the stack). If the next opcode
 * should be skipped, increase the program counter by four.
 */

     cpu.pc += 2;
     if cpu.opcode & 0xFFFF == 0x00e0 { _00e0(cpu); return; }
     if cpu.opcode & 0xFFFF == 0x00ee { _00ee(cpu); return; }
     if cpu.opcode & 0xF000 == 0x1000 { _1nnn(cpu); return; }
     if cpu.opcode & 0xF000 == 0x2000 { _2nnn(cpu); return; }
     if cpu.opcode & 0xF000 == 0x3000 { _3xkk(cpu); return; }
     if cpu.opcode & 0xF000 == 0x4000 { _4xkk(cpu); return; }
     if cpu.opcode & 0xF000 == 0x5000 { _5xy0(cpu); return; }
     if cpu.opcode & 0xF000 == 0x6000 { _6xkk(cpu); return; }
     if cpu.opcode & 0xF000 == 0x7000 { _7xkk(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8000 { _8xy0(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8001 { _8xy1(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8002 { _8xy2(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8003 { _8xy3(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8004 { _8xy4(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8005 { _8xy5(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8006 { _8xy6(cpu); return; }
     if cpu.opcode & 0xF00F == 0x8007 { _8xy7(cpu); return; }
     if cpu.opcode & 0xF00F == 0x800e { _8xye(cpu); return; }
     if cpu.opcode & 0xF000 == 0x9000 { _9xy0(cpu); return; }
     if cpu.opcode & 0xF000 == 0xa000 { _annn(cpu); return; }
     if cpu.opcode & 0xF000 == 0xb000 { _bnnn(cpu); return; }
     if cpu.opcode & 0xF000 == 0xc000 { _cxkk(cpu); return; }
     if cpu.opcode & 0xF000 == 0xd000 { _dxyn(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xe09e { _ex9e(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xe0a1 { _exa1(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xf007 { _fx07(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xf00a { _fx0a(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xf015 { _fx15(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xf01e { _fx1e(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xf033 { _fx33(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xf055 { _fx55(cpu); return; }
     if cpu.opcode & 0xF0FF == 0xf065 { _fx65(cpu); return; }

     // TODO:
     // fx18, fx29
}

fn _00e0(cpu : &mut Cpu) {
    // CLS - Clear the display.

    println!("{}",
    "CLS -------------------------00e0--".black().on_white());
    for i in 0..cpu.v_memory.len() {
        cpu.v_memory[i] = 0;
    }
}

fn _00ee(cpu : &mut Cpu) {
    // Return from a subroutine.
    // The interpreter sets the program counter to the address at
    // the top of the stack, then subtracts 1 from the stack pointer.

    println!("{}",
    "RET -------------------------00ee--".black().on_white().bold());

    if cpu.stack.len() > 0 && cpu.sp > 0 {
        cpu.pc = cpu.stack.pop().unwrap();
        cpu.sp -= 1;
    }
}

fn _1nnn(cpu : &mut Cpu) {
    // Jump to location nnn.
    // The interpreter sets the program counter to nnn.

    println!("{}",
    "JP ADDR ---------------------1nnn--".black().on_red().bold());

    let nnn : u16 = cpu.opcode & 0x0FFF;
    //println!("Opcode: {:x}, NNN: {:x}, PC: {:x}", cpu.opcode, nnn, cpu.pc);
    cpu.pc = nnn;
}

fn _2nnn(cpu : &mut Cpu) {
    // Call subroutine at nnn.
    // The interpreter increments the stack pointer, then puts the
    // current PC on the top of the stack. The PC is then set to nnn.

    println!("{}",
    "CALL ADDR -------------------2nnn--".black().on_red().underline());

    let nnn : u16 = cpu.opcode & 0x0FFF;
    cpu.sp += 1;
    cpu.stack.push(cpu.pc);
    cpu.pc = nnn;

}

fn _3xkk(cpu : &mut Cpu) {
    // Skip next instruction if Vx = kk.
    // The interpreter compares register Vx to kk, and if they are
    // equal, increments the program counter by 2.

    println!("{}",
    "SE Vx, BYTE -----------------3xkk--".red().on_blue().bold());
    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let kk : u8 = (cpu.opcode & 0x00FF) as u8;

    //println!("Opcode: {:x} x: {:x} kk: {:x}", cpu.opcode, x, kk);

    if cpu.v_regs[x as usize] == kk {
        cpu.pc += 2;
    }
}

fn _4xkk(cpu : &mut Cpu) {
    // Skip next instruction if Vx != kk.
    // The interpreter compares register Vx to kk, and if they are
    // not equal, increments the program counter by 2.

    println!("{}",
    "SNE Vx, BYTE ----------------4xkk--".red().on_blue());
    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let kk : u8 = (cpu.opcode & 0x00FF) as u8;

    //println!("Opcode: {:x} x: {:x} kk: {:x}", cpu.opcode, x, kk);

    if cpu.v_regs[x as usize] != kk {
        cpu.pc += 2;
    }
}

fn _5xy0(cpu : &mut Cpu) {
    // Skip next instruction if Vx = Vy.
    // The interpreter compares register Vx to register Vy, and if
    // they are equal, increments the program counter by 2.

    println!("{}",
    "SE Vx, Vy -------------------5xy0--".red().on_blue().underline());
    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    //println!("Opcode: {:x} x: {:x} y: {:x}", cpu.opcode, x, y);

    if cpu.v_regs[x as usize] == cpu.v_regs[y as usize] {
        cpu.pc += 2;
    }
}

fn _6xkk(cpu : &mut Cpu) {
    // Set Vx = kk.
    // The interpreter puts the value kk into register Vx.

    println!("{}",
    "LD Vx, BYTE -----------------6xkk--".white().on_cyan());
    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let kk : u8 = (cpu.opcode & 0x00FF) as u8;

    //println!("Opcode: {:x} x: {:x} kk: {:x}", cpu.opcode, x, kk);

    cpu.v_regs[x as usize] = kk;
}

fn _7xkk(cpu : &mut Cpu) {
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then
    // stores the result in Vx.

    println!("{}",
    "ADD Vx, BYTE ----------------7xkk--".blue().on_green());
    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let kk : u8 = (cpu.opcode & 0x00FF) as u8;

    //println!("Opcode: {:x} Vx: {:x} x: {:x} kk: {:x}", cpu.opcode, cpu.v_regs[x as usize], x, kk);
    let mut value : u16 = cpu.v_regs[x as usize] as u16 + kk as u16;
    value = value % 256;

    cpu.v_regs[x as usize] = value as u8;
}

fn _8xy0(cpu : &mut Cpu) {
    // Set Vx = Vy.
    // Stores the value of register Vy in register Vx.

    println!("{}",
    "LD Vx, Vy -------------------8xy0--".white().on_cyan().underline());
    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    //println!("Opcode: {:x} x: {:x} y: {:x}", cpu.opcode, x, y);

    cpu.v_regs[x as usize] = cpu.v_regs[y as usize];
}

fn _8xy1(cpu : &mut Cpu) {
    // Set Vx = Vx OR Vy.
    // Performs a bitwise OR on the values of Vx and Vy, then stores
    // the result in Vx. A bitwise OR compares the corrseponding bits
    // from two values, and if either bit is 1, then the same bit in the
    // result is also 1. Otherwise, it is 0.

    println!("{}",
    "OR Vx, Vy -------------------8xy1--".white().on_green());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    //println!("Opcode: {:x} x: {:x} y: {:x}", cpu.opcode, x, y);

    cpu.v_regs[x as usize] |= cpu.v_regs[y as usize];
}

fn _8xy2(cpu : &mut Cpu) {
    // Set Vx = Vx AND Vy.
    // Performs a bitwise AND on the values of Vx and Vy, then stores
    // the result in Vx.

    println!("{}",
    "AND Vx, Vy ------------------8xy2--".white().on_green().underline());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    //println!("Opcode: {:x} x: {:x} y: {:x}", cpu.opcode, x, y);

    cpu.v_regs[x as usize] &= cpu.v_regs[y as usize];
}

fn _8xy3(cpu : &mut Cpu) {
    // Set Vx = Vx XOR Vy.
    // Performs a bitwise XOR on the values of Vx and Vy, then stores
    // the result in Vx.

    println!("{}",
    "XOR Vx, Vy ------------------8xy3--".white().on_green().bold());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    //println!("Opcode: {:x} x: {:x} y: {:x}", cpu.opcode, x, y);

    cpu.v_regs[x as usize] ^= cpu.v_regs[y as usize];
}

fn _8xy4(cpu : &mut Cpu) {
    // Set Vx = Vx + Vy, set VF = carry.
    // The values of Vx and Vy are added together. If the result is
    // greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept, and stored in Vx.

    println!("{}",
    "ADD Vx, Vy ------------------8xy4--".blue().on_green().bold());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    //println!("Opcode: {:x} x: {:x} y: {:x}", cpu.opcode, x, y);

    let xy : u16 = cpu.v_regs[x as usize] as u16 +
                    cpu.v_regs[y as usize] as u16 ;

    if xy > 255 {
        cpu.v_regs[15] = 1;
        cpu.v_regs[x as usize] = xy as u8;
    } else {
        cpu.v_regs[15] = 0;
        cpu.v_regs[x as usize] = xy as u8;
    }
}

fn _8xy5(cpu : &mut Cpu) {
    // Set Vx = Vx - Vy, set VF = NOT borrow.
    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted
    // from Vx, and the results stored in Vx.

    println!("{}",
    "SUB Vx, Vy ------------------8xy5--".white().on_green().bold());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    if cpu.v_regs[x as usize] > cpu.v_regs[y as usize] {
        cpu.v_regs[15] = 1;
        cpu.v_regs[x as usize] -= cpu.v_regs[y as usize];
    } else {
        cpu.v_regs[15] = 0;
    }
}

// TODO: ????
fn _8xy6(cpu : &mut Cpu) {
    // Set Vx = Vx SHR 1.
    // If the least-significant bit of Vx is 1, then VF is set to 1,
    // otherwise 0. Then Vx is divided by 2.

    println!("{}",
    "SHR Vx ----------------------8xy6--".blue().on_green());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;

    if cpu.v_regs[x as usize] & 0b0000_0001 == 1 {
        cpu.v_regs[15] = 1;
        cpu.v_regs[x as usize] = cpu.v_regs[x as usize] >> 1;
    } else {
        cpu.v_regs[15] = 0;
        cpu.v_regs[x as usize] = cpu.v_regs[x as usize] >> 1;
    }
}

fn _8xy7(cpu : &mut Cpu) {
    // Set Vx = Vy - Vx, set VF = NOT borrow.
    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted
    // from Vy, and the results stored in Vx.

    println!("{}",
    "SUBN Vx, Vy ------------------8xy7--".white().on_green().bold().underline());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    if cpu.v_regs[y as usize] > cpu.v_regs[x as usize] {
        cpu.v_regs[15] = 1;
        cpu.v_regs[x as usize] = cpu.v_regs[y as usize] - cpu.v_regs[x as usize];
    } else {
        cpu.v_regs[15] = 0;
    }
}

// TODO:????
fn _8xye(cpu : &mut Cpu) {
    // Set Vx = Vx SHL 1.
    // If the least-significant bit of Vx is 1, then VF is set to 1,
    // otherwise 0. Then Vx is divided by 2.

    println!("{}",
    "SHL Vx ----------------------8xye--".blue().on_green().bold());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;

    if cpu.v_regs[x as usize] & 0b0000_0001 == 1 {
        cpu.v_regs[15] = 1;
        cpu.v_regs[x as usize] = cpu.v_regs[x as usize] << 1;
    } else {
        cpu.v_regs[15] = 0;
        cpu.v_regs[x as usize] = cpu.v_regs[x as usize] << 1;
    }
}

fn _9xy0(cpu : &mut Cpu) {
    // Skip next instruction if Vx != Vy.
    // The values of Vx and Vy are compared, and if they are not equal,
    // the program counter is increased by 2.

    println!("{}",
    "SNE Vx, Vy ------------------9xy0--".blue().on_cyan());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;

    if cpu.v_regs[x as usize] != cpu.v_regs[y as usize] {
        cpu.pc += 2;
    }
}

fn _annn(cpu : &mut Cpu) {
    // Set I = nnn.
    // The value of register I is set to nnn.

    println!("{}",
    "LD I, ADDR ------------------annn--".green().on_cyan());

    let nnn : u16 = cpu.opcode & 0x0FFF;

    cpu.i_reg = nnn;
}

fn _bnnn(cpu : &mut Cpu) {
    // Jump to location nnn + V0.
    // The program counter is set to nnn plus the value of V0.

    println!("{}",
    "JP V0, ADDR ------------------bnnn--".white().on_cyan());

    let nnn : u16 = cpu.opcode & 0x0FFF;

    cpu.pc = nnn + cpu.v_regs[0] as u16;
}

fn _cxkk(cpu : &mut Cpu) {
    // Set Vx = random byte AND kk.
    // The interpreter generates a random number from 0 to 255,
    // which is then ANDed with the value kk. The results are
    // stored in Vx.

    println!("{}",
    "RND Vx, BYTE ----------------cxkk--".yellow().on_cyan().underline());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let kk : u8 = (cpu.opcode & 0x00FF) as u8;
    let random : u8 = super::rand::thread_rng().gen_range(0,255);

    cpu.v_regs[x as usize] = random & kk;
}

fn _dxyn(cpu : &mut Cpu) {
    // Display n-byte sprite starting at memory location I at
    // (Vx, Vy), set VF = collision.
    // The interpreter reads n bytes from memory, starting at the
    // address stored in I. These bytes are then displayed as sprites
    // on screen at coordinates (Vx, Vy). Sprites are XORed onto the
    // existing screen. If this causes any pixels to be erased, VF is
    // set to 1, otherwise it is set to 0. If the sprite is positioned
    // so part of it is outside the coordinates of the display, it wraps
    // around to the opposite side of the screen.

    println!("{}",
    "DRW Vx, Vy ------------------dxyn--".yellow().on_cyan().bold());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let y : u8 = ((cpu.opcode & 0x00F0) >> 4) as u8;
    let n : u8 = (cpu.opcode & 0x000F) as u8;
    let vx : u8 = cpu.v_regs[x as usize];
    let vy : u8 = cpu.v_regs[y as usize];

    //println!("x: {:x} y: {:x} n: {:x}", vx, vy, n);

    cpu.v_regs[15] = 0;
    for i in 0..n {
        for j in 0..8 {
            if (cpu.memory[cpu.i_reg as usize + i as usize] & (0x80 >> j)) != 0 {
                /*
                println!("I: {} n: {} BYTE: {:b}",
                    cpu.i_reg + i as u16,
                    n,
                    cpu.memory[cpu.i_reg as usize + i as usize]);
                */

                if cpu.v_memory[coord((vx as usize + j as usize),
                (vy as usize + i as usize))] != 0 {
                    cpu.v_regs[15] = 1;
                }
                cpu.v_memory[coord((vx as usize + j as usize),
                (vy as usize + i as usize))] ^= 0xFFFFFFFF;
            }
        }
    }
}

fn _ex9e(cpu : &mut Cpu) {
    // Skip next instruction if key with the value of Vx is pressed.
    // Checks the keyboard, and if the key corresponding to the value
    // of Vx is currently in the down position, PC is increased by 2.

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;

    if cpu.keypad[cpu.v_regs[x as usize] as usize] == 1 {
        cpu.pc += 2;
    }
}

fn _exa1(cpu : &mut Cpu) {
    // Skip next instruction if key with the value of Vx is not pressed.
    // Checks the keyboard, and if the key corresponding to the value
    // of Vx is currently in the up position, PC is increased by 2.

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;

    if cpu.keypad[cpu.v_regs[x as usize] as usize] == 0 {
        cpu.pc += 2;
    }
}

fn _fx07(cpu : &mut Cpu) {
    // Set Vx = delay timer value.
    // The value of DT is placed into Vx.

    println!("{}",
    "LD Vx, DT -------------------fx07--".green().on_cyan());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    cpu.v_regs[x as usize] = cpu.dt;
}

fn _fx0a(cpu : &mut Cpu) {
    // Wait for a key press, store the value of the key in Vx.
    // All execution stops until a key is pressed, then the value of
    //  that key is stored in Vx.

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    let mut count = 0;

    for i in 0..16 {
        if cpu.keypad[i] == 0 {
            count += 1;
        } else {
            cpu.v_regs[x as usize] = i as u8;
            return;
        }
    }

    if count == 16 {
        cpu.pc -= 2;
    }
}

fn _fx15(cpu : &mut Cpu) {
    // Set delay timer = Vx.
    // DT is set equal to the value of Vx.

    println!("{}",
    "LD DT, Vx -------------------fx15--".green().on_cyan());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    //println!("Vx: {:x} DT: {:x}", cpu.v_regs[x as usize], cpu.dt);
    cpu.dt = cpu.v_regs[x as usize];
}

fn _fx1e(cpu : &mut Cpu) {
    // Set I = I + Vx.
    // The values of I and Vx are added, and the results are stored in I.

    println!("{}",
    "ADD I, Vx -------------------fx1e--".blue().on_green().bold());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    cpu.i_reg += cpu.v_regs[x as usize] as u16;
}

fn _fx33(cpu : &mut Cpu) {
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    // The interpreter takes the decimal value of Vx, and places the
    // hundreds digit in memory at location in I, the tens digit at
    // location I+1, and the ones digit at location I+2.

    println!("{}",
    "ADD B, Vx -------------------fx33--".blue().on_green().bold());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;
    cpu.memory[cpu.i_reg as usize] = cpu.v_regs[x as usize] / 100;
    cpu.memory[(cpu.i_reg + 1) as usize] = (cpu.v_regs[x as usize] / 10) % 10;
    cpu.memory[(cpu.i_reg + 2) as usize] = (cpu.v_regs[x as usize] % 100) % 10;
}

fn _fx55(cpu : &mut Cpu) {
    // Store registers V0 through Vx in memory starting at location I.
    // The interpreter copies the values of registers V0 through Vx into
    // memory, starting at the address in I.

    println!("{}",
    "LD [I], Vx ------------------fx55--".green().on_cyan());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;

    for i in 0..(x+1) {
        cpu.memory[cpu.i_reg as usize + i as usize] = cpu.v_regs[i as usize];
    }
}

fn _fx65(cpu : &mut Cpu) {
    // Read registers V0 through Vx from memory starting at location I.
    // The interpreter reads values from memory starting at location I
    // into registers V0 through Vx.

    println!("{}",
    "LD Vx, [I] ------------------fx65--".green().on_cyan());

    let x : u8 = ((cpu.opcode & 0x0F00) >> 8) as u8;

    for i in 0..(x+1) {
        cpu.v_regs[i as usize] = cpu.memory[cpu.i_reg as usize + i as usize];
    }
}

fn coord(x : usize, y : usize) -> usize {
    (((y % 32) * 64) + (x % 64))
}
