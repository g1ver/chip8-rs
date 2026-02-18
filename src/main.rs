use std::fs;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const MEMORY_SIZE: usize = 4096;
const INSTRUCTIONS_PER_SEC: usize = 700;

const FONTS: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    frame_buffer: [u8; WIDTH * HEIGHT],
    program_counter: u16,
    index_register: u16,
    stack: [u16; 12],
    stack_pointer: u8, // points at next empty slot
    // V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF
    // VF is the flag register
    registers: [u8; 16],
    sound_timer: u8,
    delay_timer: u8,
}

impl std::fmt::Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Chip-8 State ===")?;
        writeln!(
            f,
            "PC: 0x{:04X}  I: 0x{:04X}  SP: 0x{:02X}",
            self.program_counter, self.index_register, self.stack_pointer
        )?;
        writeln!(
            f,
            "DT: 0x{:02X}  ST: 0x{:02X}",
            self.delay_timer, self.sound_timer
        )?;

        write!(f, "Registers: ")?;
        for (i, &reg) in self.registers.iter().enumerate() {
            write!(f, "V{:X}:0x{:02X} ", i, reg)?;
        }
        writeln!(f)?;

        writeln!(f, "\nMemory:")?;
        for (i, chunk) in self.memory.chunks(16).enumerate() {
            write!(f, "{:04X}: ", i * 16)?;
            for byte in chunk {
                write!(f, "{:02X} ", byte)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Chip8 {
    fn new() -> Self {
        let mut chip8 = Self {
            memory: [0u8; MEMORY_SIZE],
            frame_buffer: [0u8; WIDTH * HEIGHT],
            program_counter: 0x200,
            index_register: 0x0,
            stack: [0u16; 12],
            stack_pointer: 0,
            registers: [0u8; 16],
            sound_timer: 0,
            delay_timer: 0,
        };

        chip8.load_fonts();
        chip8
    }

    fn load_fonts(&mut self) {
        // Community convention is to load fonts into 0x050 to 0x09F
        self.memory[0x050..=0x09F].copy_from_slice(&FONTS[..]);
    }

    fn load_rom(&mut self, file_path: String) {
        let rom = fs::read(&file_path);

        let rom = match rom {
            Ok(rom) => {
                println!("Loaded ROM from {}", file_path);
                rom
            }
            Err(e) => panic!("Error: Could not load ROM.\n{}", e),
        };

        // 0x000 to 0x200 is reserved
        if rom.len() > (MEMORY_SIZE - 0x200) {
            panic!("Error: ROM is too large.")
        }

        self.memory[0x200..(0x200 + rom.len())].copy_from_slice(&rom[..]);
    }

    // get the current instruction at the program counter location
    fn fetch(&mut self) -> u16 {
        let pc = self.program_counter as usize;
        let hi = self.memory[pc];
        let lo = self.memory[pc + 1];

        let instruction = (hi as u16) << 8 | (lo as u16);
        self.program_counter += 2;
        instruction
    }

    fn decode_exec(&mut self, instruction: u16) {
        let opcode = ((instruction & 0xF000) >> 12) as u8;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        let n = (instruction & 0x000F) as u8;
        let kk = (instruction & 0x00FF) as u8;
        let nnn = (instruction & 0x0FFF) as u16;

        match opcode {
            0x00 => match kk {
                0xE0 => {
                    // 00E0 - CLS
                    self.frame_buffer.fill(0);
                }
                0xEE => {
                    todo!()
                }
                _ => panic!("Error: No instruction found."),
            },
            0x1 => {
                // 1nnn - JP addr
                self.program_counter = nnn;
            }
            0x2 => todo!(),
            0x3 => todo!(),
            0x4 => todo!(),
            0x5 => todo!(),
            0x6 => {
                // 6xkk - LD Vx, byte
                self.registers[x] = kk;
            }
            0x7 => todo!(),
            0x8 => todo!(),
            0x9 => todo!(),
            0xA => {
                // Annn - LD I, addr
                self.index_register = nnn;
            }
            0xB => todo!(),
            0xC => todo!(),
            0xD => {
                // Dxyn - DRW Vx, Vy, nibble
                let x_coordinate = self.registers[x] % (WIDTH as u8);
                let y_coordinate = self.registers[y] % (HEIGHT as u8);
                self.registers[0xF] = 0;

                for row_index in 0..n {
                    let sprite_byte =
                        self.memory[(self.index_register as usize) + (row_index as usize)];

                    for col_index in 0..8 {
                        let sprite_pixel = (sprite_byte >> (7 - col_index)) & 1;

                        let screen_x = x_coordinate as usize + col_index;
                        let screen_y = y_coordinate as usize + row_index as usize;

                        if screen_x < WIDTH && screen_y < HEIGHT {
                            if sprite_pixel == 1 {
                                let screen_idx = screen_y * WIDTH + screen_x;
                                if self.frame_buffer[screen_idx] == 1 {
                                    // collison
                                    self.registers[0xF] = 1;
                                }
                                self.frame_buffer[screen_idx] ^= 1;
                            }
                        }
                    }
                }
            }
            0xE => todo!(),
            0xF => todo!(),
            _ => panic!("Error: No instruction found."),
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom(String::from("./roms/1-chip8-logo.ch8"));
    println!("{}", chip8)
}
