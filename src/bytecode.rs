pub mod bytecode {
    pub enum BytecodeDef {
        // memory
        Load,
        LoadImmediate,
        Store,
        // control flow
        Jump,
        JumpCond,
        Call,
        Ret,
        // math
        Incr,
        Decr,
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        // comparisons
        Eq,
        Neq,
        Gt,
        Gte,
        Lt,
        Lte,
        // logical
        BitwiseAnd,
        BitwiseOr,
        BitwiseXor,
        ShiftLeft,
        ShiftRight,
    }
}