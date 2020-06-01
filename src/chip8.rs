use super::games;
use rand::prelude::random;
use std::fs;

const OFFSET: usize = 512;

pub struct CPU {
    //The Chip 8 has 35 opcodes which are all two bytes long.
    //To store the current opcode, we need a data type that allows us to store two bytes.
    opcode: u16,
    //The Chip 8 has 4K memory in total, which we can emulated
    memory: [u8; 4096],
    //CPU registers: The Chip 8 has 15 8-bit general purpose registers named V0,V1 up to VE. The 16th register is used  for the ‘carry flag’.
    v: [u8; 16],

    //There is an Index register I and a program counter (pc) which can have a value from 0x000 to 0xFFF
    pub i: u16,
    pc: u16,

    stack: Vec<u16>,
    sp: u8,
    key: [u8; 16],
    //
    pub draw_flag: bool,
    timer: u8,
    pub gfx: [u8; 2048],
    pub log: Vec<String>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: OFFSET as u16,
            stack: Vec::new(),
            sp: 0,
            key: [0; 16],
            draw_flag: false,
            gfx: [0; 2048],
            timer: 0,
            log: Vec::new(),
        }
    }

    pub fn load_game(&mut self, name: &str) {
        self.put_fonst_to_memory();

        for (i, b) in games::get_data(name).iter().enumerate() {
            self.memory[i + OFFSET] = *b;
        }
    }

    fn put_fonst_to_memory(&mut self) {
        let chip8_fontset = [
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
        let fonst: Vec<&u8> = chip8_fontset.iter().flat_map(|f| f.iter()).collect();
        // fonst area from 0x000 to 0x1FF
        for (i, font) in fonst.iter().enumerate() {
            self.memory[i] = **font;
        }
    }

    fn fetch_opcode(&mut self) {
        let first = (self.memory[self.pc as usize] as u16) << 8;
        let second = self.memory[(self.pc + 1) as usize] as u16;
        self.opcode = first | second;
    }

    fn clear_display(&mut self) {
        self.gfx = [0; 2048];
        self.pc += 2;
    }

    //The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn return_from_subroutine(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }

    //The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn call_subroutine(&mut self) {
        self.stack.push(self.pc + 2);
        self.pc = self.opcode & 0x0FFF;
    }

    //The interpreter sets the program counter to nnn.
    fn jump_to(&mut self) {
        self.pc = self.opcode & 0x0FFF;
    }

    //Skip next instruction if Vx = kk.
    //The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn skip_next_instruction_if_eq(&mut self) {
        let reg_nbr = (self.opcode & 0x0F00) >> 8;
        let reg_val = (self.opcode & 0x00FF) as u8;

        if self.v[reg_nbr as usize] == reg_val {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    //Skip next instruction if Vx != kk.
    //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn skip_next_instruction_if_not_eq(&mut self) {
        let reg_nbr = (self.opcode & 0x0F00) >> 8;
        let reg_val = (self.opcode & 0x00FF) as u8;

        if self.v[reg_nbr as usize] != reg_val {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    //Skip next instruction if Vx = Vy.
    //The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn skip_next_instruction_if_registers_eq(&mut self) {
        let reg_x = (self.opcode & 0x0F00) >> 8;
        let reg_y = (self.opcode & 0x00F0) >> 4;
        if self.v[reg_x as usize] == self.v[reg_y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    //7xkk
    //Set Vx = Vx + kk.
    //Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn add_val_to_register(&mut self) {
        let reg_x = (self.opcode & 0x0F00) >> 8;
        let val = (self.opcode & 0x00FF) as u8;
        self.v[reg_x as usize] = self.v[reg_x as usize].wrapping_add(val);
        self.pc += 2;
    }

    //8xy_ - register operations
    fn reg_op(&mut self) {
        let reg_x = (self.opcode & 0x0F00) >> 8;
        let reg_y = (self.opcode & 0x00F0) >> 4;
        let param = self.opcode & 0x000F;
        match param {
            0x0 => {
                //Set Vx = Vy.
                self.v[reg_x as usize] = self.v[reg_y as usize]
            }
            0x1 => {
                //Set Vx = Vx OR Vy.
                self.v[reg_x as usize] |= self.v[reg_y as usize]
            }
            0x2 => {
                //Set Vx = Vx AND Vy.
                self.v[reg_x as usize] &= self.v[reg_y as usize]
            }
            0x3 => {
                //Set Vx = Vx XOR Vy..
                self.v[reg_x as usize] ^= self.v[reg_y as usize]
            }
            0x4 => {
                //Set Vx = Vx + Vy, set VF = carry.
                //The values of Vx and Vy are added together.
                //If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
                //Only the lowest 8 bits of the result are kept, and stored in Vx.
                let (result, overflow) =
                    self.v[reg_x as usize].overflowing_add(self.v[reg_y as usize]);
                if overflow {
                    self.v[15] = 1;
                } else {
                    self.v[15] = 0;
                }

                self.v[reg_x as usize] = result;
            }
            0x5 => {
                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                let vx = self.v[reg_x as usize];
                let vy = self.v[reg_y as usize];

                if vx > vy {
                    self.v[reg_x as usize] = vx - vy;
                    self.v[15] = 1;
                } else {
                    self.v[reg_x as usize] = vy - vx;
                    self.v[15] = 0;
                }
            }
            0x6 => {
                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                self.v[reg_x as usize] >>= 1;
                self.v[15] = self.v[reg_x as usize] & 0x1;
            }
            0x7 => {
                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                let vx = self.v[reg_x as usize];
                let vy = self.v[reg_y as usize];

                if vy > vx {
                    self.v[reg_x as usize] = vy - vx;
                    self.v[15] = 1;
                } else {
                    self.v[reg_x as usize] = vx - vy;
                    self.v[15] = 0;
                }
            }
            0xE => {
                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                self.v[reg_x as usize] <<= 1;
                self.v[15] = self.v[reg_x as usize] & 0x80;
            }
            _ => panic!("wrong 8XXX opcode"),
        }
        self.pc += 2;
    }

    fn set_val_to_register(&mut self) {
        let reg_nbr = (self.opcode & 0x0F00) >> 8;
        let reg_val = (self.opcode & 0x00FF) as u8;
        self.v[reg_nbr as usize] = reg_val;
        self.pc += 2;
    }

    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn skip_next_instr(&mut self) {
        let reg_x = (self.opcode & 0x0F00) >> 8;
        let reg_y = (self.opcode & 0x00F0) >> 4;
        if self.v[reg_x as usize] != self.v[reg_y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /*
    Annn - LD I, addr
    Set I = nnn.
    The value of register I is set to nnn.
    */
    fn set_i(&mut self) {
        self.i = self.opcode & 0x0FFF;
        self.pc += 2;
    }

    /*
    Bnnn - JP V0, addr
    Jump to location nnn + V0.
    The program counter is set to nnn plus the value of V0.
    */
    fn jump(&mut self) {
        let addr = self.opcode & 0x0FFF;
        self.pc = addr + self.v[0] as u16;
    }

    /*
    Cxkk - RND Vx, byte
    Set Vx = random byte AND kk.
    The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
    */
    fn set_random_to_v(&mut self) {
        let reg_x = (self.opcode & 0x0F00) >> 8;
        let kk = (self.opcode & 0x00FF) as u8;
        let rand = random::<u8>();
        let val = rand as u8 & kk;
        self.v[reg_x as usize] = val;
        self.pc += 2;
    }

    /*
    Dxyn - DRW Vx, Vy, nibble
    Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    The interpreter reads n bytes from memory, starting at the address stored in I.
    These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    Sprites are XORed onto the existing screen.
    If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
    */
    fn display(&mut self) {
        let reg_x = (self.opcode & 0x0F00) >> 8;
        let reg_y = (self.opcode & 0x00F0) >> 4;
        let height = (self.opcode & 0x000F) as u8;

        let (x, y) = (self.v[reg_x as usize], self.v[reg_y as usize]);
        self.v[15] = 0;
        self.draw_flag = false;

        for y_line in 0..height {
            let i = self.i + y_line as u16;
            let pixels_row = &self.memory[i as usize];

            for x_pos in 0..8 {
                let new_x = x.wrapping_add(x_pos) as u16;
                let new_y = y.wrapping_add(y_line) as u16 * 64;
                let pos = new_x + new_y;
                if pixels_row & (0x80 >> x_pos) != 0x0 {
                    if self.gfx[pos as usize] == 1 {
                        self.v[15] = 1;
                    }
                    self.gfx[pos as usize] ^= 1;
                }
            }
        }
        self.draw_flag = true;
        self.pc += 2;
    }

    fn handle_keys(&mut self) {
        /*
        Ex9E - SKP Vx
        Skip next instruction if key with the value of Vx is pressed.
        Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.

        ExA1 - SKNP Vx
        Skip next instruction if key with the value of Vx is not pressed.
        Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.


        */
        let x = (self.opcode & 0x0F00) >> 8;
        let param = self.opcode & 0x0FFF;
        match param & 0x0FF {
            0x9E => {
                if self.key[x as usize] == 1 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA1 => {
                self.log.clear();
                self.log.push(format!("Key {} set 0", x));
                if self.key[x as usize] == 0 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            _ => panic!("HANDLE KEY: {}", param),
        }

        // self.key = [0; 16]; // clear keys
    }

    //Fxxx
    fn f_codes(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let param = self.opcode & 0x0FFF;

        match param & 0x0FF {
            0x07 => {
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value.
                // The value of DT is placed into Vx.

                self.v[x as usize] = self.timer;
            }
            0x0A => {
                // Fx0A - LD Vx, K
                // Wait for a key press, store the value of the key in Vx.
                // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                // self.v[x as usize] = 0xA;
                panic!("Wait for key");
            }
            0x15 => {
                // Fx15 - LD DT, Vx
                // Set delay timer = Vx.
                // DT is set equal to the value of Vx.
                self.timer = self.v[x as usize];
            }
            0x18 => {
                // Fx18 - LD ST, Vx
                // Set sound timer = Vx.
                // ST is set equal to the value of Vx.
            }
            0x1E => {
                // Fx1E - ADD I, Vx
                // Set I = I + Vx.
                // The values of I and Vx are added, and the results are stored in I.
                self.i += self.v[x as usize] as u16;
            }
            0x29 => {
                // Set I = location of sprite for digit Vx.
                // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.

                self.i = (self.v[x as usize] * 5) as u16;
            }
            0x33 => {
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                // The interpreter takes the decimal value of Vx, and places
                // the hundreds digit in memory at location in I,
                // the tens digit at location I+1,
                // and the ones digit at location I+2.
                let val = self.v[x as usize];

                self.memory[self.i as usize] = (val / 100) as u8;
                self.memory[(self.i + 1) as usize] = ((val / 10) % 10) as u8;
                self.memory[(self.i + 2) as usize] = ((val % 10) % 10) as u8;
            }
            0x55 => {
                // Fx55 - LD [I], Vx
                // Store registers V0 through Vx in memory starting at location I.
                // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                for i in 0..x {
                    let index = (self.i + i) as usize;
                    self.memory[index] = self.v[i as usize];
                }
                self.i += x + 1;
            }
            0x65 => {
                // Read registers V0 through Vx from memory starting at location I.
                // The interpreter reads values from memory starting at location I into registers V0 through Vx.

                for i in 0..x {
                    let index = (self.i + i) as usize;
                    self.v[i as usize] = self.memory[index] as u8;
                }
                self.i += x + 1;
            }
            _ => {
                panic!("don know {:#04X}", self.opcode);
            }
        }

        self.pc += 2;
    }

    fn get_key_index(&self, key: &str) -> usize {
        match key {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "q" => 4,
            "w" => 5,
            "e" => 6,
            "r" => 7,
            "a" => 8,
            "s" => 9,
            "d" => 10,
            "f" => 11,
            "z" => 12,
            "x" => 13,
            "c" => 14,
            "v" => 15,
            _ => 0,
        }
    }

    pub fn key_down(&mut self, key: &str) {
        let index = self.get_key_index(key);
        self.key[index] = 1;
    }

    pub fn key_up(&mut self, key: &str) {
        let index = self.get_key_index(key);
        self.key[index] = 0;
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        // Decode Opcode & Execute Opcode
        match self.opcode {
            0x00E0 => self.clear_display(),
            0x00EE => self.return_from_subroutine(),
            opcode => match opcode & 0xF000 {
                0x1000 => self.jump_to(),
                0x2000 => self.call_subroutine(),
                0x3000 => self.skip_next_instruction_if_eq(),
                0x4000 => self.skip_next_instruction_if_not_eq(),
                0x5000 => self.skip_next_instruction_if_registers_eq(),
                0x6000 => self.set_val_to_register(),
                0x7000 => self.add_val_to_register(),
                0x8000 => self.reg_op(),
                0x9000 => self.skip_next_instr(),
                0xA000 => self.set_i(),
                0xB000 => self.jump(),
                0xC000 => self.set_random_to_v(),
                0xD000 => self.display(),
                0xE000 => self.handle_keys(),
                0xF000 => self.f_codes(),
                _ => {
                    panic!("MAIN LOOP DONT KNOW: {:#04X}", self.opcode);
                }
            },
        }

        // Update timers
        if self.timer > 0 {
            self.timer -= 1;
        }
    }

    pub fn v_display(&self) -> String {
        format!("V: {}\n", arr_to_str(&self.v))
    }

    pub fn keys_display(&self) -> String {
        format!("KEY: {}\n", arr_to_str(&self.key))
    }
}

fn arr_to_str(data: &[u8]) -> String {
    data.iter()
        .map(|r| format!("{:#04X}", r))
        .collect::<Vec<String>>()
        .join(" ")
}
