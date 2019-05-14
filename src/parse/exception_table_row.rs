use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct ExceptionTableRow {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantIndex,
}

impl<R: Read> ReadType<R> for ExceptionTableRow {
    type Output = Self;
    fn read(reader: &mut Reader<'_, R>) -> Result<Self::Output> {
        Ok(Self {
            start_pc: reader.read_u16("start_pc")?,
            end_pc: reader.read_u16("end_c")?,
            handler_pc: reader.read_u16("handler_pc")?,
            catch_type: ConstantIndex::read(reader)?,
        })
    }
}
