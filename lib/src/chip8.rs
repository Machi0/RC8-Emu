use std::io::prelude::*;
use std::fs::File;
use rand::Rng;

#[derive(Debug)]
pub struct Chip8 {
    pc: u16,
    memory: [u8; 4096],
    v_reg: [u8; 16],
    i_reg: u16,

    stack: [u16; 16],
    sp: u16,

    pub gfx: [u8; 64 * 32],
    pub keypad: [u8; 16],
    pub sound_timer: u8,
    delay_timer: u8,
}


impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            pc: 0x200,
            memory: [0; 4096],
            v_reg: [0; 16],
            i_reg: 0,
            stack: [0; 16],
            sp: 0,
            gfx: [0; 64 * 32],
            keypad: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_rom(&mut self, path: &str) -> () {
        let f = File::open(path).unwrap_or_else(|_| panic!("Error opening file {}", path));
        let mut i = 0x200;

        for byte in f.bytes() {
            self.memory[i] = byte.unwrap();
            i += 1;
        }
    }

    pub fn load_fontset(&mut self) -> () {
        let fontset: [u8; 80] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..80 {
            self.memory[i + 0x50] = fontset[i];
        }
    }

    pub fn interpret(&mut self) -> () {
        let opcode: u16 = 
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
        println!("{:#4x?}", opcode);

        match opcode & 0xF000 {
            0x0000 =>
                match opcode & 0x000F {
                    0x0000 => {
                        self.gfx.iter_mut().for_each(|n| *n = 0);
                        self.pc += 2;
                    },

                    0x000E => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    },

                    _ => throw_error(opcode),
                },
            
            0x1000 => self.pc = opcode & 0x0FFF,

            0x2000 => {
                self.stack[self.sp as usize] = self.pc + 2;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            },

            0x3000 => {
                self.pc +=
                    if self.v_reg[((opcode & 0x0F00) >> 8) as usize] == opcode as u8 { 4 } else { 2 };
            },

            0x4000 => {
                self.pc +=
                    if self.v_reg[((opcode & 0x0F00) >> 8) as usize] != opcode as u8 { 4 } else { 2 };
            },

            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v_reg[x as usize] == self.v_reg[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },

            0x6000 => {
                self.v_reg[((opcode & 0x0F00) >> 8) as usize] = opcode as u8;
                self.pc += 2;
            },

            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;

                self.v_reg[x] = self.v_reg[x].wrapping_add(opcode as u8);

                self.pc += 2;
            },

            0x8000 =>
                match opcode & 0x000F {
                    0x0000 => {
                        self.v_reg[((opcode & 0x0F00) >> 8) as usize] = 
                            self.v_reg[((opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0001 => {
                        self.v_reg[((opcode & 0x0F00) >> 8) as usize] |= 
                            self.v_reg[((opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0002 => {
                        self.v_reg[((opcode & 0x0F00) >> 8) as usize] &= 
                            self.v_reg[((opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0003 => {
                        self.v_reg[((opcode & 0x0F00) >> 8) as usize] ^= 
                            self.v_reg[((opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0004 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        let y = ((opcode & 0x00F0) >> 4) as usize;

                        let sum = self.v_reg[x] as u16 + self.v_reg[y] as u16;

                        self.v_reg[0xF] = if sum > 255 { 1 } else { 0 };
                        self.v_reg[x] = (sum & 0x00FF) as u8;
                        self.pc += 2;
                    },

                    0x0005 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        let y = ((opcode & 0x00F0) >> 4) as usize;
                        
                        if self.v_reg[x] > self.v_reg[y] {
                            self.v_reg[0xF] = 1;
                            self.v_reg[x] -= self.v_reg[y];
                        } else {
                            self.v_reg[0xF] = 0;
                            self.v_reg[x] = self.v_reg[x].wrapping_sub(self.v_reg[y]);
                        }

                        self.pc += 2;
                    },

                    0x0006 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        let y = ((opcode & 0x00F0) >> 4) as usize;

                        self.v_reg[0xF] = self.v_reg[y] & 0b00000001;

                        self.v_reg[x] = self.v_reg[y] >> 1;
                        self.pc += 2;
                    },

                    0x0007 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        let y = ((opcode & 0x00F0) >> 4) as usize;

                        if self.v_reg[y] > self.v_reg[x] {
                            self.v_reg[0xF] = 1;
                            self.v_reg[x] = self.v_reg[y] - self.v_reg[x];
                        } else {
                            self.v_reg[0xF] = 0;
                            self.v_reg[x] = self.v_reg[y].wrapping_sub(self.v_reg[x]);
                        }

                        self.pc += 2;
                    },

                    0x000E => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        let y = ((opcode & 0x00F0) >> 4) as usize;

                        self.v_reg[0xF] = self.v_reg[y] & 0b10000000;

                        self.v_reg[x] = self.v_reg[y] << 1;
                        self.pc += 2;
                    },

                    _ => throw_error(opcode),
                },

            0x9000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                self.pc += if self.v_reg[x] != self.v_reg[y] { 4 } else { 2 };
            },

            0xA000 => {
                self.i_reg = opcode & 0x0FFF;
                self.pc += 2;
            },

            0xB000 => self.pc = (opcode & 0x0FFF) + self.v_reg[0] as u16,

            0xC000 => {
                let mut rng = rand::thread_rng();
                self.v_reg[((opcode & 0x0F00) >> 8) as usize] =
                    (opcode & 0x00FF) as u8 & rng.gen_range(0..=255);

                self.pc += 2;
            },

            0xD000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let height = opcode & 0x000F;

                let x_pos = self.v_reg[x];
                let y_pos = self.v_reg[y];

                self.v_reg[0xF] = 0;

                for row in 0..height {
                    let pixel = self.memory[(self.i_reg + row) as usize];

                    for col in 0..8 {
                        if (pixel & (0x80 >> col)) != 0 {
                            let pos = ((x_pos as u16 + col as u16) + ((y_pos as u16 + row) * 64)) % 2048;

                            if self.gfx[pos as usize] == 1 {
                                self.v_reg[0xF] = 1;
                            }
                            self.gfx[pos as usize] ^= 1;
                        }
                    }
                }

                self.pc += 2;
            },

            0xE000 => 
                match opcode & 0x000F {
                    0x000E => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        self.pc += 
                            if self.keypad[self.v_reg[x] as usize] != 0 { 4 } else { 2 };
                    },

                    0x0001 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        self.pc += 
                            if self.keypad[self.v_reg[x] as usize] == 0 { 4 } else { 2 };
                    },

                    _ => throw_error(opcode),
            },

            0xF000 =>
                match opcode & 0x00FF {
                    0x0007 => {
                        self.v_reg[((opcode & 0x0F00) >> 8) as usize] = self.delay_timer;
                        self.pc += 2;
                    },

                    0x000A => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        if self.keypad[0] == 1 {
                            self.v_reg[x] = 0;
                            self.pc += 2;
                        } else if self.keypad[1] == 1 {
                            self.v_reg[x] = 1;
                            self.pc += 2;
                        } else if self.keypad[2] == 1 {
                            self.v_reg[x] = 2;
                            self.pc += 2;
                        } else if self.keypad[3] == 1 {
                            self.v_reg[x] = 3;
                            self.pc += 2;
                        } else if self.keypad[4] == 1 {
                            self.v_reg[x] = 4;
                            self.pc += 2;
                        } else if self.keypad[5] == 1 {
                            self.v_reg[x] = 5;
                            self.pc += 2;
                        } else if self.keypad[6] == 1 {
                            self.v_reg[x] = 6;
                            self.pc += 2;
                        } else if self.keypad[7] == 1 {
                            self.v_reg[x] = 7;
                            self.pc += 2;
                        } else if self.keypad[8] == 1 {
                            self.v_reg[x] = 8;
                            self.pc += 2;
                        } else if self.keypad[9] == 1 {
                            self.v_reg[x] = 9;
                            self.pc += 2;
                        } else if self.keypad[10] == 1 {
                            self.v_reg[x] = 10;
                            self.pc += 2;
                        } else if self.keypad[11] == 1 {
                            self.v_reg[x] = 11;
                            self.pc += 2;
                        } else if self.keypad[12] == 1 {
                            self.v_reg[x] = 12;
                            self.pc += 2;
                        } else if self.keypad[13] == 1 {
                            self.v_reg[x] = 13;
                            self.pc += 2;
                        } else if self.keypad[14] == 1 {
                            self.v_reg[x] = 14;
                            self.pc += 2;
                        } else if self.keypad[15] == 1 {
                            self.v_reg[x] = 15;
                            self.pc += 2;
                        }
                    },

                    0x0015 => {
                        self.delay_timer = self.v_reg[((opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    },

                    0x0018 => {
                        self.sound_timer = self.v_reg[((opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    },

                    0x001E => {
                        self.i_reg += self.v_reg[((opcode & 0x0F00) >> 8) as usize] as u16;
                        self.pc += 2;
                    },

                    0x0029 => {
                        self.i_reg = 0x50 + 5 * self.v_reg[((opcode & 0x0F00) >> 8) as usize] as u16;
                        self.pc += 2;
                    },

                    0x0033 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        let mut val = self.v_reg[x];

                        self.memory[(self.i_reg + 2) as usize] = val % 10;
                        val /= 10;
                        self.memory[(self.i_reg + 1) as usize] = val % 10;
                        val /= 10;
                        self.memory[self.i_reg as usize] = val % 10;
                        
                        self.pc += 2;
                    },

                    0x0055 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;

                        for i in 0..=x {
                            self.memory[self.i_reg as usize + i] = self.v_reg[i];
                        }

                        self.i_reg += (x + 1) as u16;
                        self.pc += 2;
                    },

                    0x0065 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;

                        for i in 0..=x {
                            self.v_reg[i] = self.memory[self.i_reg as usize + i];
                        }
                        self.i_reg += (x + 1) as u16;
                        self.pc += 2;
                    },

                    _ => throw_error(opcode),
                },

            _ => throw_error(opcode),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            println!("BEEP");
            self.sound_timer -= 1;
        }
    }

}

fn throw_error(opcode: u16) -> () {
    panic!("Invalid opcode: {:#4x?}", opcode);
}
