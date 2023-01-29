use halo2_proofs::halo2curves::{bn256::Fq, FieldExt};

use crate::code;
use std::io::{Read, Write};

pub struct Interpreter {
    pub code: Vec<Fq>,
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

    pub fn set_code(&mut self, code: Vec<Fq>) {
        self.code = code;
    }

    pub fn run(&mut self) {
        loop {
            if self.ip >= self.code.len() {
                break;
            }
            match self.code[self.ip].get_lower_128() as u8 {
                code::SHL => {
                    self.mp -= 1;
                    self.ip += 1;
                }
                code::SHR => {
                    self.mp += 1;
                    if self.mp == self.memory.len() {
                        self.memory.push(0)
                    }
                    self.ip += 1;
                }
                code::ADD => {
                    self.memory[self.mp] = self.memory[self.mp].wrapping_add(1);
                    self.ip += 1;
                }
                code::SUB => {
                    self.memory[self.mp] = self.memory[self.mp].wrapping_sub(1);
                    self.ip += 1;
                }
                code::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf).unwrap();
                    self.memory[self.mp] = buf[0];
                    self.ip += 1;
                }
                code::PUTCHAR => {
                    std::io::stdout().write_all(&[self.memory[self.mp]]).unwrap();
                    self.ip += 1;
                }
                code::LB => {
                    if self.memory[self.mp] == 0x00 {
                        self.ip = self.code[self.ip + 1].get_lower_128() as usize;
                    } else {
                        self.ip += 2;
                    }
                }
                code::RB => {
                    if self.memory[self.mp] != 0x00 {
                        self.ip = self.code[self.ip + 1].get_lower_128() as usize;
                    } else {
                        self.ip += 2;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
