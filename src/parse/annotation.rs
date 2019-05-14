use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Annotation {
    type_index: ConstantIndex,
    indices_with_values: Vec<(ConstantIndex, ElementValue)>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParameterAnnotation(Vec<Annotation>);
