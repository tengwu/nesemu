use super::{
    instruction::{ADCInst, Instruction, LDAInst, InstExe},
    memory::Memory,
    monitor::MonitorState,
};
// use super::instruction::

pub struct StatusRegister {
    pub negative: bool,
    pub overflow: bool,
    pub b_high: bool,
    pub b_low: bool,
    pub decimal: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

impl std::convert::From<&StatusRegister> for u8 {
    fn from(x: &StatusRegister) -> Self {
        ((x.negative as u8) << 7)
            | ((x.overflow as u8) << 6)
            | ((x.b_high as u8) << 5)
            | ((x.b_low as u8) << 4)
            | ((x.decimal as u8) << 3)
            | ((x.interrupt_disable as u8) << 2)
            | ((x.zero as u8) << 1)
            | (x.carry as u8)
    }
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister {
            negative: false,
            overflow: false,
            b_high: false,
            b_low: false,
            decimal: false,
            interrupt_disable: false,
            zero: false,
            carry: false,
        }
    }
}

pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: StatusRegister,
    pub pc: u16,
}

enum StatusRegBit {
    Negative,
    Overflow,
    BHigh,
    BLow,
    Decimal,
    InterruptDisable,
    Zero,
    Carry,
}

const OPERAND_SINGLE_BYTE: u8 = 1;
const OPERAND_DOUBLE_BYTES: u8 = 2;
const OPERAND_NON: u8 = 0;

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            status: StatusRegister::new(),
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = StatusRegister::new();
        self.pc = 0; /* TODO: Does PC go from 0x0? */
    }

    fn _resolve_imm_opnd(&mut self, memory: &Memory) -> u16 {
        let operand = memory.read(self.pc);
        self.pc += 1;
        operand as u16
    }

    fn _resolve_zero_page_opnd(&mut self, memory: &Memory) -> u16 {
        let zero_page_addr = memory.read(self.pc);
        self.pc += 1;
        zero_page_addr as u16
    }

    fn _resolve_zero_page_x_opnd(&mut self, memory: &Memory) -> u16 {
        let zero_page_addr = memory.read(self.pc);
        self.pc += 1;
        (zero_page_addr + self.x) as u16
    }

    fn _resolve_zero_page_y_opnd(&mut self, memory: &Memory) -> u16 {
        let zero_page_addr = memory.read(self.pc);
        self.pc += 1;
        (zero_page_addr + self.y) as u16
    }

    fn _resolve_absolute_opnd(&mut self, memory: &Memory) -> u16 {
        let low_byte = memory.read(self.pc);
        self.pc += 1;
        let high_byte = memory.read(self.pc);
        self.pc += 1;
        (high_byte as u16) << 8 | low_byte as u16
    }

    fn _resolve_absolute_x_opnd(&mut self, memory: &Memory) -> u16 {
        let low_byte = memory.read(self.pc);
        self.pc += 1;
        let high_byte = memory.read(self.pc);
        self.pc += 1;
        ((high_byte as u16) << 8 | low_byte as u16) + self.x as u16
    }

    fn _resolve_absolute_y_opnd(&mut self, memory: &Memory) -> u16 {
        let low_byte = memory.read(self.pc);
        self.pc += 1;
        let high_byte = memory.read(self.pc);
        self.pc += 1;
        ((high_byte as u16) << 8 | low_byte as u16) + self.y as u16
    }

    fn _resolve_indirect_opnd(&mut self, memory: &Memory) -> u16 {
        let low_byte = memory.read(self.pc);
        self.pc += 1;
        let high_byte = memory.read(self.pc);
        self.pc += 1;
        let indirect_addr = ((high_byte as u16) << 8 | low_byte as u16) + self.x as u16;
        let low_byte = memory.read(indirect_addr);
        let high_byte = memory.read(indirect_addr + 1);
        (high_byte as u16) << 8 | low_byte as u16
    }

    fn _resolve_index_indirect_opnd(&mut self, memory: &Memory) -> u16 {
        let base_addr = memory.read(self.pc);
        self.pc += 1;
        let addr = (base_addr + self.x) as u16;
        let low_byte = memory.read(addr);
        let high_byte = memory.read(addr + 1);
        (high_byte as u16) << 8 | low_byte as u16
    }

    fn _resolve_indirect_index_opnd(&mut self, memory: &Memory) -> u16 {
        let indirect_addr = memory.read(self.pc);
        self.pc += 1;
        let low_byte = memory.read(indirect_addr as u16);
        let high_byte = memory.read((indirect_addr + 1) as u16);
        ((high_byte as u16) << 8 | low_byte as u16) + self.y as u16
    }

    fn fetch_inst(&mut self, memory: &Memory) -> Instruction {
        let opcode = memory.read(self.pc);
        self.pc += 1;

        /* TODO: Refactor the process to fetch operand for some insts */
        match opcode {
            0x69 => {
                Instruction::ADC(opcode, self._resolve_imm_opnd(memory), OPERAND_SINGLE_BYTE)
            }
            0x65 => {
                Instruction::ADC(opcode, self._resolve_zero_page_opnd(memory), OPERAND_SINGLE_BYTE)
            }
            0x75 => {
                Instruction::ADC(opcode, self._resolve_zero_page_x_opnd(memory), OPERAND_SINGLE_BYTE)
            }
            0x6D => {
                Instruction::ADC(opcode, self._resolve_absolute_opnd(memory), OPERAND_DOUBLE_BYTES)
            }
            0x7D => {
                Instruction::ADC(opcode, self._resolve_absolute_x_opnd(memory), OPERAND_DOUBLE_BYTES)
            }
            0x79 => {
                Instruction::ADC(opcode, self._resolve_absolute_y_opnd(memory), OPERAND_DOUBLE_BYTES)
            }
            0x61 => {
                Instruction::ADC(opcode, self._resolve_index_indirect_opnd(memory), OPERAND_SINGLE_BYTE)
            }
            0x71 => {
                Instruction::ADC(opcode, self._resolve_indirect_index_opnd(memory), OPERAND_SINGLE_BYTE)
            }
            0xA9 => {
                let operand = memory.read(self.pc);
                self.pc += 1;
                Instruction::LDA(opcode, operand as u16, OPERAND_SINGLE_BYTE)
            }
            0xFF => Instruction::MyHalt(255),
            _ => Instruction::Unknown(opcode),
        }
    }

    /* This method is for debug */
    pub fn get_next_inst(&mut self, memory: &Memory) -> Instruction {
        /*
         * self.fetch_inst function increased PC automatically,
         * so we need to recovery it.
         **/
        let inst = self.fetch_inst(memory);

        self.pc -= 1; /* Eat opcode byte */

        /* Eat operand byte(s) */
        match inst.get_operand_size() {
            OPERAND_SINGLE_BYTE => self.pc -= 1,
            OPERAND_DOUBLE_BYTES => self.pc -= 2,
            _ => ()
        }

        inst
    }

    pub fn execute(&mut self, memory: &mut Memory) {
        let inst = self.fetch_inst(memory);
        self.interpret(&inst, memory);
    }

    /* Stub method for test */
    pub fn interpret(&mut self, inst: &Instruction, memory: &mut Memory) {
        match inst {
            /* TODO: Refactor this piece of code to a small framework */
            Instruction::LDA(_, _, _) => LDAInst::execute(self, inst, memory),
            Instruction::ADC(_, _, _) => ADCInst::execute(self, inst, memory),
            Instruction::MyHalt(_) => {
                self.pc -= 1;
            }
            _ => panic!("Unimplemented instruction"),
        }
    }

    /*
     * We have this method because all instructions
     * set Negative and Zero bits simoutaniously.
     */
    pub fn set_nz(&mut self, target_reg: u8) {
        self.status.negative = (target_reg & 0b1000_0000) != 0;
        self.status.zero = target_reg == 0;
    }
}

impl MonitorState for CPU {
    fn print_state(&self) {
        println!(
            "A: 0x{:X}\nX: 0x{:X}\nY: 0x{:X}\nflags: 0b{:b}",
            self.a,
            self.x,
            self.y,
            u8::from(&self.status)
        );
    }
}
