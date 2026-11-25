pub trait Cpu {
    fn step(&mut self);
    fn program_counter(&self) -> usize;
}

impl Cpu for r8::Cpu {
    fn step(&mut self) {
        self.step();
    }

    fn program_counter(&self) -> usize {
        self.pc
    }
}
