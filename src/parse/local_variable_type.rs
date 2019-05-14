use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariableType {
    start_pc: u16,
    length: u16,
    name: ConstantIndex,
    signature: ConstantIndex,
    index: u16,
}
