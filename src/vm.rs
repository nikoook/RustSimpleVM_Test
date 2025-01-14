enum Register {
    A, B, C, M, SP, PC, BP, FLAGS,
}

trait Addressable {
    fn read(&self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, value: u8) -> bool;
    fn read2(&self, addr: u16) -> option<u16> {
        if let Some(x) = self.read(addr) {
            if let Some(x) = self.read(addr + 1) {
                return Some(x0 as u16 | (x1 as u16) << 8);
            }
        };
        None
    }
    
    fn write2(&mut self, addr: u16, value: u16) -> bool {
        let lower = value & 
    }
    
    fn copy(&mut self, from: u16, to: u16 , n: uszie) -> bool {
        
    }
}

struct Machine {
    // 寄存器
    registers: [u16; 8],
    // 内存
    memory: [u8; 5000],
}

impl Machine {
    pub fn new() -> Self {
        Self {registers: [0; 8], memory: [0; 5000]}
    }
    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = self.registers[Register::PC as u16];
        self.memory.read()
    }
    pub fn read()    
}
