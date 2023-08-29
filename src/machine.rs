mod cpu;
mod monitor;
mod memory;
use std::{io::Write, process::exit};

use cpu::CPU;
use monitor::Monitor;

pub struct Machine {
  cpu: CPU,
  reset: bool,
  stop: bool,
  debug: bool,
  stub_insts: Vec<u8>
}

impl Machine {
  pub fn new() -> Self {
    Machine {
      cpu: CPU::new(),
      reset: false,
      stop: false,

      debug: true,
      stub_insts: vec![0xaa, 0xe8, 0x00]
    }
  }

  fn print_help() {
    println!("Usage: nesemu [options]");
    println!("Options:");
    println!("\t-d\t\tEnable debug mode");
    println!("\t-h\t\tPrint this help message");
    exit(0);
  }

  pub fn new_from_args(args: &Vec<String>) -> Self {
    let mut machine = Machine::new();

    for arg in args {
      match arg.as_str() {
        "-d" => machine.set_debug(true),
        "-h" => Machine::print_help(),
        _ => ()
      }
    }

    machine
  }

  pub fn set_debug(&mut self, debug: bool) {
    self.debug = debug;
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }
  
  fn stub_get_inst(&mut self) -> Vec<u8> {
    let inst = self.stub_insts[self.cpu.pc as usize];
    self.cpu.pc += 1;
    vec![inst]
  } 

  pub fn run(&mut self) -> Result<(), String> {
    loop {
      if self.reset {
        self.reset();
      }

      if self.stop {
        break;
      }

      let inst = self.stub_get_inst();
      self.cpu.interpret(&inst);
      
      self.monitor();
    }
    Ok(())
  }
}

impl Monitor for Machine {

  /*
   * TODO: Refactor this method to use advanced Rust features
   */
  fn monitor(&mut self) {
    let mut cmd = String::new();
    if self.debug {
      loop {
        cmd.clear();
        /* TODO: Print the next instruction */
        print!("(NESEmu) ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin().read_line(&mut cmd).unwrap();
        match cmd.trim() {
          "r" => {
            self.reset = true;
            println!("Resetting...");
            self.reset();
            println!("Reset complete");
          }
          "q" => self.stop = true,
          "s" => (),
          "p" => {
            println!("Print machine status");
            continue;
          }
          _ => {
            println!("Unknown command: {}", cmd.trim());
            continue;
          }
        }
        break;
      }
    }
  }
}