// use nesemu::disassemble;
mod machine;

use machine::Machine;

fn main() {
    /* TODO: Refactor args to use advanced rust crates */
    let args: Vec<String> = std::env::args().collect();
    let mut machine = Machine::new_from_args(&args);
    if let Err(msg) = machine.run() {
        println!("Error: {}", msg);
    }

    // TODO: Add asynchronous reset here
}
