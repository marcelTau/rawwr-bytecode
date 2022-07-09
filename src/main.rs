#![allow(dead_code)]

use std::env;
use std::io;

use std::fs::File;
use std::io::Read;

mod vm;
mod value;
mod chunk;
mod scanner;
mod token;
mod constants;

////use chunk::*;
use vm::*;

fn repl(vm: &mut VM) {
    loop {
        print!("> ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        vm.interpret(&input);
    }
}

fn run_file(vm: &mut VM, filename: &String) {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    match vm.interpret(&contents) {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        _ => {}
    }
}

fn main() {
    let mut vm = VM::new();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => std::process::exit(1)
    }
      
    vm.free();
}
