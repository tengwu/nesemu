mod cpu;
mod instruction;
mod memory;
mod monitor;
use std::{io::Write, process::exit};

use cpu::CPU;
use memory::Memory;
use monitor::{Monitor, MonitorState};

pub struct Machine {
    cpu: CPU,
    memory: Memory,
    reset: bool,
    stop: bool,
    debug: bool,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            cpu: CPU::new(),
            memory: Memory::new(),
            reset: false,
            stop: false,

            debug: true, /* TODO: Don't go to debug mode by defalt */
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
                _ => (),
            }
        }

        machine
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
        self.stub_fill_memory_with_insts();
    }

    fn stub_fill_memory_with_insts(&mut self) {
        /* LDA #$C3 */
        self.memory.write(0x0, 0xA9);
        self.memory.write(0x1, 0xC3);

        /* My stub HALT */
        self.memory.write(0x2, 0xFF);
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.stub_fill_memory_with_insts();
        loop {
            if self.reset {
                self.reset();
            }

            if self.stop {
                break;
            }

            self.monitor();

            self.cpu.execute(&mut self.memory);
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
            let mut not_display_next_inst: bool = false;
            loop {
                cmd.clear();

                // println!("{}", disassemble(&self.cpu.get_next_inst(&self.memory)).trim());
                if !not_display_next_inst {
                    println!("{:?}", self.cpu.get_next_inst(&self.memory));
                    not_display_next_inst = false;
                }

                print!("(NESEmu) ");
                std::io::stdout().flush().expect("Failed to flush stdout");
                std::io::stdin().read_line(&mut cmd).unwrap();

                match cmd.trim() {
                    "r" => {
                        self.reset = true;
                        println!("Resetting...");
                        self.reset();
                        println!("Reset complete");
                        println!();
                    }
                    "q" => self.stop = true,
                    "s" => (),
                    "p" => {
                        self.cpu.print_state();
                        not_display_next_inst = true;
                        continue;
                    }
                    _ => {
                        println!("Unknown command: {}", cmd.trim());
                        println!();
                        continue;
                    }
                }
                break;
            }
        }
    }
}
