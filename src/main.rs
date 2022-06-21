#![allow(dead_code)]

mod value;
mod chunk;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);

    chunk.write_opcode(OpCode::Constant, 1);
    chunk.write(constant, 1);
    chunk.write_opcode(OpCode::Return, 1);

    chunk.disassemble("test");
    chunk.free();
}
