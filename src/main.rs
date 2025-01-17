mod opcode;

use opcode::{Opcode,Code};

struct Interpreter {
    stack: Vec<u8>
}

impl Interpreter {
    fn new() -> Self {
        Self {
            stack: vec![0; 1],
        }
    }
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args:Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;
    let code = Code::from(data)?;
    println!("{:?}",code.instrs);

    Ok(())
}
