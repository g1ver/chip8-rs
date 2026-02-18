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
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom(String::from("./roms/1-chip8-logo.ch8"));
    println!("{}", chip8)
}
