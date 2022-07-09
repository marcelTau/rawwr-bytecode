use crate::chunk::*;
use crate::value;
use crate::constants;

pub enum InterpretResult {
    OK,
    CompileError,
    RuntimeError,
}

pub struct VM {
    ip: usize,
    stack: [value::Value; constants::STACK_MAX],
    stack_top: usize, // always one more than the stack_top position
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: [0.0; constants::STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn free(&mut self) {}

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.ip = 0;
        self.run(chunk)
    }

    fn push(&mut self, value: value::Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> value::Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        self.ip += 1;
        chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self, chunk: &Chunk) -> value::Value {
        chunk.constants.values[self.read_byte(chunk) as usize]
    }

    fn do_binary_op(&mut self, opcode: OpCode) {
        let b = self.pop();
        let a = self.pop();

        match opcode {
            OpCode::Add => self.push(a + b),
            OpCode::Subtract => self.push(a - b),
            OpCode::Multiply => self.push(a * b),
            OpCode::Divide => self.push(a / b),
            _ => panic!("Binary operation is not allowed on type {:?}", opcode)
        }
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            // only do this in debug builds
            if cfg!(debug_assertions) {
                print!("          ");

                for i in 0..self.stack_top {
                    print!("[{}]", self.stack[i]);
                }
                println!();
                chunk.disassemble_instruction(self.ip);
            }

            match OpCode::from(self.read_byte(chunk)) {
                OpCode::Return => { 
                    println!("{}", self.pop());
                    return InterpretResult::OK
                },
                OpCode::Negate => {
                    let value = self.pop();
                    self.push(-value)
                },
                opcode_type @ (OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide) => self.do_binary_op(opcode_type),
                OpCode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.push(constant);
                }
            }
        }
    }
}
