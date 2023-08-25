/* put each type of instruction here */
pub enum Instruction
{
    InstLiteral(u8),
    InstAddValue(u8),
    InstSetValue(u8),
    InstSubValue(u8),
    InstMulValue(u8),
    InstDivValue(u8),
}

impl Instruction
{
    pub fn to_u8(&self) -> u8
    {
        match self
        {
            Instruction::InstLiteral(x) => *x,
            Instruction::InstAddValue(x) => *x,
            Instruction::InstSetValue(x) => *x,
            Instruction::InstSubValue(x) => *x,
            Instruction::InstMulValue(x) => *x,
            Instruction::InstDivValue(x) => *x,
        }
    }
}
