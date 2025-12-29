# Flare Virtual Machine Architecture

## Overview

The Flare VM uses a **two-phase execution model**: bytecode is first parsed into structured instructions, then executed using instruction indices rather than byte positions.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    COMPILATION PHASE                         │
├─────────────────────────────────────────────────────────────┤
│  Source Code (.flare)                                        │
│       ↓                                                       │
│  Lexer → Tokens                                              │
│       ↓                                                       │
│  Parser → AST                                                │
│       ↓                                                       │
│  Compiler → Vec<Instructions>                                │
│       ↓                                                       │
│  Optimizer (constant folding, jump fixing)                   │
│       ↓                                                       │
│  Serializer → Binary File (.bytes)                           │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                     EXECUTION PHASE                          │
├─────────────────────────────────────────────────────────────┤
│  Binary File (.bytes)                                        │
│       ↓                                                       │
│  BytecodeLoader (pre_parsing.rs)                             │
│       ↓                                                       │
│  Vec<Instructions> (in memory)                               │
│       ↓                                                       │
│  VM.run() - Execute instructions                             │
└─────────────────────────────────────────────────────────────┘
```

## Why Pre-Parse?

### The Problem with Byte-Based Execution

Originally, the VM executed bytecode directly from raw bytes:

```rust
// BAD: Direct byte execution
loop {
    match bytes[ip] {
        40 => {  // Jump opcode
            ip += 1;
            let addr = read_u16_from_bytes();  // Read address
            ip = addr;  // Jump to BYTE position
        }
    }
}
```

**Issues:**
- Jump addresses are **byte positions** (0, 5, 10, 347...)
- When optimizer removes instructions, ALL byte positions change
- Jump fixing becomes extremely complex
- Can jump into middle of instruction (corruption)
- Hard to debug ("what's at byte 347?")

### The Solution: Instruction-Based Execution

Pre-parse bytes into structured `Instructions`, then execute:

```rust
// GOOD: Instruction-based execution
loop {
    match instructions[ip] {
        Instructions::Jump(addr) => {
            ip = addr;  // Jump to INSTRUCTION index
        }
    }
}
```

**Benefits:**
- Jump addresses are **instruction indices** (0, 1, 2, 3...)
- Optimizer works naturally with instruction indices
- Jump fixing is simple (already implemented in `optimization/optimze.rs`)
- Can't jump into middle of instruction
- Easy to debug ("instruction 5 is PushNumber")

## Components

### 1. BytecodeLoader (`pre_parsing.rs`)

**Responsibility:** Convert binary bytecode → `Vec<Instructions>`

**Process:**
```rust
pub struct BytecodeLoader {
    bytes: Vec<u8>,    // Raw bytecode from file
    pos: usize,        // Current read position
}

impl BytecodeLoader {
    pub fn from_file(path: &str) -> Result<Vec<Instructions>, Error> {
        // 1. Read binary file
        // 2. Parse each instruction
        // 3. Return structured vector
    }
}
```

**Example Parsing:**

Binary bytes:
```
[08 01 29 05 00 09 0A 00 00 00 FF]
```

Parsed instructions:
```rust
vec![
    Instructions::PushBool(true),      // 08 01
    Instructions::JumpIfFalse(5),      // 29 05 00
    Instructions::PushNumber(10.0),    // 09 0A 00 00 00
    Instructions::Halt,                // FF
]
```

**Why it works:**
- Parses once at load time (not during execution)
- Validates bytecode (catches corruption early)
- Returns instruction indices (0, 1, 2, 3) not byte positions

### 2. Virtual Machine (`virtual_machine.rs`)

**Responsibility:** Execute parsed instructions

```rust
pub struct VM {
    pub ip: usize,                        // Instruction pointer (index)
    pub stack: Vec<Value>,                // Value stack
    pub instructions: Vec<Instructions>,  // Parsed instructions
    pub variables: HashMap<String, Variable>,
}
```

**Execution Loop:**
```rust
pub fn run(&mut self) -> Result<(), String> {
    loop {
        match &self.instructions[self.ip] {
            Instructions::Jump(addr) => {
                self.ip = *addr;  // Simple index assignment
            }
            Instructions::JumpIfFalse(addr) => {
                match self.pop()? {
                    Bool(false) => self.ip = *addr,
                    Bool(true) => self.ip += 1,
                    _ => return Err("Type error"),
                }
            }
            // ... other instructions
        }
    }
}
```

**Key Points:**
- `ip` is an instruction index, not a byte position
- Jump addresses refer to instruction indices
- Clean pattern matching on structured data
- No byte manipulation during execution

## Jump Address Management

### Question: Is optimization jump fixing still needed?

**YES! Absolutely required.**

### Why?

Even with instruction-based execution, the optimizer still changes instruction positions:

**Before optimization:**
```
Index | Instruction
------|------------------
0     | PushNumber(5.0)
1     | PushNumber(3.0)
2     | Add
3     | JumpIfFalse(7)    ← Points to instruction 7
4     | PushNumber(1.0)
5     | Jump(7)           ← Points to instruction 7
6     | PushNumber(0.0)
7     | Halt
```

**After constant folding:**
```
Index | Instruction
------|------------------
0     | PushNumber(8.0)   ← Instructions 0,1,2 merged!
1     | JumpIfFalse(7)    ← BROKEN! Points to non-existent instruction
2     | PushNumber(1.0)
3     | Jump(7)           ← BROKEN!
4     | PushNumber(0.0)
5     | Halt              ← Should be target
```

**After jump fixing:**
```
Index | Instruction
------|------------------
0     | PushNumber(8.0)
1     | JumpIfFalse(5)    ← FIXED! Now points to instruction 5
2     | PushNumber(1.0)
3     | Jump(5)           ← FIXED!
4     | PushNumber(0.0)
5     | Halt
```

### The Optimization Process

Located in `compiler/optimization/`:

1. **constant_folding.rs**: Performs optimization, returns `(code, index_mapping)`
2. **optimze.rs**: Fixes all jump addresses using the mapping

```rust
pub fn optimize(code: Vec<Instructions>) -> Vec<Instructions> {
    // Apply constant folding and get index mapping
    let (code, old_to_new) = constant_folding(code);
    
    // Fix all jump addresses
    fix_jump_addresses(code, old_to_new)
}
```

The `old_to_new` HashMap maps:
- Old instruction index → New instruction index
- Example: `{0→0, 1→0, 2→0, 3→1, 4→2, ...}`

**This works perfectly with instruction-based VM!**

## Complete Flow Example

### Source Code
```flare
if(true){
    writeLn!("hello")
}
```

### 1. Compilation
```rust
vec![
    Instructions::PushBool(true),
    Instructions::JumpIfFalse(5),
    Instructions::PushString("hello"),
    Instructions::WriteLnLastOnStack,
    Instructions::Jump(5),
    Instructions::Halt,
]
```

### 2. Serialization (save.rs)
```
[08 01] [29 05 00] [05 05 00 00 00 68 65 6C 6C 6F] [14] [28 05 00] [FF]
  │       │          │                                 │     │         │
  │       │          │                                 │     │         └─ Halt
  │       │          │                                 │     └─ Jump(5)
  │       │          │                                 └─ WriteLn
  │       │          └─ PushString("hello")
  │       └─ JumpIfFalse(5)
  └─ PushBool(true)
```

Saved to: `target/test-3`

### 3. Loading (pre_parsing.rs)
```rust
BytecodeLoader::from_file("target/test-3")
// Reads bytes, parses back to:
vec![
    Instructions::PushBool(true),       // Index 0
    Instructions::JumpIfFalse(5),       // Index 1
    Instructions::PushString("hello"),  // Index 2
    Instructions::WriteLnLastOnStack,   // Index 3
    Instructions::Jump(5),              // Index 4
    Instructions::Halt,                 // Index 5
]
```

### 4. Execution (virtual_machine.rs)
```
IP | Instruction                | Stack       | Action
---|----------------------------|-------------|------------------
0  | PushBool(true)             | []          | Push true, ip=1
1  | JumpIfFalse(5)             | [true]      | Pop true, false? No, ip=2
2  | PushString("hello")        | []          | Push "hello", ip=3
3  | WriteLnLastOnStack         | ["hello"]   | Pop & print, ip=4
4  | Jump(5)                    | []          | ip=5
5  | Halt                       | []          | STOP
```

Output: `hello`

## Performance Considerations

### Parse Time vs Runtime

**One-time cost (loading):**
- Parse bytecode → Instructions: ~1-5ms for typical program
- Validates all opcodes
- Builds structured data

**Runtime benefit:**
- No byte manipulation
- Direct instruction matching
- CPU cache-friendly (structured data)
- Faster than byte parsing per instruction

### Memory Usage

**Byte-based (old):**
```rust
instr: Vec<u8>  // 1 byte per opcode, 2-5 bytes per arg
```

**Instruction-based (new):**
```rust
instructions: Vec<Instructions>  // ~16-32 bytes per instruction (enum + data)
```

**Trade-off:**
- Uses more memory (~3-5x)
- But gained: speed, safety, maintainability, modularity

For typical programs (1000-10000 instructions), this is acceptable.

## Debugging Support

With instruction-based execution, debugging becomes natural:

```rust
// Set breakpoint at instruction 5
if ip == 5 {
    println!("Breakpoint hit!");
    println!("Stack: {:?}", stack);
}

// Trace execution
println!("{}: {:?}", ip, instructions[ip]);

// Disassemble
for (idx, instr) in instructions.iter().enumerate() {
    println!("{:04}: {:?}", idx, instr);
}
```

## Future Extensions

The instruction-based architecture enables:

1. **JIT Compilation**: Compile hot instructions → native code
2. **Profiler**: Track time per instruction index
3. **Debugger**: Step through instructions, inspect state
4. **Optimizer**: More passes (dead code elimination, etc.)
5. **Type Checker**: Verify instruction sequences
6. **Security**: Sandboxing, instruction whitelisting

## Summary

**Pre-parsing solves the fundamental problem:**

❌ **Byte-based**: Jumps break when bytes removed (optimization nightmare)  
✅ **Instruction-based**: Jumps stay valid with instruction indices (optimization works)

**The optimization jump fixing (`optimization/optimze.rs`) is still required and works perfectly with this architecture.**

**Key insight:** Separate concerns
- **Serialization** (save.rs): Deals with bytes
- **Optimization** (optimze.rs): Deals with instruction indices
- **Execution** (virtual_machine.rs): Deals with instruction indices

This modular design makes the system maintainable and extensible.