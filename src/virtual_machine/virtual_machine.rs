use std::{
    fs,
    error::Error,
    f32
};
pub struct VM{
    pub ip:usize,
    pub stack:Vec<f32>,
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
    pub fn run(&mut self)->Result<(),String>{
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
                        self.stack.push(number);
                    }
                    1=>{
                        let left = self.pop()?;
                        let right = self.pop()?;
                        self.stack.push(left+right);
                        self.ip+=1;
                    }
                    2=>{
                        let left = self.pop()?;
                        let right = self.pop()?;
                        self.stack.push(left-right);
                        self.ip+=1;
                    }
                    3=>{
                        let left = self.pop()?;
                        let right = self.pop()?;
                        self.stack.push(left*right);
                        self.ip+=1;
                    }
                    4=>{
                        let left = self.pop()?;
                        let right = self.pop()?;
                        if left == 0f32 {
                            return Err("Cannot divide by zero".into());
                        }
                        else {
                            self.stack.push(left/right)
                        }
                        self.ip+=1;
                    }
                    //Halt
                    255=>{
                        println!("{}",self.stack[0]);
                        break
                    },
                    _=>panic!()
                }
            }

        }
        Ok(())
    }
    fn pop(&mut self) -> Result<f32, String> {
        self.stack.pop().ok_or("Stack underflow".into())
    }
}