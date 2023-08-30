use super::{memory::Memory, instruction::{Instruction, OperandType, Operand}};

pub struct CPU {
  pub a: u8,
  pub x: u8,
  pub y: u8,
  pub status: u8,
  pub pc: u16,
}

enum StatusReg {
  Negative,
  Overflow,
  BHigh,
  BLow,
  Decimal,
  InterruptDisable,
  Zero,
  Carry
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      a: 0,
      x: 0,
      y: 0,
      status: 0,
      pc: 0,
    }
  }

  pub fn reset(&mut self) {
    self.a = 0;
    self.x = 0;
    self.y = 0;
    self.status = 0;
    self.pc = 0;  /* TODO: Does PC go from 0x0? */
  }

  fn fetch_inst(&mut self, memory: &Memory) -> Instruction {
    let opcode = memory.read(self.pc);
    self.pc += 1;

    /* TODO: Refactor the process to fetch operand for some insts */
    if opcode == 0xA9 {
      let operand = memory.read(self.pc);
      self.pc += 1;
      Instruction::LDA(OperandType::Immediate(operand))
    } else if opcode == 0xFF {
      Instruction::MyHalt
    } else {
      Instruction::Unknown
    }
  }

  /* This method is for debug */
  pub fn get_next_inst(&mut self, memory: &Memory) -> Instruction {
    /* 
     * self.fetch_inst function increased PC automatically,
     * so we need to recovery it.
     **/
    let inst = self.fetch_inst(memory);

    self.pc -= 1;  /* Eat opcode byte */

    match inst.get_operand_type() {
      OperandType::Absolute(_)  |
      OperandType::AbsoluteX(_) |
      OperandType::AbsoluteY(_)
        => self.pc -= 2,  /* Eat double operands */
      OperandType::Immediate(_)  |
      OperandType::IndirectX(_)  |
      OperandType::IndirectY(_)  |
      OperandType::ZeroPage(_)   |
      OperandType::ZeroPageX(_)
        => self.pc -= 1,  /* Eat single operand */
      OperandType::NoOperands => (),  /* Eat nothing when no operands */
    }

    inst
  }

  pub fn execute(&mut self, memory: &mut Memory) {
    let inst = self.fetch_inst(memory);
    self.interpret(&inst);
  }

  /* Stub method for test */
  pub fn interpret(&mut self, inst: &Instruction) {
    let operand_type = inst.get_operand_type();
    let operand: Operand = operand_type.get_operand();
    
    /* TODO: Get and set status register outside */
    match inst {
      Instruction::LDA(_) => {
        self.a = operand.first;

        /* TODO: Add struct StatusReg to Instruction */
        self.set_status_register(self.a, &vec![StatusReg::Negative, StatusReg::Zero]);
      }
      Instruction::MyHalt => {
        self.pc -= 1;
      }
      _ => panic!("Unknown instruction {:?}", inst)
    }
  }

  fn set_status_register(&mut self, target_reg: u8, status_bits: &Vec<StatusReg>) {
    /* TODO: Implement this method */
    for bit in status_bits {
      match bit {
        StatusReg::Negative =>
          self.status = self.status & (0b0111_1111 | target_reg),
        StatusReg::Zero =>
          self.status = (self.status == 0) as u8,
        _ => ()
      }
    }
  }
}