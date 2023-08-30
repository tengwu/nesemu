use std::ops::BitAndAssign;

use super::memory::Memory;

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

  fn fetch_insts(&mut self, memory: &Memory) -> Vec<u8> {
    let mut inst: Vec<u8> = Vec::new();

    let opcode = memory.read(self.pc);
    inst.push(opcode);
    self.pc += 1;

    /* TODO: Refactor the process to fetch operand for some insts */
    if opcode == 0xA9 {
      let operand = memory.read(self.pc);
      inst.push(operand);
      self.pc += 1;
    }

    inst
  }

  pub fn get_next_inst(&mut self, memory: &Memory) -> Vec<u8> {
    let inst = self.fetch_insts(memory);

    /* TODO: Refactor the process to recovery PC */
    self.pc -= 1;
    if inst[0] == 0xA9 {
      self.pc -= 1;
    }

    inst
  }

  pub fn execute(&mut self, memory: &mut Memory) {
    let insts = self.fetch_insts(memory);
    self.interpret(&insts);
  }

  /* Stub method for test */
  pub fn interpret(&mut self, insts: &Vec<u8>) {
    let opcode = insts[0];
    match opcode {
      0xA9 => {
        let operand = insts[1];
        self.a = operand;

        self.set_status_register(self.a, &vec![StatusReg::Negative, StatusReg::Zero]);
      }
      0xFF => {
        self.pc -= 1; /* Halt on this instruction */
      }
      _ => todo!("Unimplement opcode {}", opcode),
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