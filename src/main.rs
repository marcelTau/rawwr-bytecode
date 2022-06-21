#![allow(dead_code)]

mod chunk;
use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);

    chunk.disassemble("test chunk");

    chunk.free();
}
