# Flare Programming Language

Flare is a statically-typed programming language with a focus on simplicity and performance. It compiles to bytecode and runs on a custom virtual machine.

## Features

### Data Types
- **Numbers**: Integer values && float (`numb`)
- **Strings**: Text values (`string`)
- **Booleans**: `true` and `false` values

### Variables
- **Variable Declaration**: `var name:type` or `var name = value`
- **Constant Declaration**: `const name:type = value`
- **Variable Assignment**: `name = value`
- **Type Inference**: Automatic type detection when not explicitly specified

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparison**: `>`, `<`,
- **Logical**: Boolean operations with `true`/`false`

### Control Flow
- **Conditional Statements**: `if` and `else` blocks

### Built-in Macros
- `writeLn!(value1,value2,...)`: Print value with newline
- `write!(value1, value2, ...)`: Print values without newline
- `processExit!(code)`: Exit program with status code
- `readInput!(prompt)`: Read user input with prompt

### C Integration
Flare supports calling C functions directly from Flare code, providing access to system-level operations and performance-critical functions.

#### Available C Functions

**Math Functions:**
- `c_add(a, b)`: Add two integers
- `c_multiply(a, b)`: Multiply two integers
- `c_sqrt(x)`: Calculate square root of a float
- `c_pow(base, exponent)`: Calculate base raised to exponent

**String Functions:**
- `c_string_length(str)`: Get string length
- `c_string_concat(str1, str2)`: Concatenate two strings
- `c_string_upper(str)`: Convert string to uppercase

**System Functions:**
- `c_print_hello()`: Print "Hello from C!"
- `c_print_number(num)`: Print an integer from C
- `c_print_float(num)`: Print a float from C

**File Operations:**
- `c_file_exists(filename)`: Check if file exists (returns boolean)
- `c_write_to_file(filename, content)`: Write content to file (returns boolean)

**Random Functions:**
- `c_seed_random(seed)`: Set random seed
- `c_random_int(min, max)`: Generate random integer in range

## Syntax Examples

### Expressions Syntax

```
5+3 //<- no semicolon becouse 5+3 is an expression
```

#### Variable Declaration and Assignment
```flare
var x:numb = 42;
var name:string = "Flare";
const PI:float = 3.14;
var result = 10 + 5;  // Type inferred as numb
```

#### Conditional Statements
```flare
if(x > 10) {
    writeLn!("Greater than 10")
} else {
    writeLn!("Less than or equal to 10")
}
```

#### User Input/Output
```flare
var userName:string = readInput!("Enter your name: ");
write!("Hello, ", userName)
writeLn!("!")
```

#### Mathematical Operations
```flare
var a = 10;
var b = 5;
var sum = a + b;
var product = a * b;
writeLn!(sum)
```

#### C Function Integration
```flare
// Using C functions for math operations
var result = c_add(10, 5);
var squareRoot = c_sqrt(25.0);
var powered = c_pow(2.0, 8.0);

// String operations with C
var text = "hello world";
var upperText = c_string_upper(text);
var length = c_string_length(text);
var combined = c_string_concat("Hello ", "from C!");

// System functions
c_print_hello();
c_print_number(42);

// File operations
var writeSuccess = c_write_to_file("output.txt", "Hello from Flare!");
var fileExists = c_file_exists("output.txt");

// Random numbers
c_seed_random(123);
var randomNum = c_random_int(1, 100);
```

## Building and Running Code

### Build Commands
- `build <src> <output>` - Compiles source file to `./target/<output>`
- `run <src>` - Runs compiled file from source
- `exec <src> <output>` - Builds and runs in one command

### Example Usage
```bash
# Build a Flare program
flare build program.flare my_program

# Run a compiled program
flare run program.flare

# Build and run in one step
flare exec program.flare my_program
```

## Project Structure

- **Lexer**: Tokenization of source code
- **Parser**: AST (Abstract Syntax Tree) generation
- **Compiler**: Bytecode generation with compile-time type checking
- **Virtual Machine**: Bytecode execution engine
- **Built-in Macros**: Core language functionality
- **C Integration**: Foreign Function Interface (FFI) for calling C functions

## Current Status

Flare is in active development (v0.1.2). Current capabilities include:
- Basic arithmetic and string operations
- Variable management with type checking
- Conditional statements
- Input/output operations
- Compile-time type safety
- C function integration via FFI
- File operations through C functions
- Advanced math operations via C libraries

## Building with C Integration

To use C functions in your Flare programs:

1. Place C source files in the `c_src/` directory
2. Build the project with `cargo build` (C code is automatically compiled)
3. Use C functions in your Flare code as shown in the examples above

### Adding Custom C Functions

1. Add your C function to `c_src/flare_c_functions.c`
2. Declare it in `c_src/flare_c_functions.h`
3. Register it in `src/c_integration.rs`
4. The function will be available in Flare programs

## License

This project is licensed under the MIT License - see the [license.md](license.md) file for details.

## Contributing

Flare is an open-source project. Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.
