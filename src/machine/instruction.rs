use super::cpu::{ CPU, OperandType };
use super::memory::Memory;
use core::panic;

#[derive(Debug)]
pub enum Instruction {
    /* InstructionName(Opcode, Operand, Oprand byte size) */
    ADC(u8, u16, u8, OperandType),
    AND(u8, u16, u8, OperandType),
    ASL(u8, u16, u8, OperandType),
    BCC(u8, u16, u8, OperandType),
    BCS(u8, u16, u8, OperandType),
    BEQ(u8, u16, u8, OperandType),
    BIT(u8, u16, u8, OperandType),
    BMI(u8, u16, u8, OperandType),
    BNE(u8, u16, u8, OperandType),
    BPL(u8, u16, u8, OperandType),
    BRK(u8, u16, u8, OperandType),
    BVC(u8, u16, u8, OperandType),
    BVS(u8, u16, u8, OperandType),
    CLC(u8, u16, u8, OperandType),
    CLD(u8, u16, u8, OperandType),
    CLI(u8, u16, u8, OperandType),
    CLV(u8, u16, u8, OperandType),
    CMP(u8, u16, u8, OperandType),
    CPX(u8, u16, u8, OperandType),
    CPY(u8, u16, u8, OperandType),
    DEC(u8, u16, u8, OperandType),
    DEX(u8, u16, u8, OperandType),
    DEY(u8, u16, u8, OperandType),
    EOR(u8, u16, u8, OperandType),
    INC(u8, u16, u8, OperandType),
    INX(u8, u16, u8, OperandType),
    INY(u8, u16, u8, OperandType),
    JMP(u8, u16, u8, OperandType),
    JSR(u8, u16, u8, OperandType),
    LDA(u8, u16, u8, OperandType),
    LDX(u8, u16, u8, OperandType),
    LDY(u8, u16, u8, OperandType),
    LSR(u8, u16, u8, OperandType),
    NOP(u8, u16, u8, OperandType),
    ORA(u8, u16, u8, OperandType),
    PHA(u8, u16, u8, OperandType),
    PHP(u8, u16, u8, OperandType),
    PLA(u8, u16, u8, OperandType),
    PLP(u8, u16, u8, OperandType),
    ROL(u8, u16, u8, OperandType),
    ROR(u8, u16, u8, OperandType),
    RTI(u8, u16, u8, OperandType),
    RTS(u8, u16, u8, OperandType),
    SBC(u8, u16, u8, OperandType),
    SEC(u8, u16, u8, OperandType),
    SED(u8, u16, u8, OperandType),
    SEI(u8, u16, u8, OperandType),
    STA(u8, u16, u8, OperandType),
    STX(u8, u16, u8, OperandType),
    STY(u8, u16, u8, OperandType),
    TAX(u8, u16, u8, OperandType),
    TAY(u8, u16, u8, OperandType),
    TSX(u8, u16, u8, OperandType),
    TXA(u8, u16, u8, OperandType),
    TXS(u8, u16, u8, OperandType),
    TYA(u8, u16, u8, OperandType),

    MyHalt(u8),
    Unknown(u8),
}

impl Instruction {
    pub fn get_contents(&self) -> (u8, u16, u8, &OperandType) {
        match self {
              Instruction::ADC(opcode, operand, operand_size, operand_type)
            | Instruction::AND(opcode, operand, operand_size, operand_type)
            | Instruction::ASL(opcode, operand, operand_size, operand_type)
            | Instruction::BCC(opcode, operand, operand_size, operand_type)
            | Instruction::BCS(opcode, operand, operand_size, operand_type)
            | Instruction::BEQ(opcode, operand, operand_size, operand_type)
            | Instruction::BIT(opcode, operand, operand_size, operand_type)
            | Instruction::BMI(opcode, operand, operand_size, operand_type)
            | Instruction::BNE(opcode, operand, operand_size, operand_type)
            | Instruction::BPL(opcode, operand, operand_size, operand_type)
            | Instruction::BRK(opcode, operand, operand_size, operand_type)
            | Instruction::BVC(opcode, operand, operand_size, operand_type)
            | Instruction::BVS(opcode, operand, operand_size, operand_type)
            | Instruction::CLC(opcode, operand, operand_size, operand_type)
            | Instruction::CLD(opcode, operand, operand_size, operand_type)
            | Instruction::CLI(opcode, operand, operand_size, operand_type)
            | Instruction::CLV(opcode, operand, operand_size, operand_type)
            | Instruction::CMP(opcode, operand, operand_size, operand_type)
            | Instruction::CPX(opcode, operand, operand_size, operand_type)
            | Instruction::CPY(opcode, operand, operand_size, operand_type)
            | Instruction::DEC(opcode, operand, operand_size, operand_type)
            | Instruction::DEX(opcode, operand, operand_size, operand_type)
            | Instruction::DEY(opcode, operand, operand_size, operand_type)
            | Instruction::EOR(opcode, operand, operand_size, operand_type)
            | Instruction::INC(opcode, operand, operand_size, operand_type)
            | Instruction::INX(opcode, operand, operand_size, operand_type)
            | Instruction::INY(opcode, operand, operand_size, operand_type)
            | Instruction::JMP(opcode, operand, operand_size, operand_type)
            | Instruction::JSR(opcode, operand, operand_size, operand_type)
            | Instruction::LDA(opcode, operand, operand_size, operand_type)
            | Instruction::LDX(opcode, operand, operand_size, operand_type)
            | Instruction::LDY(opcode, operand, operand_size, operand_type)
            | Instruction::LSR(opcode, operand, operand_size, operand_type)
            | Instruction::NOP(opcode, operand, operand_size, operand_type)
            | Instruction::ORA(opcode, operand, operand_size, operand_type)
            | Instruction::PHA(opcode, operand, operand_size, operand_type)
            | Instruction::PHP(opcode, operand, operand_size, operand_type)
            | Instruction::PLA(opcode, operand, operand_size, operand_type)
            | Instruction::PLP(opcode, operand, operand_size, operand_type)
            | Instruction::ROL(opcode, operand, operand_size, operand_type)
            | Instruction::ROR(opcode, operand, operand_size, operand_type)
            | Instruction::RTI(opcode, operand, operand_size, operand_type)
            | Instruction::RTS(opcode, operand, operand_size, operand_type)
            | Instruction::SBC(opcode, operand, operand_size, operand_type)
            | Instruction::SEC(opcode, operand, operand_size, operand_type)
            | Instruction::SED(opcode, operand, operand_size, operand_type)
            | Instruction::SEI(opcode, operand, operand_size, operand_type)
            | Instruction::STA(opcode, operand, operand_size, operand_type)
            | Instruction::STX(opcode, operand, operand_size, operand_type)
            | Instruction::STY(opcode, operand, operand_size, operand_type)
            | Instruction::TAX(opcode, operand, operand_size, operand_type)
            | Instruction::TAY(opcode, operand, operand_size, operand_type)
            | Instruction::TSX(opcode, operand, operand_size, operand_type)
            | Instruction::TXA(opcode, operand, operand_size, operand_type)
            | Instruction::TXS(opcode, operand, operand_size, operand_type)
            | Instruction::TYA(opcode, operand, operand_size, operand_type) => (*opcode, *operand, *operand_size, operand_type),
            Instruction::MyHalt(opcode) => (255, 0, 0, &OperandType::Imm),
            _ => {
                panic!("Unknown instruction {:?}", self);
            }
        }
    }

    pub fn get_operand(&self, memory: &mut Memory) -> u16 {
        match self.get_contents().3 {
            OperandType::Imm => self.get_contents().1,
            OperandType::Mem => memory.read(self.get_contents().1) as u16,
            _ => 0
        }
    }

    pub fn get_opcode(&self) -> u8 {
        self.get_contents().0
    }

    pub fn get_operand_size(&self) -> u8 {
        self.get_contents().2
    }
}

pub trait InstEXE {
    fn execute(cpu: &mut CPU, inst: &Instruction, memory: &mut Memory);
}

pub struct ADCInst;
impl InstEXE for ADCInst {
    fn execute(cpu: &mut CPU, inst: &Instruction, memory: &mut Memory) {
        let operand = inst.get_operand(memory);
        (_, cpu.status.overflow) = (cpu.a as i8).overflowing_add(operand as i8);
        (cpu.a, cpu.status.carry) = cpu.a.overflowing_add(operand as u8);
        cpu.set_nz(cpu.a);
    }
}

pub struct LDAInst;
impl InstEXE for LDAInst {
    fn execute(cpu: &mut CPU, inst: &Instruction, memory: &mut Memory) {
        let operand = inst.get_operand(memory);
        cpu.a = operand as u8;
        cpu.set_nz(cpu.a);
    }
}
