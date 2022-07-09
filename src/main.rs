#![allow(dead_code)]

mod vm;
mod value;
mod chunk;
mod constants;

use chunk::*;
use vm::*;

fn main() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);

    chunk.write_opcode(OpCode::Constant, 1);
    chunk.write(constant, 1);
    chunk.write_opcode(OpCode::Negate, 1);
    chunk.write_opcode(OpCode::Return, 1);
    chunk.disassemble("test");

    vm.interpret(&chunk);

    chunk.free();
    vm.free();
}
