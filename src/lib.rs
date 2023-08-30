use rs6502::Disassembler;

pub fn disassemble(code: &Vec<u8>) -> String {
  let dasm = Disassembler::new();
  let asm = dasm.disassemble(&code);
  asm
}