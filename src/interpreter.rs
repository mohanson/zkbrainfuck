use crate::code;
use std::io::{Read, Write};

pub struct Interpreter {
    pub code: Vec<u16>,
    pub ip: usize,
    pub mp: usize,
    pub memory: Vec<u8>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            ip: 0,
            mp: 0,
            memory: vec![0],
        }
    }

    pub fn set_code(&mut self, code: Vec<u16>) {
        self.code = code;
    }

    pub fn run(&mut self) {
        const SHL: u16 = code::SHL as u16;
        const SHR: u16 = code::SHR as u16;
        const ADD: u16 = code::ADD as u16;
        const SUB: u16 = code::SUB as u16;
        const GETCHAR: u16 = code::GETCHAR as u16;
        const PUTCHAR: u16 = code::PUTCHAR as u16;
        const LB: u16 = code::LB as u16;
        const RB: u16 = code::RB as u16;
        loop {
            if self.ip >= self.code.len() {
                break;
            }
            match self.code[self.ip] {
                SHL => {
                    self.mp -= 1;
                    self.ip += 1;
                }
                SHR => {
                    self.mp += 1;
                    if self.mp == self.memory.len() {
                        self.memory.push(0)
                    }
                    self.ip += 1;
                }
                ADD => {
                    self.memory[self.mp] = self.memory[self.mp].wrapping_add(1);
                    self.ip += 1;
                }
                SUB => {
                    self.memory[self.mp] = self.memory[self.mp].wrapping_sub(1);
                    self.ip += 1;
                }
                GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf).unwrap();
                    self.memory[self.mp] = buf[0];
                    self.ip += 1;
                }
                PUTCHAR => {
                    std::io::stdout().write_all(&[self.memory[self.mp]]).unwrap();
                    self.ip += 1;
                }
                LB => {
                    if self.memory[self.mp] == 0x00 {
                        self.ip = self.code[self.ip + 1] as usize;
                    } else {
                        self.ip += 2;
                    }
                }
                RB => {
                    if self.memory[self.mp] != 0x00 {
                        self.ip = self.code[self.ip + 1] as usize;
                    } else {
                        self.ip += 2;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
