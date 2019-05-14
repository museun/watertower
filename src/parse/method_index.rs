use super::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct MethodIndex(pub u16);

impl<R: Read> ReadType<R> for MethodIndex {
    type Output = Self;
    fn read(reader: &mut Reader<'_, R>) -> Result<Self::Output> {
        reader.read_u16("method index").map(Self)
    }
}
