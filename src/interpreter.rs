use crate::code;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::halo2curves::{bn256::Fq, FieldExt};
use std::io::{Read, Write};

#[derive(Clone, Debug, Default)]
pub struct Register {
    pub cycle: Fq,
    pub instruction_pointer: Fq,
    pub current_instruction: Fq,
    pub next_instruction: Fq,
    pub memory_pointer: Fq,
    pub memory_value: Fq,
    pub memory_value_inverse: Fq,
}

impl Register {
    fn ip(&self) -> usize {
        self.instruction_pointer.get_lower_128() as usize
    }

    fn mp(&self) -> usize {
        self.memory_pointer.get_lower_128() as usize
    }
}

pub struct Interpreter {
    pub code: Vec<Fq>,
    pub memory: Vec<Fq>,
    pub register: Register,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            memory: vec![Fq::zero()],
            register: Register::default(),
        }
    }

    pub fn set_code(&mut self, code: Vec<Fq>) {
        self.code = code;
    }

    pub fn run(&mut self) {
        self.register.current_instruction = self.code[0];
        if self.code.len() == 1 {
            self.register.next_instruction = Fq::zero()
        } else {
            self.register.next_instruction = self.code[1];
        }
        loop {
            if self.register.instruction_pointer >= Fq::from(self.code.len() as u64) {
                break;
            }
            match self.register.current_instruction.get_lower_128() as u8 {
                code::SHL => {
                    self.register.memory_pointer -= Fq::one();
                    self.register.instruction_pointer += Fq::one();
                }
                code::SHR => {
                    self.register.memory_pointer += Fq::one();
                    if self.register.mp() == self.memory.len() {
                        self.memory.push(Fq::zero())
                    }
                    self.register.instruction_pointer += Fq::one();
                }
                code::ADD => {
                    self.memory[self.register.mp()] += Fq::one();
                    self.register.instruction_pointer += Fq::one();
                }
                code::SUB => {
                    self.memory[self.register.mp()] -= Fq::one();
                    self.register.instruction_pointer += Fq::one();
                }
                code::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf).unwrap();
                    self.memory[self.register.mp()] = Fq::from(buf[0] as u64);
                    self.register.instruction_pointer += Fq::one();
                }
                code::PUTCHAR => {
                    std::io::stdout().write_all(&[self.register.memory_value.get_lower_128() as u8]).unwrap();
                    self.register.instruction_pointer += Fq::one();
                }
                code::LB => {
                    if self.memory[self.register.mp()] == Fq::zero() {
                        self.register.instruction_pointer = self.code[self.register.ip() + 1];
                    } else {
                        self.register.instruction_pointer += Fq::from(2);
                    }
                }
                code::RB => {
                    if self.memory[self.register.mp()] != Fq::zero() {
                        self.register.instruction_pointer = self.code[self.register.ip() + 1];
                    } else {
                        self.register.instruction_pointer += Fq::from(2);
                    }
                }
                _ => unreachable!(),
            }
            self.register.cycle += Fq::one();
            if self.register.instruction_pointer < Fq::from(self.code.len() as u64) {
                self.register.current_instruction = self.code[self.register.ip()];
            } else {
                self.register.current_instruction = Fq::zero();
            }
            if self.register.instruction_pointer < Fq::from(self.code.len() as u64) - Fq::one() {
                self.register.next_instruction = self.code[self.register.ip() + 1];
            } else {
                self.register.next_instruction = Fq::zero()
            }
            self.register.memory_value = self.memory[self.register.mp()];
            self.register.memory_value_inverse = if self.register.memory_value == Fq::zero() {
                Fq::zero()
            } else {
                self.register.memory_value.invert().unwrap()
            };
        }
    }
}
