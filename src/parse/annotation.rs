use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Annotation {
    pub type_index: ConstantIndex,
    pub indices_with_values: Vec<(ConstantIndex, ElementValue)>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParameterAnnotation(pub Vec<Annotation>);
