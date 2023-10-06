use super::{
    instruction::{ADCInst, InstEXE, Instruction, LDAInst},
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

const OPERAND_SINGLE_ENCODING: u8 = 1;
const OPERAND_DOUBLE_ENCODING: u8 = 2;
const OPERAND_NON: u8 = 0;

#[derive(Debug)]
pub enum OperandType {
    Imm,
    Mem,
    Implied,
    Accumulator,
    Relative,
    Indirect,
}

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
            0x69 => Instruction::ADC(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),
            0x65 => Instruction::ADC(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),
            0x75 => Instruction::ADC(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x6D => Instruction::ADC(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x7D => Instruction::ADC(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x79 => Instruction::ADC(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x61 => Instruction::ADC(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x71 => Instruction::ADC(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x29 => Instruction::AND(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0x25 => Instruction::AND(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x35 => Instruction::AND(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x2D => Instruction::AND(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x3D => Instruction::AND(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x39 => Instruction::AND(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x21 => Instruction::AND(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x31 => Instruction::AND(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x0A => Instruction::ASL(opcode, 0, OPERAND_NON, OperandType::Accumulator),

            0x06 => Instruction::ASL(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x16 => Instruction::ASL(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x0E => Instruction::ASL(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x1E => Instruction::ASL(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x90 => Instruction::BCC(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0xB0 => Instruction::BCS(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0xF0 => Instruction::BEQ(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0x24 => Instruction::BIT(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x2C => Instruction::BIT(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x30 => Instruction::BMI(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0xD0 => Instruction::BNE(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0x10 => Instruction::BPL(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0x00 => Instruction::BRK(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x50 => Instruction::BVC(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0x70 => Instruction::BVS(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Relative,
            ),

            0x18 => Instruction::CLC(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xD8 => Instruction::CLD(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x58 => Instruction::CLI(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xB8 => Instruction::CLV(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xC9 => Instruction::CMP(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0xC5 => Instruction::CMP(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xD5 => Instruction::CMP(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xCD => Instruction::CMP(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xDD => Instruction::CMP(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xD9 => Instruction::CMP(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xC1 => Instruction::CMP(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xD1 => Instruction::CMP(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xE0 => Instruction::CPX(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0xE4 => Instruction::CPX(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xEC => Instruction::CPX(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xC0 => Instruction::CPY(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0xC4 => Instruction::CPY(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xCC => Instruction::CPY(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xC6 => Instruction::DEC(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xD6 => Instruction::DEC(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xCE => Instruction::DEC(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xDE => Instruction::DEC(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xCA => Instruction::DEX(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x88 => Instruction::DEY(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x49 => Instruction::EOR(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0x45 => Instruction::EOR(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x55 => Instruction::EOR(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x4D => Instruction::EOR(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x5D => Instruction::EOR(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x59 => Instruction::EOR(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x41 => Instruction::EOR(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x51 => Instruction::EOR(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xE6 => Instruction::INC(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xF6 => Instruction::INC(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xEE => Instruction::INC(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xFE => Instruction::INC(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xE8 => Instruction::INX(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xC8 => Instruction::INY(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x4C => Instruction::JMP(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x6C => Instruction::JMP(
                opcode,
                self._resolve_indirect_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Indirect,
            ),

            0x20 => Instruction::JSR(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xA9 => Instruction::LDA(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0xA5 => Instruction::LDA(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xB5 => Instruction::LDA(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xAD => Instruction::LDA(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xBD => Instruction::LDA(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xB9 => Instruction::LDA(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xA1 => Instruction::LDA(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xB1 => Instruction::LDA(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xA2 => Instruction::LDX(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0xA6 => Instruction::LDX(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xB6 => Instruction::LDX(
                opcode,
                self._resolve_zero_page_y_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xAE => Instruction::LDX(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xBE => Instruction::LDX(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xA0 => Instruction::LDY(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0xA4 => Instruction::LDY(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xB4 => Instruction::LDY(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xAC => Instruction::LDY(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xBC => Instruction::LDY(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x4A => Instruction::LSR(opcode, 0, OPERAND_NON, OperandType::Accumulator),

            0x46 => Instruction::LSR(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x56 => Instruction::LSR(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x4E => Instruction::LSR(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x5E => Instruction::LSR(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xEA => Instruction::NOP(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x09 => Instruction::ORA(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0x05 => Instruction::ORA(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x15 => Instruction::ORA(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x0D => Instruction::ORA(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x1D => Instruction::ORA(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x19 => Instruction::ORA(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x01 => Instruction::ORA(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x11 => Instruction::ORA(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x48 => Instruction::PHA(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x08 => Instruction::PHP(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x68 => Instruction::PLA(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x28 => Instruction::PLP(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x2A => Instruction::ROL(opcode, 0, OPERAND_NON, OperandType::Accumulator),

            0x26 => Instruction::ROL(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x36 => Instruction::ROL(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x2E => Instruction::ROL(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x3E => Instruction::ROL(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x6A => Instruction::ROR(opcode, 0, OPERAND_NON, OperandType::Accumulator),

            0x66 => Instruction::ROR(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x76 => Instruction::ROR(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x6E => Instruction::ROR(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x7E => Instruction::ROR(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x40 => Instruction::RTI(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x60 => Instruction::RTS(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xE9 => Instruction::SBC(
                opcode,
                self._resolve_imm_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Imm,
            ),

            0xE5 => Instruction::SBC(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xF5 => Instruction::SBC(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xED => Instruction::SBC(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xFD => Instruction::SBC(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xF9 => Instruction::SBC(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xE1 => Instruction::SBC(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0xF1 => Instruction::SBC(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x38 => Instruction::SEC(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xF8 => Instruction::SED(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x78 => Instruction::SEI(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x85 => Instruction::STA(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x95 => Instruction::STA(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x8D => Instruction::STA(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x9D => Instruction::STA(
                opcode,
                self._resolve_absolute_x_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x99 => Instruction::STA(
                opcode,
                self._resolve_absolute_y_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x81 => Instruction::STA(
                opcode,
                self._resolve_index_indirect_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x91 => Instruction::STA(
                opcode,
                self._resolve_indirect_index_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x86 => Instruction::STX(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x96 => Instruction::STX(
                opcode,
                self._resolve_zero_page_y_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x8E => Instruction::STX(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0x84 => Instruction::STY(
                opcode,
                self._resolve_zero_page_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x94 => Instruction::STY(
                opcode,
                self._resolve_zero_page_x_opnd(memory),
                OPERAND_SINGLE_ENCODING,
                OperandType::Mem,
            ),

            0x8C => Instruction::STY(
                opcode,
                self._resolve_absolute_opnd(memory),
                OPERAND_DOUBLE_ENCODING,
                OperandType::Mem,
            ),

            0xAA => Instruction::TAX(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xA8 => Instruction::TAY(opcode, 0, OPERAND_NON, OperandType::Implied),

            0xBA => Instruction::TSX(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x8A => Instruction::TXA(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x9A => Instruction::TXS(opcode, 0, OPERAND_NON, OperandType::Implied),

            0x98 => Instruction::TYA(opcode, 0, OPERAND_NON, OperandType::Implied),

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
            OPERAND_SINGLE_ENCODING => self.pc -= 1,
            OPERAND_DOUBLE_ENCODING => self.pc -= 2,
            _ => (),
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
            Instruction::LDA(_, _, _, _) => LDAInst::execute(self, inst, memory),
            Instruction::ADC(_, _, _, _) => ADCInst::execute(self, inst, memory),
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
