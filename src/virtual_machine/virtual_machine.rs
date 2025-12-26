use {
    crate::virtual_machine::{
        value::Value::{self, Number, StringValue},
        variables::variable::Variable,
    },
    std::{collections::HashMap, error::Error, f32, fs, string::String},
};

pub struct VM {
    pub ip: usize,
    pub stack: Vec<Value>,
    pub instr: Vec<u8>,
    pub variables: HashMap<String, Variable>,
}

impl VM {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let bytes = fs::read(path).map_err(|e| format!("Unable to run file {}: {}", path, e))?;
        Ok(Self {
            ip: 0,
            stack: Vec::new(),
            instr: bytes,
            variables: HashMap::from([(
                "a".to_string(),
                Variable {
                    value: Number(10.5),
                },
            )]),
        })
    }
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.ip >= self.instr.len() {
                return Err("Unexpected EOF".into());
            } else {
                match self.instr[self.ip] {
                    1 => {
                        let right = self.pop()?;
                        let left = self.pop()?;
                        let result = match (left, right) {
                            (Number(a), Number(b)) => Number(a + b),
                            (StringValue(a), StringValue(b)) => StringValue(a + &b),
                            _ => {
                                return Err(
                                    "Type error: '+' expects number+number or string+string".into(),
                                );
                            }
                        };

                        self.stack.push(result);
                        self.ip += 1;
                    }
                    2 => {
                        let right = self.pop()?;
                        let left = self.pop()?;

                        match (left, right) {
                            (Number(a), Number(b)) => {
                                self.stack.push(Number(a - b));
                            }
                            _ => return Err("Type error: '-' expects numbers".into()),
                        }

                        self.ip += 1;
                    }
                    3 => {
                        let right = self.pop()?;
                        let left = self.pop()?;

                        match (left, right) {
                            (Number(a), Number(b)) => {
                                self.stack.push(Number(a * b));
                            }
                            _ => return Err("Type error: '*' expects numbers".into()),
                        }
                        self.ip += 1;
                    }
                    4 => {
                        let right = self.pop()?;
                        let left = self.pop()?;
                        match (left, right) {
                            (Number(a), Number(b)) => {
                                if b == 0.0 {
                                    return Err("Cannot divide by zero".into());
                                }
                                self.stack.push(Number(a / b));
                            }
                            _ => return Err("Type error: '/' expects numbers".into()),
                        }
                        self.ip += 1;
                    }
                    5 => {
                        self.ip += 1;
                        let len = u32::from_le_bytes(
                            self.instr[self.ip..self.ip + 4].try_into().unwrap(),
                        ) as usize;
                        self.ip += 4;

                        let s =
                            String::from_utf8(self.instr[self.ip..self.ip + len].to_vec()).unwrap();
                        self.ip += len;

                        self.stack.push(StringValue(s));
                    }
                    6 => {
                        self.ip += 1;
                        let len = u32::from_le_bytes(
                            self.instr[self.ip..self.ip + 4].try_into().unwrap(),
                        ) as usize;
                        self.ip += 4;
                        let name = &String::from_utf8(self.instr[self.ip..self.ip + len].to_vec())
                            .unwrap();
                        self.ip += len;
                        let variable = self.variables.get(name);
                        self.stack.push(variable.unwrap().value.clone())
                    }
                    7 => {
                        self.ip += 1;
                        let len = u32::from_le_bytes(
                            self.instr[self.ip..self.ip + 4].try_into().unwrap(),
                        ) as usize;
                        self.ip += 4;
                        let name =
                            String::from_utf8(self.instr[self.ip..self.ip + len].to_vec()).unwrap();
                        self.ip += len;
                        let var = Variable { value: self.pop()? };
                        println!("Saving variable:{:?}", var);
                        self.variables.insert(name, var);
                    }
                    8 => {
                        self.ip += 1;
                        let byte: u8 = self.instr[self.ip];
                        self.ip += 1;

                        let value: bool = match byte {
                            0 => false,
                            1 => true,
                            _ => return Err("Invalid bool value".into()),
                        };
                        (&mut self.stack).push(Value::Bool(value));
                    }
                    9 => {
                        self.ip += 1;
                        let number: f32 = f32::from_le_bytes(
                            self.instr[self.ip..self.ip + 4].try_into().unwrap(),
                        );
                        self.ip += 4;
                        (&mut self.stack).push(Number(number));
                    }
                    20 => {
                        self.ip += 1;
                        match (&mut *self).pop()? {
                            StringValue(s) => {
                                println!("{}", s);
                            }
                            Number(n) => {
                                println!("{}", n)
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                    21 => {
                        self.ip += 1;
                        match self.pop()? {
                            StringValue(s) => {
                                print!("{}", s)
                            }
                            Number(n) => {
                                print!("{}", n)
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                    //Halt
                    255 => {
                        if !self.stack.is_empty() {
                            println!("{:?}", self.stack[0]);
                        }
                        break;
                    }
                    _ => panic!(),
                }
            }
        }
        Ok(())
    }
    fn pop(&mut self) -> Result<Value, String> {
        (&mut self.stack).pop().ok_or("Stack underflow".into())
    }
}
