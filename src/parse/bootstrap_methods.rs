use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct BootstrapMethods {
    method: ConstantIndex,
    arguments: Vec<ConstantIndex>,
}
