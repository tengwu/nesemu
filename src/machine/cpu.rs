use super::memory::MemoryBus;

pub struct CPU {
  pub a: u8,
  pub x: u8,
  pub y: u8,
  pub status: u8,
  pub pc: u16,
  pub memory_bus: MemoryBus
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      a: 0,
      x: 0,
      y: 0,
      status: 0,
      pc: 0,
      memory_bus: MemoryBus::new()
    }
  }

  pub fn reset(&mut self) {
    self.a = 0;
    self.x = 0;
    self.y = 0;
    self.status = 0;
    self.pc = 0;
    self.memory_bus.reset();
  }

  /* Stub method for test */
  pub fn interpret(&mut self, insts: &Vec<u8>) {
    let opcode = insts[0];
    match opcode {
      0xA9 => {
        let operand = insts[1];
        self.a = operand;

        self.set_status_register(self.a);
      }
      _ => todo!("Unimplement opcode {}", opcode),
    }
  }

  pub fn set_status_register(&mut self, target_reg: u8) {
    todo!("Implement set_status_register");
  }
}