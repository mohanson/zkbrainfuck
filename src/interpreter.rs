use crate::opcode;
use std::io::{Read, Write};

pub struct Interpreter {
    stack: Vec<u8>,
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    pub fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = opcode::Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0;
        let mut ps = 0;
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                opcode::Opcode::SHL => ps = if ps == 0 { 0 } else { ps - 1 },
                opcode::Opcode::SHR => {
                    ps += 1;
                    if ps == self.stack.len() {
                        self.stack.push(0)
                    }
                }
                opcode::Opcode::ADD => {
                    self.stack[ps] = self.stack[ps].overflowing_add(1).0;
                }
                opcode::Opcode::SUB => {
                    self.stack[ps] = self.stack[ps].overflowing_sub(1).0;
                }
                opcode::Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[ps]])?;
                }
                opcode::Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[ps] = buf[0];
                }
                opcode::Opcode::LB => {
                    if self.stack[ps] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                opcode::Opcode::RB => {
                    if self.stack[ps] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}
