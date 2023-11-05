pub mod vm {
    use crate::bytecode::bytecode::{Bytecode, Opcode};
    use log::debug;
    use std::collections::{BTreeMap, HashMap, VecDeque};

    // TODO: should contain pc, fp, sp, frames stack
    struct VM {}

    #[derive(Debug)]
    pub(crate) struct StackFrame {
        args: Vec<i32>,
        locals: Vec<i32>,
        start_addr: i32,
    }

    impl Default for StackFrame {
        fn default() -> Self {
            StackFrame {
                args: vec![],
                locals: Vec::with_capacity(2),
                start_addr: 0,
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
            start_addr: i32,
            // ) -> Result<StackFrame, StackFrameError> {
        ) -> StackFrame {
            StackFrame {
                args,
                locals,
                start_addr,
            }
            // .verify()
        }

        fn verify(self, instructions: &[Bytecode]) -> Result<StackFrame, StackFrameError> {
            let mut operands_num: BTreeMap<Opcode, usize> = BTreeMap::new();
            operands_num.insert(Opcode::Load, 1);
            operands_num.insert(Opcode::Store, 1);
            operands_num.insert(Opcode::Push, 1);
            operands_num.insert(Opcode::Pop, 0);
            operands_num.insert(Opcode::Pop2, 0);
            operands_num.insert(Opcode::Ret, 0);

            let mut stack_size = 0;

            for instr in instructions {
                let opcode = &instr.opcode;
                let operands = &instr.operands;
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
        pub(crate) pc: usize,
        // TODO: should be private? package-private for test purposes
        // Note: we're using i32 for simplicity
        stack: VecDeque<i32>,
        frame: VecDeque<StackFrame>,
        instructions: Vec<Bytecode>,
        // start address -> subroutine args
        func_registers: HashMap<i32, i32>,
    }

    impl Interpreter {
        pub(crate) fn new(initial_frame: StackFrame, instructions: Vec<Bytecode>) -> Interpreter {
            Interpreter {
                pc: 0,
                stack: VecDeque::default(),
                frame: VecDeque::from(vec![initial_frame]),
                instructions,
                func_registers: HashMap::default(),
            }
        }

        pub(crate) fn run(&mut self) {
            // TODO: implement jumps
            let mut frame: StackFrame = self.frame.pop_back().unwrap();
            loop {
                let curr = self.instructions.get(self.pc).unwrap();
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
                        self.stack.push_back(frame.locals[curr.operands[0] as usize]);
                        debug!("Load. Stack: {:?}, locals: {:?}", self.stack, frame.locals);
                    }
                    // Store value from stack to local defined by operand
                    Opcode::Store => {
                        let value = self.stack.pop_back().unwrap();
                        frame.locals[curr.operands[0] as usize] = value;
                        debug!("Store. Stack: {:?}, locals: {:?}", self.stack, frame.locals);
                    }
                    Opcode::Call => {
                        let return_addr = self.pc + 1;

                        // TODO: find function by address

                        debug!("Call. Function address: {:?}", curr.operands[0]);
                        self.stack.push_back(return_addr as i32);
                        self.frame.push_back(frame);
                        self.pc = curr.operands[0] as usize;
                        return;
                    }
                    // Writes return address to PC, return from function, ejects stack frame.
                    Opcode::Ret => {
                        frame = self.frame.pop_back().unwrap();

                        let return_addr = self.stack.pop_back().unwrap() as usize;
                        self.pc = return_addr;

                        debug!("Ret. Stack: {:?}, locals: {:?}", self.stack, frame.locals);
                        debug!("Frame execution is over, return address is {:?}", return_addr);
                        return;
                    }
                    _ => {
                        panic!("Not implemented")
                    }
                }
                self.pc += 1;
            }
        }
    }
}
