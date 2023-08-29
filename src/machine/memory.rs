pub struct MemoryBus {
  pub memory: Vec<u8>
}

impl MemoryBus {
  pub fn new() -> Self {
    MemoryBus {
      memory: vec![0; 0xFFFF]
    }
  }

  pub fn reset(&mut self) {
    self.memory = vec![0; 0xFFFF];
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }
}