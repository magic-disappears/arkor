use env_logger::Env;

use crate::bytecode::bytecode::{Bytecode, Opcode};
use crate::interpreter::vm;
use crate::interpreter::vm::Interpreter;

mod bytecode;
mod interpreter;
mod interpreter_test;

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let instr: Vec<Bytecode> = vec![
        Bytecode::new(Opcode::Push, vec![1]),
        Bytecode::new(Opcode::Store, vec![0]),
        Bytecode::new(Opcode::Load, vec![0]),
        Bytecode::new(Opcode::Ret, vec![])];

    // match vm::StackFrame::new(vec![], vec![0; 4], 0, 4) {
    //     Err(err) => println!("Stack frame is not correct: {}.", err),
    //     Ok(frame) => {
    //         let mut interpreter: Interpreter = Interpreter::new(frame);
    //         interpreter.run();
    //     }
    // }

    let frame = vm::StackFrame::new(vec![], vec![0; 4], 0);
    let mut interpreter: Interpreter = Interpreter::new(frame, instr);
    interpreter.run();
}
