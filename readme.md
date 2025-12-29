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

## Current Status

Flare is in active development (v0.1.2). Current capabilities include:
- Basic arithmetic and string operations
- Variable management with type checking
- Conditional statements
- Input/output operations
- Compile-time type safety

## License

This project is licensed under the MIT License - see the [license.md](license.md) file for details.

## Contributing

Flare is an open-source project. Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.
