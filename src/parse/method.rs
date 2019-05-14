use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Method {
    pub flags: MethodFlags,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

impl<'a, R: Read> ReadType<'a, R> for Method {
    type Output = Self;
    type Context = ReadContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self> {
        let flags = reader
            .read_u16("access_flags")
            .map(MethodFlags::from_bits)?
            .expect("valid method flags");
        let name = ConstantIndex::read(reader, &NullContext)?;
        let descriptor = ConstantIndex::read(reader, &NullContext)?;
        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes_count"), //
            |reader| Attribute::read(reader, &context),
        )?;
        Ok(Self {
            flags,
            name,
            descriptor,
            attributes,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct MethodIndex(pub u16);

impl<R: Read> ReadType<'_, R> for MethodIndex {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        reader.read_u16("method index").map(Self)
    }
}

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

bitflags! {
    pub struct MethodFlags: u16 {
        const PUBLIC       = 0x0001;
        const PRIVATE      = 0x0002;
        const PROTECTED    = 0x0004;
        const STATIC       = 0x0008;
        const FINAL        = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE       = 0x0040;
        const VARARGS      = 0x0080;
        const NATIVE       = 0x0100;
        const ABSTRACT     = 0x0400;
        const STRICT       = 0x0800;
        const SYNTHETIC    = 0x1000;
    }
}
