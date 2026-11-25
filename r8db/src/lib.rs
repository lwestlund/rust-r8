mod cpu;
mod instruction;
mod prompt;

use crate::cpu::Cpu;
pub use crate::instruction::Instruction;
pub use crate::prompt::prompt_for_input;

pub struct R8db<C> {
    cpu: C,
    last_instruction: Option<Instruction>,
}

impl<C> R8db<C>
where
    C: Cpu,
{
    pub fn new(cpu: C) -> Self {
        Self {
            cpu,
            last_instruction: None,
        }
    }

    pub fn format_prompt(&self) -> String {
        format!("{pc:04}> ", pc = self.cpu.program_counter())
    }

    pub fn run_instruction(&mut self, instruction: Option<Instruction>) {
        if let Some(instruction) = instruction {
            self.last_instruction = Some(instruction);
        }
        let Some(instruction) = self.last_instruction.as_ref() else {
            return;
        };
        match instruction {
            Instruction::Step { count } => {
                for _ in 0..count.get() {
                    self.cpu.step();
                }
            }
        }
    }
}
