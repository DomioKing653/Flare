# Flare Testing Suite

Python-based test suite for the Flare programming language compiler and VM.

## Setup

### Prerequisites
- Python 3.8+
- Flare compiler built at `target/debug/flarec` (or `target/debug/flarec.exe` on Windows)

### Build Flare Compiler
```bash
cargo build
```

## Running Tests

### Run All Tests
```bash
python testing/test_if_else.py
python testing/test_macros.py
```

### Run with pytest (optional)
```bash
pip install pytest
pytest testing/
```

## Test Files

### `test_if_else.py`
Tests for if-else control flow statements:
- ✓ Basic if statements (true/false conditions)
- ✓ If-else branches
- ✓ Comparison operators (`>`, `<`)
- ✓ Variables in conditions
- ✓ Constant folding in conditions
- ✓ Multiple statements in branches
- ✓ Variable assignments in branches
- ✓ Multiple if statements
- ✓ Sequential if-else statements

**14 test cases**

### `test_macros.py`
Tests for builtin macros:

#### `writeLn!()` - Print with newline
- String, number, float output
- Variables and expressions
- Multiple calls

#### `write!()` - Print without newline
- String concatenation
- Multiple arguments
- Combining with `writeLn!()`

#### `readInput!()` - Read user input
- Basic input reading
- Using input in variables
- Multiple inputs
- Input in conditionals

#### `processExit!()` - Exit program
- Exit codes (0, nonzero)
- Early termination
- Exit in conditionals

**25 test cases**

## Test Structure

Each test:
1. Writes Flare source code to `testing/temp/test.flare`
2. Compiles: `flarec build test.flare OUTPUT_NAME`
3. Runs: `flarec run target/OUTPUT_NAME`
4. Validates output against expected results

## Example Test

```python
def test_if_true_with_else():
    code = """
if(true){
    writeLn!("then branch")
}
else{
    writeLn!("else branch")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "then branch" in stdout
    assert "else branch" not in stdout
```

## Temporary Files

Tests create temporary files in `testing/temp/`:
- `test.flare` - Source code
- `target/test-*` - Compiled bytecode

These are overwritten on each test run.

## Adding New Tests

1. Create a new function starting with `test_`
2. Write Flare code as a string
3. Call `run_flare_code(code, input_data=None)`
4. Assert expected behavior
5. Add test to the `tests` list in `if __name__ == "__main__"`

Example:
```python
def test_my_new_feature():
    code = """writeLn!("hello")"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "hello" in stdout
```

## Expected Output

```
Running if-else tests...
✓ if true no else
✓ if false no else
✓ if true with else
...
14 passed, 0 failed

Running macro tests...
✓ writeLn string
✓ writeLn number
✓ write string
...
25 passed, 0 failed
```

## Troubleshooting

### Compiler not found
Make sure `COMPILER_PATH` in test files points to your compiled `flarec` binary:
```python
COMPILER_PATH = "target/debug/flarec"  # or "target/debug/flarec.exe" on Windows
```

### Tests fail with "Unexpected EOF"
The VM might be reading invalid bytecode. Rebuild the compiler:
```bash
cargo clean
cargo build
```

### Stack underflow errors
Check that all instructions are implemented in:
- `src/compiler/instructions.rs`
- `src/virtual_machine/virtual_machine.rs`
- `src/virtual_machine/pre_parsing.rs`
- `src/compiler/saving_bytes/save.rs`

## Coverage

Current test coverage:
- ✅ If-else statements
- ✅ Comparison operators
- ✅ I/O macros (write, writeLn, readInput)
- ✅ Process control (processExit)
- ✅ Variable assignments
- ✅ Arithmetic expressions
- ✅ Constant folding optimization
- ✅ Jump address fixing

Future tests needed:
- [ ] Loops (when implemented)
- [ ] Functions (when implemented)
- [ ] Arrays/Collections (when implemented)
- [ ] Error handling
- [ ] Edge cases (division by zero, etc.)