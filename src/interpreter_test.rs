#[cfg(test)]
mod tests {
    use crate::bytecode::bytecode::{Bytecode, Opcode};
    use crate::interpreter::vm::{Interpreter, StackFrame};

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    /// Test simultaneously push and ret instructions.
    /// The interpreter pushes '5' to the stack and then return it as address from stack frame.
    #[test]
    fn test_push_and_ret() {
        init_logger();

        let instr = vec![
            Bytecode::new(Opcode::Push, vec![5]),
            Bytecode::new(Opcode::Ret, vec![])];

        let frame: StackFrame = StackFrame::new(vec![], vec![1], 0);
        let mut interpreter: Interpreter = Interpreter::new(frame, instr);

        interpreter.run();

        assert_eq!(interpreter.pc, 5);
    }

    /// The interpreter pushes a lot of '10's and '5's to the stack,
    /// pops 3 values from the stack and then return '10' as address from stack frame.
    #[test]
    fn test_pop() {
        init_logger();

        let instr = vec![
            Bytecode::new(Opcode::Push, vec![10]),
            Bytecode::new(Opcode::Push, vec![5]),
            Bytecode::new(Opcode::Push, vec![5]),
            Bytecode::new(Opcode::Push, vec![5]),
            Bytecode::new(Opcode::Pop, vec![]),
            Bytecode::new(Opcode::Pop2, vec![]),
            Bytecode::new(Opcode::Ret, vec![])];

        let frame: StackFrame = StackFrame::new(vec![], vec![1], 0);
        let mut interpreter: Interpreter = Interpreter::new(frame, instr);

        interpreter.run();

        assert_eq!(interpreter.pc, 10);
    }

    /// The interpreter loads '5' from local to the stack and
    /// then return '5' as address from stack frame.
    #[test]
    fn test_load() {
        init_logger();

        let instr = vec![
            Bytecode::new(Opcode::Load, vec![1]),
            Bytecode::new(Opcode::Ret, vec![])];

        let frame: StackFrame = StackFrame::new(vec![], vec![5; 2], 0);
        let mut interpreter: Interpreter = Interpreter::new(frame, instr);

        interpreter.run();

        assert_eq!(interpreter.pc, 5);
    }

    /// The interpreter stores '5' to local from the stack and
    /// then return '5' as address from stack frame.
    #[test]
    fn test_store() {
        init_logger();

        let instr = vec![
            Bytecode::new(Opcode::Push, vec![5; 1]),
            Bytecode::new(Opcode::Store, vec![1]),
            Bytecode::new(Opcode::Load, vec![1]),
            Bytecode::new(Opcode::Ret, vec![])];

        let frame: StackFrame = StackFrame::new(vec![], vec![0; 2], 0);
        let mut interpreter: Interpreter = Interpreter::new(frame, instr);

        interpreter.run();

        assert_eq!(interpreter.pc, 5);
    }
}
