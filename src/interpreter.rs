pub mod vm {
    use std::collections::{BTreeMap, VecDeque};
    use crate::bytecode::bytecode::{Bytecode, Opcode};

    // TODO: should contain fp, sp, frames stack
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

    impl StackFrame {
        pub(crate) fn new(args: Vec<i32>,
                          locals: Vec<i32>,
                          instr: BTreeMap<i32, Bytecode>) -> StackFrame {
            StackFrame {
                args,
                locals,
                instructions: instr,
            }
        }
    }

    // Stack-based interpreter
    #[derive(Debug)]
    pub(crate) struct Interpreter {
        pc: i32,
        // Note: temporarily, we're using i32 for simplicity
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
                        println!("Push. Stack: {:?}", self.stack)
                    }
                    // Pops stack's top element
                    Opcode::Pop => {
                        self.stack.pop_back();
                        println!("Pop. Stack: {:?}", self.stack)
                    }
                    // Pops stack's 2 top elements
                    Opcode::Pop2 => {
                        self.stack.pop_back();
                        self.stack.pop_back();
                        println!("Pop2. Stack: {:?}", self.stack)
                    }
                    // Load value from local defined by operand to stack
                    Opcode::Load => {
                        self.stack.push_back(self.frame.locals[curr.operands[0] as usize]);
                        println!("Load. Stack: {:?}, locals: {:?}", self.stack, self.frame.locals);
                    }
                    // Store value from stack to local defined by operand
                    Opcode::Store => {
                        let value = self.stack.pop_back().unwrap();
                        self.frame.locals[curr.operands[0] as usize] = value;
                        println!("Store. Stack: {:?}, locals: {:?}", self.stack, self.frame.locals);
                    }
                    // Writes return address to PC, return from function, ejects stack frame.
                    Opcode::Ret => {
                        let return_addr = self.stack.pop_back().unwrap();
                        pc = return_addr;
                        println!("Ret. Stack: {:?}, locals: {:?}", self.stack, self.frame.locals);
                        println!("Frame execution is over, return address is {:?}", return_addr);
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