mod gameboy;
mod cartridge;
mod cpu;
mod register;
mod register_pair;
mod memory_manager;
mod display_manager;
mod interrupt_handler;
mod gamepad;
mod instructions;

use gameboy::*;

fn main() {
    let mut gameboy = Gameboy::new();
    loop {
        if !gameboy.step() {
            break;
        }
    }
}