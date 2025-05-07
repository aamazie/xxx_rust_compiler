#[derive(Debug)]
pub enum Instruction {
    Return(i32),
}

#[derive(Debug)]
pub struct IRProgram {
    pub instructions: Vec<Instruction>,
}
