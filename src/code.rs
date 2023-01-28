pub const SHL: u8 = 0x3C;
pub const SHR: u8 = 0x3E;
pub const ADD: u8 = 0x2B;
pub const SUB: u8 = 0x2D;
pub const GETCHAR: u8 = 0x2C;
pub const PUTCHAR: u8 = 0x2E;
pub const LB: u8 = 0x5B;
pub const RB: u8 = 0x5D;

pub fn compile(code: Vec<u8>) -> Vec<u16> {
    let filter: Vec<u8> = vec![SHL, SHR, ADD, SUB, GETCHAR, PUTCHAR, LB, RB];
    let mut instrs: Vec<u16> = Vec::new();
    let mut jstack: Vec<u16> = Vec::new();
    for i in code {
        if !filter.contains(&i) {
            continue;
        }
        instrs.push(i as u16);
        if i == LB as u8 {
            instrs.push(0);
            jstack.push(instrs.len() as u16 - 1);
        }
        if i == RB as u8 {
            instrs.push(*jstack.last().unwrap() + 1);
            instrs[*jstack.last().unwrap() as usize] = instrs.len() as u16;
            jstack.pop();
        }
    }
    return instrs;
}
