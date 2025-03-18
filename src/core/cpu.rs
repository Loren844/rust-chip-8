use crate::core::memory::Memory;
use crate::core::timers::Timers;
use crate::core::stack::Stack;

pub struct Cpu {
    pub memory: Memory,
    pub timers: Timers,
    pub program_counter: u16,
    pub stack: Stack,
    pub index_register: u16,
    pub v_registers: [u8; 16],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: Memory::new(),
            timers: Timers::new(),
            program_counter: 0x200,
            stack: Stack::new(),
            index_register: 0,
            v_registers: [0; 16],
        }
    }

    pub fn read_instruction(&mut self) -> u16 {
        let b1 = self.memory.get_instruction(self.program_counter);
        let b2 = self.memory.get_instruction(self.program_counter + 1);
        
        self.program_counter += 2;
        return ((b1 as u16) << 8) | (b2 as u16);
    }
}
