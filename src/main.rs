use std::collections::BTreeMap;
use crate::bytecode::bytecode::{Bytecode, Opcode};
use crate::interpreter::vm;
use crate::interpreter::vm::Interpreter;

mod bytecode;
mod interpreter;

fn main() {
    let mut instr: BTreeMap<i32, Bytecode> = BTreeMap::new();
    instr.insert(0, Bytecode::new(Opcode::Push, vec![1]));
    instr.insert(1, Bytecode::new(Opcode::Store, vec![0]));
    instr.insert(2, Bytecode::new(Opcode::Load, vec![0]));
    instr.insert(3, Bytecode::new(Opcode::Ret, vec![]));

    let frame: vm::StackFrame = vm::StackFrame::new(vec![], vec![0; 4], instr);
    let mut interpreter: Interpreter = Interpreter::new(frame);

    interpreter.run();
}