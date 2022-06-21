use crate::value::*;

pub enum OpCode {
    Constant,
    Return,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::Constant,
            1 => OpCode::Return,
            _ => unimplemented!("Invalid Opcode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        code as u8
    }
}

pub struct Chunk {
    code: Vec<u8>,
    constants: ValueArray,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_opcode(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte.into());
        self.lines.push(line);
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn free(&mut self) {
        self.code = Vec::new();
        self.lines = Vec::new();
        self.constants.free();
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write(value);
        (self.constants.len() - 1) as u8
    }

    pub fn disassemble<T>(&self, name: T)
    where
        T: ToString + std::fmt::Display,
    {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && (self.lines[offset] == self.lines[offset - 1]) {
            print!("   | ");
        } else {
            print!("{:04} ", self.lines[offset]);
        }

        let instruction: OpCode = self.code[offset].into();
        match instruction {
            OpCode::Return => self.simple_instruction("OP_RETURN", offset),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{name:-16} {constant:4} '");
        self.constants.print_value(constant);
        println!("'");
        offset + 2
    }
}
