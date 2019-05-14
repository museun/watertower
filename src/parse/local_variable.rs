use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariable {
    start_pc: u16,
    length: u16,
    name: ConstantIndex,
    descriptor: ConstantIndex,
    index: u16,
}
