use super::*;

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ConstantIndex(pub u16);

impl<R: Read> ReadType<'_, R> for ConstantIndex {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        reader.read_u16("constant index").map(Self)
    }
}

impl ConstantIndex {
    // TODO impl this as Index on &'a [T] where T: Constant
    pub fn lookup(self, pool: &[Constant]) -> Result<&Constant> {
        if self.0 == 0 {
            return Err(Error::ZeroIndex);
        } else if pool.len() < self.0 as usize {
            return Err(Error::OutOfRange { index: self.0 });
        }
        let constant = &pool[(&self.0 - 1) as usize];
        match *constant {
            Constant::Padding => Err(Error::IndexInsideDoubleWidthConstant { index: self.0 }),
            _ => Ok(constant),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    Utf8(String),

    ClassRef(ConstantIndex),
    StringRef(ConstantIndex),

    MethodRef(MethodRef),
    FieldRef(FieldRef),
    InterfaceMethodRef(InterfaceMethodRef),
    NameAndTypeRef(NameAndTypeRef),
    InvokeDynamicRef(InvokeDynamicRef),

    MethodHandleRef(MethodHandle),
    MethodType(ConstantIndex),

    // for padding
    Padding,
}

impl<R: Read> ReadType<'_, R> for Constant {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self> {
        macro_rules! read_map {
            ($left:ident => $right:ident) => {
                $left::read(reader, context).map(Constant::$right)
            };
            ($left:ident, $name:expr => $right:ident) => {
                reader.$left(stringify!($name)).map(Constant::$right)
            };
        }

        match reader.read_u8("tag")? {
            1 => read_utf8(reader).map(Constant::Utf8),
            3 => read_map!(read_u32, "integer" => Integer),
            4 => read_map!(read_f32, "float" => Float),
            5 => read_map!(read_u64, "long" => Long),
            6 => read_map!(read_f64, "double" => Double),
            7 => read_map!(ConstantIndex => ClassRef),
            8 => read_map!(ConstantIndex => StringRef),
            9 => read_map!(FieldRef => FieldRef),
            10 => read_map!(MethodRef => MethodRef),
            11 => read_map!(InterfaceMethodRef => InterfaceMethodRef),
            12 => read_map!(NameAndTypeRef => NameAndTypeRef),
            15 => read_map!(MethodHandle => MethodHandleRef),
            16 => read_map!(ConstantIndex => MethodType),
            18 => read_map!(InvokeDynamicRef => InvokeDynamicRef),
            e => Err(Error::UnknownTag { tag: e }),
        }
    }
}

impl Constant {
    #[inline]
    pub fn get_tag(self) -> Option<u8> {
        match self {
            Constant::Utf8(..) => Some(1),
            Constant::Integer(..) => Some(3),
            Constant::Float(..) => Some(4),
            Constant::Long(..) => Some(5),
            Constant::Double(..) => Some(6),
            Constant::ClassRef(..) => Some(7),
            Constant::StringRef(..) => Some(8),
            Constant::FieldRef(..) => Some(9),
            Constant::MethodRef(..) => Some(10),
            Constant::InterfaceMethodRef(..) => Some(11),
            Constant::NameAndTypeRef(..) => Some(12),
            Constant::MethodHandleRef(..) => Some(15),
            Constant::MethodType(..) => Some(16),
            Constant::InvokeDynamicRef(..) => Some(18),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodRef {
    pub class: ConstantIndex,
    pub name_and_type: ConstantIndex,
}

impl<R: Read> ReadType<'_, R> for MethodRef {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            class: ConstantIndex::read(reader, context)?,
            name_and_type: ConstantIndex::read(reader, context)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FieldRef {
    pub class: ConstantIndex,
    pub name_and_type: ConstantIndex,
}

impl<R: Read> ReadType<'_, R> for FieldRef {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            class: ConstantIndex::read(reader, context)?,
            name_and_type: ConstantIndex::read(reader, context)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceMethodRef {
    pub class: ConstantIndex,
    pub name_and_type: ConstantIndex,
}

impl<R: Read> ReadType<'_, R> for InterfaceMethodRef {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            class: ConstantIndex::read(reader, context)?,
            name_and_type: ConstantIndex::read(reader, context)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NameAndTypeRef {
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
}

impl<R: Read> ReadType<'_, R> for NameAndTypeRef {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            name: ConstantIndex::read(reader, context)?,
            descriptor: ConstantIndex::read(reader, context)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvokeDynamicRef {
    pub bootstrap: MethodIndex,
    pub name_and_type: ConstantIndex,
}

impl<R: Read> ReadType<'_, R> for InvokeDynamicRef {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            bootstrap: MethodIndex::read(reader, context)?,
            name_and_type: ConstantIndex::read(reader, context)?,
        })
    }
}

#[inline]
fn read_utf8<R: Read>(reader: &mut Reader<'_, R>) -> Result<String> {
    let len = reader.read_u16("utf-8 length")? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf, "utf-8 string")?;
    std::str::from_utf8(&buf)
        .map(ToString::to_string)
        .map_err(|err| Error::InvalidString { error: err })
}
