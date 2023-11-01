pub mod bytecode {

    #[warn(dead_code, unused_variables)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Opcode {
        // stack
        Load,  // 1 operand, loads FROM locals TO stack
        Store, // 1 operand, stores TO locals FROM stack
        Push,  // 1 operand, pushes constant value to stack
        Pop,   // 0 operands, pops 1 value from stack
        Pop2,  // 0 operands, pops 2 values from stack
        // control flow
        Jump,
        JumpCond,
        Call, // 1 operand, enters new stack frame
        Ret,  // 0 operand, leaves the current stack frame, write return addr to local.
        // math
        Incr,
        Decr,
        Add,
        Sub,
        Mul,
        Div,
        // comparisons
        Eq,
        Neq,
        Gt,
        Gte,
        Lt,
        Lte,
    }

    #[derive(Debug)]
    pub struct Bytecode {
        pub opcode: Opcode,
        pub operands: Vec<i32>,
    }

    impl Bytecode {
        pub(crate) fn new(opcode: Opcode, operands: Vec<i32>) -> Bytecode {
            Bytecode { opcode, operands }
        }
    }
}
