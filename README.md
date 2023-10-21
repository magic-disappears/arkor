# Arkor

Two smart folks learning JIT compilers.

## Project idea

We want to create a toy JIT compiler for but with adult approach.

## Plans
- [x] Define opcodes
- [ ] Create testing tool for execution
- [ ] Opcode verifier (optional).
- [ ] Create opcodes interpreter.
- [ ] Implement OSR for JIT-compiled functions.
- [ ] Implement profiler for the interpreter.
- [ ] Implement optimizer based on traditional IR : BB + SSA approach:
  - [ ] Basic blocks
  - [ ] SSA-form
  - [ ] Dominator Tree
  - [ ] Pass manager (something like 'new' pass manager in LLVM)
    - [ ] Dominator tree construction
    - [ ] Analysis passes
    - [ ] Transformation passes
  - [ ] Basic optimizations
    - [ ] Constant propagation
    - [ ] Constant folding
    - [ ] Peephole optimizations from Hacker's Delight
    - [ ] Loop invariant code motion
    - [ ] Dead code elimination
- [ ] Implement 'hot' methods compiler:
    - [ ] Register allocator
    - [ ] Assembly emitter
      - [ ] x86_64
      - [ ] ARM (optional)
 