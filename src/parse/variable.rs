use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub index: u16,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantIndex,
    pub signature: ConstantIndex,
    pub index: u16,
}
