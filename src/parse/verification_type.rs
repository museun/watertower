use super::*;

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object(ConstantIndex),
    Uninitialized(u16),
}

impl VerificationType {
    pub(super) fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        use VerificationType::*;
        match reader.read_u8("verification_type")? {
            0 => Ok(Top),
            1 => Ok(Integer),
            2 => Ok(Float),
            3 => Ok(Long),
            4 => Ok(Double),
            5 => Ok(Null),
            6 => Ok(UninitializedThis),
            7 => ConstantIndex::read(reader).map(Object),
            8 => reader.read_u16("uninitialized").map(Uninitialized),
            e => Err(Error::InvalidVerificationType(e)),
        }
    }
}
