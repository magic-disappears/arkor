#[cfg(test)]
mod tests {
    use crate::bytecode::bytecode::{Bytecode, Opcode};
    use crate::interpreter::vm::{Interpreter, StackFrame};
    use std::collections::BTreeMap;

    /// Test simultaneously push and ret instructions.
    /// The interpreter pushes '5' to the stack and then return it as address from stack frame.
    #[test]
    fn test_push_and_ret() {
        let mut instr: BTreeMap<i32, Bytecode> = BTreeMap::new();
        instr.insert(0, Bytecode::new(Opcode::Push, vec![5]));
        instr.insert(1, Bytecode::new(Opcode::Ret, vec![]));

        let frame: StackFrame = StackFrame::new(vec![], vec![1], instr);
        let mut interpreter: Interpreter = Interpreter::new(frame);

        interpreter.run();

        assert_eq!(interpreter.pc, 5);
    }

    /// The interpreter pushes a lot of '10's and '5's to the stack,
    /// pops 3 values from the stack and then return '10' as address from stack frame.
    #[test]
    fn test_pop() {
        let mut instr: BTreeMap<i32, Bytecode> = BTreeMap::new();
        instr.insert(0, Bytecode::new(Opcode::Push, vec![10]));
        instr.insert(1, Bytecode::new(Opcode::Push, vec![5]));
        instr.insert(2, Bytecode::new(Opcode::Push, vec![5]));
        instr.insert(3, Bytecode::new(Opcode::Push, vec![5]));
        instr.insert(4, Bytecode::new(Opcode::Pop, vec![]));
        instr.insert(5, Bytecode::new(Opcode::Pop2, vec![]));
        instr.insert(6, Bytecode::new(Opcode::Ret, vec![]));

        let frame: StackFrame = StackFrame::new(vec![], vec![1; 1], instr);
        let mut interpreter: Interpreter = Interpreter::new(frame);

        interpreter.run();

        assert_eq!(interpreter.pc, 10);
    }

    /// The interpreter loads '5' from local to the stack and
    /// then return '5' as address from stack frame.
    #[test]
    fn test_load() {
        let mut instr: BTreeMap<i32, Bytecode> = BTreeMap::new();
        instr.insert(0, Bytecode::new(Opcode::Load, vec![1]));
        instr.insert(1, Bytecode::new(Opcode::Ret, vec![]));

        let frame: StackFrame = StackFrame::new(vec![], vec![5; 2], instr);
        let mut interpreter: Interpreter = Interpreter::new(frame);

        interpreter.run();

        assert_eq!(interpreter.pc, 5);
    }

    /// The interpreter stores '5' to local from the stack and
    /// then return '5' as address from stack frame.
    #[test]
    fn test_store() {
        let mut instr: BTreeMap<i32, Bytecode> = BTreeMap::new();
        instr.insert(0, Bytecode::new(Opcode::Push, vec![5; 1]));
        instr.insert(1, Bytecode::new(Opcode::Store, vec![1]));
        instr.insert(2, Bytecode::new(Opcode::Load, vec![1]));
        instr.insert(3, Bytecode::new(Opcode::Ret, vec![]));

        let frame: StackFrame = StackFrame::new(vec![], vec![0; 2], instr);
        let mut interpreter: Interpreter = Interpreter::new(frame);

        interpreter.run();

        assert_eq!(interpreter.pc, 5);
    }
}
