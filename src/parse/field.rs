use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Field {
    pub flags: FieldFlags,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

impl<R: Read> ReadTypeContext<R> for Field {
    type Output = Self;
    fn read(reader: &mut Reader<'_, R>, constants: &[Constant]) -> Result<Self> {
        let flags = reader
            .read_u16("field_flags")
            .map(FieldFlags::from_bits)?
            .expect("valid field_flags");
        let name = ConstantIndex::read(reader)?;
        let descriptor = ConstantIndex::read(reader)?;
        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes length"),
            |reader| Attribute::read(reader, constants),
        )?;
        Ok(Self {
            flags,
            name,
            descriptor,
            attributes,
        })
    }
}
