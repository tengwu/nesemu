use super::cpu::CPU;
use super::memory::Memory;
use core::panic;

#[derive(Debug)]
pub enum Instruction {
    /* InstructionName(Opcode, Operand, Oprand byte size) */
    ADC(u8, u16, u8),
    AND(u8, u16, u8),
    ASL(u8, u16, u8),
    BCC(u8, u16, u8),
    BCS(u8, u16, u8),
    BEQ(u8, u16, u8),
    BIT(u8, u16, u8),
    BMI(u8, u16, u8),
    BNE(u8, u16, u8),
    BPL(u8, u16, u8),
    BRK(u8, u16, u8),
    BVC(u8, u16, u8),
    BVS(u8, u16, u8),
    CLC(u8, u16, u8),
    CLD(u8, u16, u8),
    CLI(u8, u16, u8),
    CLV(u8, u16, u8),
    CMP(u8, u16, u8),
    CPX(u8, u16, u8),
    CPY(u8, u16, u8),
    DEC(u8, u16, u8),
    DEX(u8, u16, u8),
    DEY(u8, u16, u8),
    EOR(u8, u16, u8),
    INC(u8, u16, u8),
    INX(u8, u16, u8),
    INY(u8, u16, u8),
    JMP(u8, u16, u8),
    JSR(u8, u16, u8),
    LDA(u8, u16, u8),
    LDX(u8, u16, u8),
    LDY(u8, u16, u8),
    LSR(u8, u16, u8),
    NOP(u8, u16, u8),
    ORA(u8, u16, u8),
    PHA(u8, u16, u8),
    PHP(u8, u16, u8),
    PLA(u8, u16, u8),
    PLP(u8, u16, u8),
    ROL(u8, u16, u8),
    ROR(u8, u16, u8),
    RTI(u8, u16, u8),
    RTS(u8, u16, u8),
    SBC(u8, u16, u8),
    SEC(u8, u16, u8),
    SED(u8, u16, u8),
    SEI(u8, u16, u8),
    STA(u8, u16, u8),
    STX(u8, u16, u8),
    STY(u8, u16, u8),
    TAX(u8, u16, u8),
    TAY(u8, u16, u8),
    TSX(u8, u16, u8),
    TXA(u8, u16, u8),
    TXS(u8, u16, u8),
    TYA(u8, u16, u8),

    MyHalt(u8),
    Unknown(u8),
}

impl Instruction {
    pub fn get_contents(&self) -> (u8, u16, u8) {
        match self {
              Instruction::ADC(opcode, operand, operand_size)
            | Instruction::AND(opcode, operand, operand_size)
            | Instruction::ASL(opcode, operand, operand_size)
            | Instruction::BCC(opcode, operand, operand_size)
            | Instruction::BCS(opcode, operand, operand_size)
            | Instruction::BEQ(opcode, operand, operand_size)
            | Instruction::BIT(opcode, operand, operand_size)
            | Instruction::BMI(opcode, operand, operand_size)
            | Instruction::BNE(opcode, operand, operand_size)
            | Instruction::BPL(opcode, operand, operand_size)
            | Instruction::BRK(opcode, operand, operand_size)
            | Instruction::BVC(opcode, operand, operand_size)
            | Instruction::BVS(opcode, operand, operand_size)
            | Instruction::CLC(opcode, operand, operand_size)
            | Instruction::CLD(opcode, operand, operand_size)
            | Instruction::CLI(opcode, operand, operand_size)
            | Instruction::CLV(opcode, operand, operand_size)
            | Instruction::CMP(opcode, operand, operand_size)
            | Instruction::CPX(opcode, operand, operand_size)
            | Instruction::CPY(opcode, operand, operand_size)
            | Instruction::DEC(opcode, operand, operand_size)
            | Instruction::DEX(opcode, operand, operand_size)
            | Instruction::DEY(opcode, operand, operand_size)
            | Instruction::EOR(opcode, operand, operand_size)
            | Instruction::INC(opcode, operand, operand_size)
            | Instruction::INX(opcode, operand, operand_size)
            | Instruction::INY(opcode, operand, operand_size)
            | Instruction::JMP(opcode, operand, operand_size)
            | Instruction::JSR(opcode, operand, operand_size)
            | Instruction::LDA(opcode, operand, operand_size)
            | Instruction::LDX(opcode, operand, operand_size)
            | Instruction::LDY(opcode, operand, operand_size)
            | Instruction::LSR(opcode, operand, operand_size)
            | Instruction::NOP(opcode, operand, operand_size)
            | Instruction::ORA(opcode, operand, operand_size)
            | Instruction::PHA(opcode, operand, operand_size)
            | Instruction::PHP(opcode, operand, operand_size)
            | Instruction::PLA(opcode, operand, operand_size)
            | Instruction::PLP(opcode, operand, operand_size)
            | Instruction::ROL(opcode, operand, operand_size)
            | Instruction::ROR(opcode, operand, operand_size)
            | Instruction::RTI(opcode, operand, operand_size)
            | Instruction::RTS(opcode, operand, operand_size)
            | Instruction::SBC(opcode, operand, operand_size)
            | Instruction::SEC(opcode, operand, operand_size)
            | Instruction::SED(opcode, operand, operand_size)
            | Instruction::SEI(opcode, operand, operand_size)
            | Instruction::STA(opcode, operand, operand_size)
            | Instruction::STX(opcode, operand, operand_size)
            | Instruction::STY(opcode, operand, operand_size)
            | Instruction::TAX(opcode, operand, operand_size)
            | Instruction::TAY(opcode, operand, operand_size)
            | Instruction::TSX(opcode, operand, operand_size)
            | Instruction::TXA(opcode, operand, operand_size)
            | Instruction::TXS(opcode, operand, operand_size)
            | Instruction::TYA(opcode, operand, operand_size) => (*opcode, *operand, *operand_size),
            Instruction::MyHalt(opcode) => (255, 0, 0),
            _ => {
                panic!("Unknown instruction {:?}", self);
            }
        }
    }

    pub fn get_operand(&self) -> u16 {
        self.get_contents().1
    }

    pub fn get_opcode(&self) -> u8 {
        self.get_contents().0
    }

    pub fn get_operand_size(&self) -> u8 {
        self.get_contents().2
    }
}

pub trait InstExe {
    fn execute(cpu: &mut CPU, inst: &Instruction, memory: &mut Memory);
}

pub struct ADCInst;
impl InstExe for ADCInst {
    fn execute(cpu: &mut CPU, inst: &Instruction, memory: &mut Memory) {
        let operand = inst.get_operand();
        (_, cpu.status.overflow) = (cpu.a as i8).overflowing_add(operand as i8);
        (cpu.a, cpu.status.carry) = cpu.a.overflowing_add(operand as u8);
        cpu.set_nz(cpu.a);
    }
}

pub struct LDAInst;
impl InstExe for LDAInst {
    fn execute(cpu: &mut CPU, inst: &Instruction, memory: &mut Memory) {
        let operand = inst.get_operand();
        cpu.a = operand as u8;
        cpu.set_nz(cpu.a);
    }
}
