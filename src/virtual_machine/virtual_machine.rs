use std::string::String;
use std::{
    fs,
    error::Error,
    f32
};
use crate::virtual_machine::value::Value;
use crate::virtual_machine::value::Value::{Number, StringValue};

pub struct VM{
    pub ip:usize,
    pub stack:Vec<Value>,
    pub instr:Vec<u8>,
}

impl VM {
    pub fn from_file(path:&str)->Result<Self,Box<dyn Error>>{
        let bytes = fs::read(path)
            .map_err(|e| format!("Unable to run file {}: {}", path, e))?;
        Ok(
            Self{
                ip:0,
                stack:Vec::new(),
                instr:bytes
            }
        )
    }
    pub fn run(&mut self)->Result<(), String>{
        loop {
            if self.ip>=self.instr.len() {
                return Err("Unexpected EOF".into())
            }else {
                match self.instr[self.ip] {
                    0=>{
                        self.ip+=1;
                        let number=f32::from_le_bytes(self.instr[self.ip..self.ip+4]
                            .try_into()
                            .unwrap());
                        self.ip+=4;
                        self.stack.push(Number(number));
                    }
                    1 => {
                        let right = self.pop()?;
                        let left = self.pop()?;

                        let result = match (left, right) {
                            (Number(a), Number(b)) => Number(a + b),
                            (StringValue(a), StringValue(b)) => StringValue(a + &b),
                            _ => return Err("Type error: '+' expects number+number or string+string".into()),
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
                            (Value::Number(a), Value::Number(b)) => {
                                self.stack.push(Value::Number(a * b));
                            }
                            _ => return Err("Type error: '*' expects numbers".into()),
                        }

                        self.ip += 1;
                    }
                    4 => {
                        let right = self.pop()?;
                        let left = self.pop()?;

                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => {
                                if b == 0.0 {
                                    return Err("Cannot divide by zero".into());
                                }
                                self.stack.push(Value::Number(a / b));
                            }
                            _ => return Err("Type error: '/' expects numbers".into()),
                        }

                        self.ip += 1;
                    }
                    5 => {
                        self.ip += 1;
                        let len = u32::from_le_bytes(
                            self.instr[self.ip..self.ip + 4].try_into().unwrap()
                        ) as usize;
                        self.ip += 4;

                        let s = String::from_utf8(
                            self.instr[self.ip..self.ip + len].to_vec()
                        ).unwrap();
                        self.ip += len;

                        self.stack.push(Value::StringValue(s));
                    }
                    //Halt
                    255=>{
                        println!("{:?}",self.stack[0]);
                        break
                    },
                    _=>panic!()
                }
            }

        }
        Ok(())
    }
    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or("Stack underflow".into())
    }
}