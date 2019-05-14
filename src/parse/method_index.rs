use super::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct MethodIndex(pub u16);

impl MethodIndex {
    pub(super) fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u16("method index").map(Self)
    }
}
