#![allow(dead_code)]

mod value;
mod chunk;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);

    chunk.write_opcode(OpCode::Constant);
    chunk.write(constant);
    chunk.write_opcode(OpCode::Return);

    chunk.disassemble("test");
    chunk.free();
}
