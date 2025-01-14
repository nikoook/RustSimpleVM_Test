use crate::memory::{Addressable, LinearMemory};

pub enum Register {
    A, B, C, M, SP, PC, BP, FLAGS,
}


pub struct Machine {
    // 寄存器
    pub registers: [u16; 8],
    // 内存
    pub memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Self {
        Self {registers: [0; 8], memory: Box::new(LinearMemory::new(8 * 1024))}
    }
    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        println!("{} @ {}", instruction, pc);
        Ok(())
    }  
}
