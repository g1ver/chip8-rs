use minifb::{Key, Window, WindowOptions};
use std::fs;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const MEMORY_SIZE: usize = 4096;
const STEP_DEBUG: bool = false;

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
        let nn = (instruction & 0x00FF) as u8;
        let nnn = (instruction & 0x0FFF) as u16;

        match opcode {
            0x00 => match nn {
                0xE0 => {
                    // 00E0 - CLS
                    self.frame_buffer.fill(0);
                }
                0xEE => {
                    todo!()
                }
                _ => panic!("Unknown opcode: {:#06x}", opcode),
            },
            0x1 => {
                // 1nnn - JP addr
                self.program_counter = nnn;
            }
            0x2 => todo!(),
            0x3 => {
                // 3xnn - SE Vx, byte
                if self.registers[x] == nn {
                    self.program_counter += 2;
                }
            }
            0x4 => {
                // 4xnn - SNE Vx, byte
                if self.registers[x] != nn {
                    self.program_counter += 2;
                }
            }
            0x5 => {
                // 5xy0 - SE Vx, Vy
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            }
            0x6 => {
                // 6xnn - LD Vx, byte
                self.registers[x] = nn;
            }
            0x7 => {
                // 7xnn - ADD Vx, byte
                self.registers[x] = self.registers[x] + nn;
            }
            0x8 => {
                match n {
                    1 => {
                        // 8xy1 - OR Vx, Vy
                        self.registers[x] = self.registers[x] | self.registers[y];
                    }
                    2 => {
                        // 8xy2 - AND Vx, Vy
                        self.registers[x] = self.registers[x] & self.registers[y];
                    }
                    3 => {
                        // 8xy3 - XOR Vx, Vy
                        self.registers[x] = self.registers[x] ^ self.registers[y];
                    }
                    4 => {
                        // 8xy4 - ADD Vx, Vy
                        let (res, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[x] = res;
                        self.registers[0xF] = overflow as u8;
                    }
                    5 => {
                        // 8xy5 - SUB Vx, Vy
                        self.registers[0xF] = (self.registers[x] > self.registers[y]) as u8;
                        self.registers[x] = self.registers[x] - self.registers[y];
                    }
                    6 => {
                        // 8xy6 - SHR Vx {, Vy}
                        // Ambiguous instruction - might need to allow for configured behavior
                        self.registers[0xF] = ((self.registers[x] & 1) == 1) as u8;
                        self.registers[x] = self.registers[x] >> 1;
                    }
                    _ => panic!("Unknown opcode: {:#06x}", opcode),
                }
            }
            0x9 => {
                // 9xy0 - SNE Vx, Vy
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            }
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
            _ => panic!("Unknown opcode: {:#06x}", opcode),
        }
    }

    fn tick(&mut self) {
        let instruction = self.fetch();
        self.decode_exec(instruction);
    }
}

fn update_minifb_buffer(chip8_buffer: &[u8; HEIGHT * WIDTH], minifb_buffer: &mut [u32]) {
    for i in 0..(HEIGHT * WIDTH) {
        minifb_buffer[i] = if chip8_buffer[i] == 1 {
            0x33FF33
        } else {
            0x000800
        };
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom(String::from("./roms/2-ibm-logo.ch8"));
    // println!("{}", chip8);

    let mut window = Window::new(
        "CHIP-8 Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if STEP_DEBUG {
            if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
                chip8.tick();
                println!("Stepped to PC: {:#06x}", chip8.program_counter);
            }
        } else {
            // running ~10 cycles per 60Hz frame ~600Hz.
            for _ in 0..10 {
                chip8.tick();
            }
        }

        // TODO: chip8.update_timers();

        update_minifb_buffer(&chip8.frame_buffer, &mut screen_buffer);

        window
            .update_with_buffer(&screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
