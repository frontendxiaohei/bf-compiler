#[derive(Debug,PartialEq)]
pub enum Opcode {
    SHR = 0x3E, // >
    SHL = 0x3C, // <
    ADD = 0x2B, // +
    SUB = 0x2D, // -
    PUTCHAR = 0x2E, // .
    GETCHAR = 0x2C, // ,
    LB = 0x5B, // [
    RB = 0x5D, // ]
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            // _ => panic!("Invalid opcode: {}", byte),
            _ => unreachable!(),
        }
    }
}


pub struct Code {
    pub instrs: Vec<Opcode>,
    pub jtable: std::collections::HashMap<usize,usize>
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHAR as u8,
            Opcode::GETCHAR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];
        let instrs:Vec<Opcode> = data.iter().filter(|x| dict.contains(x)).map(|x| Opcode::from(*x)).collect();
        Ok(Code{instrs, jtable:std::collections::HashMap::new()})
    }
}