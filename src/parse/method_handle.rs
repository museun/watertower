use super::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum MethodHandle {
    GetField(ConstantIndex),
    GetStatic(ConstantIndex),
    PutField(ConstantIndex),
    PutStatic(ConstantIndex),
    InvokeVirtual(ConstantIndex),
    InvokeDynamic(ConstantIndex),
    InvokeSpecial(ConstantIndex),
    NewInvokeSpecial(ConstantIndex),
    InvokeInterface(ConstantIndex),
}
