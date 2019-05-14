use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct InnerClassInfo {
    inner_class: ConstantIndex,
    outer_class: ConstantIndex,
    inner_class_name: ConstantIndex,
    flags: InnerClassFlags,
}
