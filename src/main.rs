use cpu_core::cpu;
mod cpu_core {
    pub mod cpu;
    pub mod memory;
    pub mod registers;
    pub mod instruction;
    pub mod FlagsRegister;

}
/* 0x0000 to 0x00FF are the ROM  */
// the prefix byte is 0xCB

fn main() {
    
    let mut cpu = cpu::CPU::new();
    loop {
        cpu.step();
    }
}
