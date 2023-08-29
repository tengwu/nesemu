pub struct Memory {
  pub blocks: Vec<u8>
}

impl Memory {
  pub fn new() -> Self {
    Memory {
      blocks: vec![0; 0xFFFF]
    }
  }

  pub fn reset(&mut self) {
    self.blocks = vec![0; 0xFFFF];
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.blocks[addr as usize]
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.blocks[addr as usize] = data;
  }
}