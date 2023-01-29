use crate::interpreter::Register;
use halo2_proofs::halo2curves::bn256::Fq;

#[derive(Default)]
pub struct Matrix {
    pub processor_matrix: Vec<Register>,
    pub instruction_matrix: Vec<InstructionMatrixRow>,
}

pub struct InstructionMatrixRow {
    pub instruction_pointer: Fq,
    pub current_instruction: Fq,
    pub next_instruction: Fq,
}

impl From<&Register> for InstructionMatrixRow {
    fn from(r: &Register) -> Self {
        Self {
            instruction_pointer: r.instruction_pointer,
            current_instruction: r.current_instruction,
            next_instruction: r.next_instruction,
        }
    }
}
