use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Method {
    pub flags: MethodFlags,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub(super) fn read<R: Read>(
        reader: &mut Reader<'_, R>,
        constants: &[Constant],
    ) -> Result<Self> {
        let flags = reader
            .read_u16("access_flags")
            .map(MethodFlags::from_bits)?
            .expect("valid method flags");
        let name = ConstantIndex::read(reader)?;
        let descriptor = ConstantIndex::read(reader)?;
        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes_count"), //
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
