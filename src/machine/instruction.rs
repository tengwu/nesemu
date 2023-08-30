use core::panic;

impl std::convert::From<Operand> for u16 {
  fn from(value: Operand) -> Self {
    ((value.first as u16) << 8) | (value.second as u16)
  }
}

impl std::convert::From<u16> for Operand {
  fn from(value: u16) -> Self {
    Operand {
      first: ((value >> 8) & 0x00FF) as u8,
      second: (value & 0x00FF) as u8
    }
  }
}

impl std::convert::From<u8> for Operand {
  fn from(value: u8) -> Self {
    Operand {
      first: value,
      second: 0
    }
  }
}

pub struct Operand {
  pub first: u8,
  pub second: u8
}

#[derive(Debug)]
pub enum OperandType {
  Immediate(u8),
  ZeroPage(u8),
  ZeroPageX(u8),
  Absolute(u16),
  AbsoluteX(u16),
  AbsoluteY(u16),
  IndirectX(u8),
  IndirectY(u8),
  NoOperands
}

impl OperandType {
  pub fn get_operand(&self) -> Operand {
    match self {
      OperandType::Absolute(x) => Operand::from(x.clone()),
      OperandType::AbsoluteX(x) => Operand::from(x.clone()),
      OperandType::AbsoluteY(x) => Operand::from(x.clone()),
      OperandType::Immediate(x) => Operand::from(x.clone()),
      OperandType::IndirectX(x) => Operand::from(x.clone()),
      OperandType::IndirectY(x) => Operand::from(x.clone()),
      OperandType::ZeroPage(x) => Operand::from(x.clone()),
      OperandType::ZeroPageX(x) => Operand::from(x.clone()),
      _ => Operand { first: 0, second: 0 }
    }
  }
}

#[derive(Debug)]
pub enum Instruction {
  ADC(OperandType),
  AND(OperandType),
  ASL(OperandType),
  BCC(OperandType),
  BCS(OperandType),
  BEQ(OperandType),
  BIT(OperandType),
  BMI(OperandType),
  BNE(OperandType),
  BPL(OperandType),
  BRK(OperandType),
  BVC(OperandType),
  BVS(OperandType),
  CLC(OperandType),
  CLD(OperandType),
  CLI(OperandType),
  CLV(OperandType),
  CMP(OperandType),
  CPX(OperandType),
  CPY(OperandType),
  DEC(OperandType),
  DEX(OperandType),
  DEY(OperandType),
  EOR(OperandType),
  INC(OperandType),
  INX(OperandType),
  INY(OperandType),
  JMP(OperandType),
  JSR(OperandType),
  LDA(OperandType),
  LDX(OperandType),
  LDY(OperandType),
  LSR(OperandType),
  NOP(OperandType),
  ORA(OperandType),
  PHA(OperandType),
  PHP(OperandType),
  PLA(OperandType),
  PLP(OperandType),
  ROL(OperandType),
  ROR(OperandType),
  RTI(OperandType),
  RTS(OperandType),
  SBC(OperandType),
  SEC(OperandType),
  SED(OperandType),
  SEI(OperandType),
  STA(OperandType),
  STX(OperandType),
  STY(OperandType),
  TAX(OperandType),
  TAY(OperandType),
  TSX(OperandType),
  TXA(OperandType),
  TXS(OperandType),
  TYA(OperandType),

  MyHalt,
  Unknown
}

impl Instruction {
  pub fn get_operand_type(&self) -> &OperandType {
    match self {
      Instruction::ADC(operand) |
      Instruction::AND(operand) |
      Instruction::ASL(operand) |
      Instruction::BCC(operand) |
      Instruction::BCS(operand) |
      Instruction::BEQ(operand) |
      Instruction::BIT(operand) |
      Instruction::BMI(operand) |
      Instruction::BNE(operand) |
      Instruction::BPL(operand) |
      Instruction::BRK(operand) |
      Instruction::BVC(operand) |
      Instruction::BVS(operand) |
      Instruction::CLC(operand) |
      Instruction::CLD(operand) |
      Instruction::CLI(operand) |
      Instruction::CLV(operand) |
      Instruction::CMP(operand) |
      Instruction::CPX(operand) |
      Instruction::CPY(operand) |
      Instruction::DEC(operand) |
      Instruction::DEX(operand) |
      Instruction::DEY(operand) |
      Instruction::EOR(operand) |
      Instruction::INC(operand) |
      Instruction::INX(operand) |
      Instruction::INY(operand) |
      Instruction::JMP(operand) |
      Instruction::JSR(operand) |
      Instruction::LDA(operand) |
      Instruction::LDX(operand) |
      Instruction::LDY(operand) |
      Instruction::LSR(operand) |
      Instruction::NOP(operand) |
      Instruction::ORA(operand) |
      Instruction::PHA(operand) |
      Instruction::PHP(operand) |
      Instruction::PLA(operand) |
      Instruction::PLP(operand) |
      Instruction::ROL(operand) |
      Instruction::ROR(operand) |
      Instruction::RTI(operand) |
      Instruction::RTS(operand) |
      Instruction::SBC(operand) |
      Instruction::SEC(operand) |
      Instruction::SED(operand) |
      Instruction::SEI(operand) |
      Instruction::STA(operand) |
      Instruction::STX(operand) |
      Instruction::STY(operand) |
      Instruction::TAX(operand) |
      Instruction::TAY(operand) |
      Instruction::TSX(operand) |
      Instruction::TXA(operand) |
      Instruction::TXS(operand) |
      Instruction::TYA(operand)
        => operand,
      Instruction::MyHalt => &OperandType::NoOperands,
      _ => {
        panic!("Unknown instruction {:?}", self);
      }
    }
  }
}