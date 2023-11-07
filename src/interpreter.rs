pub mod vm {
    use crate::bytecode::bytecode::{Bytecode, Opcode};
    use log::debug;
    use std::collections::{BTreeMap, VecDeque};

    // TODO: should contain pc, fp, sp, frames stack
    struct VM {}

    #[derive(Debug)]
    pub(crate) struct StackFrame {
        args: Vec<i32>,
        locals: Vec<i32>,
        // map addr -> opcode
        instructions: BTreeMap<i32, Bytecode>,
    }

    impl Default for StackFrame {
        fn default() -> Self {
            StackFrame {
                args: vec![],
                locals: Vec::with_capacity(2),
                instructions: BTreeMap::default(),
            }
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum StackFrameError {
        #[error("instruction is not supported")] // TODO: pass opcode name
        UnsupportedInstruction,
        #[error("instruction has wrong number of operands: {0}")]
        IncorrectOperandsNumber(usize),
        #[error("empty stack")] // TODO: pass opcode name
        EmptyStack,
        #[error("unreachable local: no {0}th local")]
        UnreachableLocal(usize),
    }

    impl StackFrame {
        pub(crate) fn new(
            args: Vec<i32>,
            locals: Vec<i32>,
            instr: BTreeMap<i32, Bytecode>,
        ) -> Result<StackFrame, StackFrameError> {
            (StackFrame {
                args,
                locals,
                instructions: instr,
            })
            .verify()
        }

        fn verify(self) -> Result<StackFrame, StackFrameError> {
            let mut operands_num: BTreeMap<Opcode, usize> = BTreeMap::new();
            operands_num.insert(Opcode::Load, 1);
            operands_num.insert(Opcode::Store, 1);
            operands_num.insert(Opcode::Push, 1);
            operands_num.insert(Opcode::Pop, 0);
            operands_num.insert(Opcode::Pop2, 0);
            operands_num.insert(Opcode::Ret, 0);

            let mut stack_size = 0;

            for instr in &self.instructions {
                let opcode = &instr.1.opcode;
                let operands = &instr.1.operands;
                if operands_num.contains_key(opcode) && operands_num.get(opcode).unwrap() != &operands.len() {
                    return Err(StackFrameError::IncorrectOperandsNumber(operands.len()));
                }

                match opcode {
                    Opcode::Load => {
                        let operand = operands[0] as usize;
                        if operand >= self.locals.len() {
                            return Err(StackFrameError::UnreachableLocal(operand));
                        }
                        stack_size += 1;
                    }
                    Opcode::Store => {
                        let operand = operands[0] as usize;
                        if operand >= self.locals.len() {
                            return Err(StackFrameError::UnreachableLocal(operand));
                        }
                        stack_size -= 1;
                    }
                    Opcode::Push => stack_size += 1,
                    Opcode::Pop | Opcode::Ret => {
                        if stack_size < 1 {
                            return Err(StackFrameError::EmptyStack);
                        }
                        stack_size -= 1;
                    }
                    Opcode::Pop2 => {
                        if stack_size < 2 {
                            return Err(StackFrameError::EmptyStack);
                        }
                        stack_size -= 2;
                    }
                    _ => return Err(StackFrameError::UnsupportedInstruction),
                }
                debug!("Verify {:?}. Stack size: {}", opcode, stack_size)
            }
            Ok(self)
        }
    }

    // Stack-based interpreter
    #[derive(Debug)]
    pub(crate) struct Interpreter {
        pub(crate) pc: i32,
        // TODO: should be private? package-private for test purposes
        // Note: we're using i32 for simplicity
        stack: VecDeque<i32>,
        frame: StackFrame,
        // frame: VecDeque<&StackFrame>,
    }

    impl Interpreter {
        pub(crate) fn new(frame: StackFrame) -> Interpreter {
            Interpreter {
                pc: 0,
                stack: VecDeque::new(),
                frame,
            }
        }

        pub(crate) fn run(&mut self) {
            // TODO: implement jumps
            let mut pc = self.pc;
            loop {
                let curr = self.frame.instructions.get(&pc).unwrap();
                match curr.opcode {
                    // Push operand[0] to stack
                    Opcode::Push => {
                        self.stack.push_back(curr.operands[0]);
                        debug!("Push. Stack: {:?}", self.stack)
                    }
                    // Pops stack's top element
                    Opcode::Pop => {
                        self.stack.pop_back();
                        debug!("Pop. Stack: {:?}", self.stack)
                    }
                    // Pops stack's 2 top elements
                    Opcode::Pop2 => {
                        self.stack.pop_back();
                        self.stack.pop_back();
                        debug!("Pop2. Stack: {:?}", self.stack)
                    }
                    // Load value from local defined by operand to stack
                    Opcode::Load => {
                        self.stack.push_back(self.frame.locals[curr.operands[0] as usize]);
                        debug!("Load. Stack: {:?}, locals: {:?}", self.stack, self.frame.locals);
                    }
                    // Store value from stack to local defined by operand
                    Opcode::Store => {
                        let value = self.stack.pop_back().unwrap();
                        self.frame.locals[curr.operands[0] as usize] = value;
                        debug!("Store. Stack: {:?}, locals: {:?}", self.stack, self.frame.locals);
                    }
                    // Writes return address to PC, return from function, ejects stack frame.
                    Opcode::Ret => {
                        let return_addr = self.stack.pop_back().unwrap();
                        self.pc = return_addr;
                        debug!("Ret. Stack: {:?}, locals: {:?}", self.stack, self.frame.locals);
                        debug!("Frame execution is over, return address is {:?}", return_addr);
                        return;
                    }
                    _ => {
                        panic!("Not implemented")
                    }
                }
                pc += 1;
            }
        }
    }
}
