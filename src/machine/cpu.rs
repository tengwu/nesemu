use nesemu::disassemble;

pub struct CPU {
  pub a: u8,
  pub x: u8,
  pub y: u8,
  pub status: u8,
  pub pc: u16
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      a: 0,
      x: 0,
      y: 0,
      status: 0,
      pc: 0
    }
  }

  pub fn reset(&mut self) {
    self.a = 0;
    self.x = 0;
    self.y = 0;
    self.status = 0;
    self.pc = 0;
  }

  pub fn interpret(&mut self, insts: &Vec<u8>) {
    println!("{}", disassemble(&insts));
  }
}