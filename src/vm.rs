
use crate::memory::{Addressable, LinearMemory};
#[derive(Debug)]
pub enum Register {
    A, B, C, M, SP, PC, BP, FLAGS,
}
#[derive(Debug)]
pub enum Op {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddRessable(Register, Register),
}

impl Op {
    pub fn value(&self) -> u8 {
        unsafe {*<*const _>::from(self).cast::<u8>()}
    } 

    // pub fn equal(x: u8, other: Self) {
    //     x == other.value()
    // }
}

fn parse_instruction(x: u16) -> Result<Op, String> {
    let op = (x & 0xff) as u8;
    match op {
        x if x == Op::Nop.value()=> Ok(Op::Nop),
        _ => Err(format!("unknow op")),
    }
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

    pub fn pop(&mut self) -> Result<u16, String> {
        let sp = self.registers[Register::SP as usize] - 2;
        if let Some(v) = self.memory.read2(sp)  {
            Ok(v)
        } else {
            Err(format!("memory read fault @ 0x{:x}", sp))
        }
    }

    pub fn push(&mut self, v: u16) -> Result<(), String> {
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write(sp, v as u8) {
            return Err(format!("memory write fault @ 0x{:X}", sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())   
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        self.registers[Register::PC as usize] = pc + 2;
        let op = parse_instruction(instruction)?;
        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => {
                self.push(v)
            },
            Op::PopReg(r) => {
                let value = self.pop()?;
                self.registers[ r as usize] = value;
                Ok(())
            },
            Op::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b)
            }
            _ => Err(format!("unknown operator 0x{:?}", op)),
        }
        
    }  
}
