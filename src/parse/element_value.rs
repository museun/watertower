use super::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ElementValue {
    Byte(ConstantIndex),
    Char(ConstantIndex),
    Double(ConstantIndex),
    Float(ConstantIndex),
    Integer(ConstantIndex),
    Long(ConstantIndex),
    Short(ConstantIndex),
    Boolean(ConstantIndex),
    String(ConstantIndex),
    Enum {
        ty: ConstantIndex,
        val: ConstantIndex,
    },
    Class(ConstantIndex),
    Anotation(Annotation),
    Array(Vec<Self>),
}
