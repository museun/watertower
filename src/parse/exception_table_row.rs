use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct ExceptionTableRow {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantIndex,
}

impl ExceptionTableRow {
    pub(super) fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        let start_pc = reader.read_u16("start_pc")?;
        let end_pc = reader.read_u16("end_c")?;
        let handler_pc = reader.read_u16("handler_pc")?;
        let catch_type = ConstantIndex::read(reader)?;
        Ok(Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}
