use core::ops::{Index, IndexMut};

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};


const SPRITE_CHARS: [[u8; 5]; 0x10] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];
const SPRITE_CHARS_ADDR: u16 = 0x0000;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGTH: usize = 32;
const MEM_SIZE: usize = 0x1000;
const ROM_ADDR: usize = 0x200;
const FRAME_TIME: isize = 16666; // In microseconds

#[derive(Clone, Copy)]
struct Reg(u8);

struct Regs([u8; 0x10]);

impl Index<Reg> for Regs {
    type Output = u8;

    fn index(&self, reg: Reg) -> &Self::Output {
        &self.0[reg.0 as usize]
    }
}

impl IndexMut<Reg> for Regs {
    fn index_mut(&mut self, reg: Reg) -> &mut Self::Output {
        &mut self.0[reg.0 as usize]
    }
}

impl Regs {
    fn new() -> Self {
        Self([0; 0x10])
    }
}

pub struct Chip8<R: RngCore> {
    mem: [u8; MEM_SIZE],
    v: Regs, // Register Set
    i: u16,  // Index Register
    pc: u16, // Program Counter
    stack: [u16; 0x10],
    sp: u8,                                     // Stack Pointer
    dt: u8,                                     // Delay Timer
    st: u8,                                     // Sound Timer
    keypad: u16,                                // Keypad
    fb: [u8; SCREEN_WIDTH * SCREEN_HEIGTH / 8], // Framebuffer
    tone: bool,                                 // Tone output enable
    time: isize,                                // Overtime in microseconds
    rng: R,                                     // Instance of a random number generator
}



impl<R: RngCore> Chip8<R> {
    // Standard Chip-8 Instructions - Using Cowgod's reference.

    /// 00E0 - Clear the display.
    fn op_cls(&mut self) -> usize {
        for b in self.fb.iter_mut() {
            *b = 0;
        }
        self.pc += 2;
        109
    }
}