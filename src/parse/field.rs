use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Field {
    pub flags: FieldFlags,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

impl<'a, R: Read> ReadType<'a, R> for Field {
    type Output = Self;
    type Context = ReadContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self> {
        let flags = reader
            .read_u16("field_flags")
            .map(FieldFlags::from_bits)?
            .expect("valid field_flags");
        let name = ConstantIndex::read(reader, &NullContext)?;
        let descriptor = ConstantIndex::read(reader, &NullContext)?;
        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes length"),
            |reader| Attribute::read(reader, context),
        )?;
        Ok(Self {
            flags,
            name,
            descriptor,
            attributes,
        })
    }
}

bitflags! {
    pub struct FieldFlags: u16 {
        const PUBLIC     = 0x0001;
        const PRIVATE    = 0x0002;
        const PROTECTED  = 0x0004;
        const STATIC     = 0x0008;
        const FINAL      = 0x0010;
        const VOLATILE   = 0x0040;
        const TRANSIENT  = 0x0080;
        const SYNTHETIC  = 0x1000;
        const ENUM       = 0x4000;
    }
}
