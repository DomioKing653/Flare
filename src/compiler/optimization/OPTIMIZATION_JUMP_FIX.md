# Jump Address Fix in Optimization

## The Problem

When optimizing bytecode instructions, some instructions may be removed or combined (e.g., constant folding). This causes a critical issue: **jump addresses become invalid** because they still reference the old instruction positions.

### Example of the Problem

**Before Optimization:**
```
Index | Instruction
------|------------------
0     | PushNumber(2.0)
1     | PushNumber(3.0)
2     | Add
3     | JumpIfFalse(7)     ← Points to index 7
4     | PushNumber(10.0)
5     | Jump(7)            ← Points to index 7
6     | PushNumber(20.0)
7     | Halt               ← Target of jumps
```

**After Constant Folding (BROKEN):**
```
Index | Instruction
------|------------------
0     | PushNumber(5.0)    ← Folded 2.0 + 3.0
1     | JumpIfFalse(7)     ← WRONG! Still points to 7, but 7 doesn't exist
2     | PushNumber(10.0)
3     | Jump(7)            ← WRONG! Still points to 7
4     | PushNumber(20.0)
5     | Halt               ← Should be target, but is now at index 5
```

The jumps still reference index 7, but the code is only 6 instructions long now!

## The Solution

The solution involves **tracking index changes** during optimization and then **fixing all jump addresses** in a second pass.

### Step 1: Track Index Mappings During Optimization

As we optimize, we maintain a `HashMap<usize, usize>` that maps old indices to new indices:

```rust
old_to_new.insert(0, 0);  // Index 0 stays at 0
old_to_new.insert(1, 0);  // Index 1 merged into 0
old_to_new.insert(2, 0);  // Index 2 merged into 0
old_to_new.insert(3, 1);  // Index 3 moves to 1
old_to_new.insert(4, 2);  // Index 4 moves to 2
old_to_new.insert(5, 3);  // Index 5 moves to 3
old_to_new.insert(6, 4);  // Index 6 moves to 4
old_to_new.insert(7, 5);  // Index 7 moves to 5
```

### Step 2: Fix Jump Addresses

After optimization, we iterate through all instructions and update jump targets:

```rust
match inst {
    Instructions::Jump(old_addr) => {
        let new_addr = old_to_new.get(&old_addr).copied().unwrap_or(old_addr);
        Instructions::Jump(new_addr)
    }
    Instructions::JumpIfFalse(old_addr) => {
        let new_addr = old_to_new.get(&old_addr).copied().unwrap_or(old_addr);
        Instructions::JumpIfFalse(new_addr)
    }
    other => other,
}
```

**After Constant Folding (FIXED):**
```
Index | Instruction
------|------------------
0     | PushNumber(5.0)    ← Folded 2.0 + 3.0
1     | JumpIfFalse(5)     ← FIXED! Now points to 5
2     | PushNumber(10.0)
3     | Jump(5)            ← FIXED! Now points to 5
4     | PushNumber(20.0)
5     | Halt               ← Correct target at index 5
```

## Edge Cases Handled

### Case 1: Jump to End of Code
When jumps point to `code.len()` (one past the last instruction), we handle this specially:

```rust
let last_old_idx = code.len();
let last_new_idx = out.len();
old_to_new.insert(last_old_idx, last_new_idx);
```

### Case 2: Jump to Removed Instruction
If a jump points to an instruction that was completely removed, we find the next valid instruction:

```rust
fn find_closest_new_address(old_to_new: &HashMap<usize, usize>, old_addr: usize) -> usize {
    let mut search_addr = old_addr;
    loop {
        if let Some(&new_addr) = old_to_new.get(&search_addr) {
            return new_addr;
        }
        search_addr += 1;
        // Safety check to prevent infinite loops
        if search_addr > old_addr + 100 {
            return old_addr;
        }
    }
}
```

## Implementation

The solution is split across two files for better separation of concerns:

### `src/compiler/optimization/constant_folding.rs`
Contains the actual constant folding logic:
- **`constant_folding()`** - Performs constant folding and returns `(optimized_code, index_mapping)`
- Tracks how instruction positions change during optimization
- Returns a `HashMap<usize, usize>` mapping old indices to new indices

### `src/compiler/optimization/optimze.rs`
Orchestrates optimization and fixes jump addresses:
1. **`optimize()`** - Main entry point that calls optimization passes and fixes jumps
2. **`fix_jump_addresses()`** - Updates all jump instructions using the index mapping
3. **`find_closest_new_address()`** - Handles edge cases for removed instructions

## How to Add New Optimizations

When adding new optimization passes, follow this pattern:

1. **Create the optimization in its own file** (e.g., `dead_code_elimination.rs`)
2. **Track mappings**: For every old instruction index, record its new index
3. **Handle merges**: If multiple instructions merge into one, map all old indices to the same new index
4. **Return both**: Return `(optimized_code, index_mapping)`
5. **Call from `optimize()`**: Add your optimization pass and apply `fix_jump_addresses()`

### Template for New Optimization (in separate file):

```rust
use crate::compiler::instructions::Instructions;
use std::collections::HashMap;

pub fn my_optimization(code: Vec<Instructions>) -> (Vec<Instructions>, HashMap<usize, usize>) {
    let mut out = Vec::new();
    let mut old_to_new: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    // Optimize and track indices
    while i < code.len() {
        // Your optimization logic here
        old_to_new.insert(i, out.len());
        out.push(code[i].clone());
        i += 1;
    }

    (out, old_to_new)
}
```

### Then in `optimze.rs`:

```rust
pub fn optimize(code: Vec<Instructions>) -> Vec<Instructions> {
    // Apply constant folding
    let (code, old_to_new) = constant_folding(code);
    let code = fix_jump_addresses(code, old_to_new);

    // Apply your new optimization
    let (code, old_to_new) = my_optimization(code);
    let code = fix_jump_addresses(code, old_to_new);

    code
}
```

## Testing

To verify the fix works correctly:

1. Create bytecode with jumps
2. Apply optimization
3. Verify jump targets point to correct instructions
4. Ensure no out-of-bounds jumps

Example test case:
```rust
// Test that constant folding preserves jump correctness
let code = vec![
    Instructions::PushNumber(5.0),
    Instructions::PushNumber(3.0),
    Instructions::Add,
    Instructions::JumpIfFalse(5),
    Instructions::Halt,
];

let optimized = optimize(code);

// After optimization: indices 0,1,2 become 0, so jump should go from 5 to 3
assert_eq!(optimized.len(), 3);
match optimized[1] {
    Instructions::JumpIfFalse(addr) => assert_eq!(addr, 2),
    _ => panic!("Expected JumpIfFalse"),
}
```

## Summary

The key insights are:

1. **Separation of concerns**: Keep optimization logic separate from jump fixing
2. **Track index changes**: Every optimization must return an index mapping
3. **Fix jumps after each pass**: Apply `fix_jump_addresses()` after each optimization
4. **Modular design**: Each optimization lives in its own file and follows the same pattern

This ensures your optimizations won't break control flow in if-else statements, loops, or any other jumping constructs.

### Architecture:
```
constant_folding.rs  ──┐
dead_code_elim.rs    ──┤  Optimization passes
my_optimization.rs   ──┘  (return code + mapping)
                        │
                        ▼
                   optimze.rs
                   (orchestrates & fixes jumps)
```