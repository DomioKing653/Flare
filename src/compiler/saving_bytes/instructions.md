# Flare Bytecode Instructions Reference

## Instruction Format

All instructions start with a 1-byte opcode, followed by arguments (if any).

## Opcode Categories

### Math Operations (1-4)
| Opcode | Instruction | Args | Total Bytes | Description |
|--------|-------------|------|-------------|-------------|
| `0x01` | Add | - | 1 | Pop b, pop a, push a+b |
| `0x02` | Sub | - | 1 | Pop b, pop a, push a-b |
| `0x03` | Mul | - | 1 | Pop b, pop a, push a*b |
| `0x04` | Div | - | 1 | Pop b, pop a, push a/b |

### Values (5-9)
| Opcode | Instruction | Args | Total Bytes | Description |
|--------|-------------|------|-------------|-------------|
| `0x05` | PushString | u32 len + bytes | 5 + len | Push string onto stack |
| `0x06` | LoadVar | u32 len + bytes | 5 + len | Load variable value |
| `0x07` | SaveVar | u32 len + bytes | 5 + len | Save value to variable |
| `0x08` | PushBool | u8 | 2 | Push boolean (0=false, 1=true) |
| `0x09` | PushNumber | f32 | 5 | Push f32 number (little-endian) |

### I/O Operations (20-21)
| Opcode | Instruction | Args | Total Bytes | Description |
|--------|-------------|------|-------------|-------------|
| `0x14` | WriteLnLastOnStack | - | 1 | Pop and print with newline |
| `0x15` | WriteLastOnStack | - | 1 | Pop and print without newline |

### Control Flow (30-41)
| Opcode | Instruction | Args | Total Bytes | Description |
|--------|-------------|------|-------------|-------------|
| `0x1E` | If | u8 | 2 | Conditional (deprecated/unused) |
| `0x23` | ProcessExit | - | 1 | Exit process |
| `0x28` | Jump | u16 | 3 | Unconditional jump to address |
| `0x29` | JumpIfFalse | u16 | 3 | Jump if top of stack is false |

### Special (255)
| Opcode | Instruction | Args | Total Bytes | Description |
|--------|-------------|------|-------------|-------------|
| `0xFF` | Halt | - | 1 | Stop execution |

## Argument Encoding

### String/Variable Names
```
[opcode: u8][length: u32 LE][utf8 bytes: length]
```
Example: `PushString("hi")`
```
05 02 00 00 00 68 69
│  └─────┬────┘ └─┬─┘
│     length=2   "hi"
opcode
```

### Boolean
```
[opcode: u8][value: u8]
```
- `0x00` = false
- `0x01` = true

### Number (f32)
```
[opcode: u8][value: f32 LE]
```
Example: `PushNumber(3.14)`
```
09 C3 F5 48 40
│  └────┬─────┘
│    3.14 as f32
opcode
```

### Jump Address (u16)
```
[opcode: u8][address: u16 LE]
```
Example: `Jump(300)`
```
28 2C 01
│  └─┬──┘
│   300 (0x012C)
opcode
```

## Example Program

### Source Code
```flare
if(true){
    writeLn!("hello")
}
```

### Bytecode (Hex)
```
08 01          PushBool(true)
29 0E 00       JumpIfFalse(14)
05 05 00 00 00 68 65 6C 6C 6F    PushString("hello")
14             WriteLnLastOnStack
28 0E 00       Jump(14)
FF             Halt
```

### Disassembly
```
0x00: PushBool(true)
0x02: JumpIfFalse(0x0E)
0x05: PushString("hello")
0x0E: WriteLnLastOnStack
0x0F: Jump(0x0E)
0x12: Halt
```
