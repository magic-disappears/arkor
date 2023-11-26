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

    /// Stack-based interpreter object. It holds:
    /// - program counter (pc);
    /// - execution stack, only i32 values are supported for simplicity;
    /// - instructions vector. Each instruction is represented by Bytecode object,
    ///   which contains opcode and operands. The whole program is represented by
    ///   single vector of instructions. The subroutines is differentiated by
    ///   [Interpreter@func_registers] hashmap, which contains start address of each subroutine.
    ///   We don't need to know the end address of each subroutine, because it always
    ///   must end with Ret instruction.
    /// - frame stack. Each frame contains locals and args.
    #[derive(Debug)]
    pub(crate) struct Interpreter {
        pub(crate) pc: usize,
        // TODO: should be private? package-private for test purposes
        stack: VecDeque<i32>,
        frame: VecDeque<StackFrame>,
        instructions: Vec<Bytecode>,
        // start address -> subroutine args
        func_registers: HashMap<i32, i32>,
    }

    impl Interpreter {
        pub(crate) fn new(
            initial_frame: StackFrame,
            instructions: Vec<Bytecode>,
            func_registers: HashMap<i32, i32>,
        ) -> Interpreter {
            Interpreter {
                pc: 0,
                stack: VecDeque::default(),
                frame: VecDeque::from(vec![initial_frame]),
                instructions,
                func_registers,
            }
        }

        /// Main function, runs the interpreter.
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
                    // Writes return address to PC, store it on stack, find function
                    Opcode::Call => {
                        let return_addr = self.pc + 1;

                        // TODO: catch None.
                        match self.func_registers.get(&curr.operands[0]) {
                            Some(function_addr_ref) => {
                                let fn_addr = *function_addr_ref;
                                debug!("Call. Function address: {:?}", fn_addr);

                                self.stack.push_back(return_addr as i32);
                                self.frame.push_back(StackFrame::new(vec![], vec![0; 4], fn_addr));
                                self.pc = fn_addr as usize;
                            }
                            None => {
                                panic!("Function address is not found")
                            }
                        }
                        return;
                    }
                    // Writes return address to PC, return from function, ejects stack frame.
                    Opcode::Ret => {
                        self.frame.pop_back();

                        let return_addr = self.stack.pop_back().unwrap() as usize;
                        self.pc = return_addr;

                        debug!("Ret. Stack: {:?}", self.stack);
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
