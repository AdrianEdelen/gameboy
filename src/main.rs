use cpu_core::cpu;
use std::fs;
use std::env;
use std::io::Error;
mod cpu_core {
    pub mod cpu;
    pub mod memory;
    pub mod registers;
    pub mod instruction;
    pub mod flags_register;

}
/* 0x0000 to 0x00FF are the ROM  */
// the prefix byte is 0xCB

fn main() {
    
    let mut cpu = cpu::CPU::new();
    let a = env::current_dir();
    match load_file("cpu_instrs.gb") {
        Ok(data) => cpu.load_rom(data),
        Err(e) => panic!("Failed to load file: {:?}", e),
    }
    
    loop {
        cpu.step();
    }
}

fn load_file(file_path: &str) -> Result<Vec<u8>, Error> {
    let data = fs::read(file_path)?;
    Ok(data)
    
}
