use super::*;

pub trait Lookup {
    fn lookup<T>(&self, index: ConstantIndex) -> Result<T>
    where
        T: Extract + Clone;
}

pub trait Extract: Sized {
    fn extract(constant: &Constant) -> Option<Self>;
    fn field() -> &'static str;
}

macro_rules! extract_impl {
    ($($kind:ident => $arg:ident);* $(;)?) => {
        $(
            impl Extract for $arg {
                fn extract(constant: &Constant) -> Option<$arg> {
                    match constant {
                        Constant::$kind(d) => Some(d.clone()),
                        _ => None,
                    }
                }
                fn field() -> &'static str {
                    stringify!($kind)
                }
            }
        )*
    };
}

extract_impl! {
    Integer => u32 ;
    Float => f32 ;
    Long => u64 ;
    Double => f64 ;
    Utf8 => String ;
}

impl<E> Lookup for E
where
    E: AsRef<[Constant]>,
{
    fn lookup<T>(&self, index: ConstantIndex) -> Result<T>
    where
        T: Extract + Clone,
    {
        let constant = index.lookup(self.as_ref())?;
        T::extract(&constant).ok_or_else(|| Error::MissingField { field: T::field() })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

impl Constant {
    pub fn dump<W: std::io::Write>(
        &self,
        depth: usize,
        w: &mut W,
        constants: &[Constant],
    ) -> std::io::Result<()> {
        let pad = " ".repeat(depth);
        write!(w, "{}", pad)?;

        macro_rules! recur {
            ($index:expr) => {{
                $index
                    .lookup(constants)
                    .unwrap()
                    .dump(depth + 4, w, constants)
            }};
        }

        match self {
            Constant::Integer(d) => write!(w, "{} (Integer)", d),
            Constant::Float(d) => write!(w, "{} (Float)", d),
            Constant::Long(d) => write!(w, "{} (Long)", d),
            Constant::Double(d) => write!(w, "{} (Double)", d),
            Constant::Utf8(d) => write!(w, "'{}' (String)", d),

            Constant::ClassRef(d) => {
                writeln!(w, "ClassRef ->")?;
                recur!(d)
            }
            Constant::StringRef(d) => {
                writeln!(w, "StringRef ->")?;
                recur!(d)
            }
            Constant::MethodType(d) => {
                writeln!(w, "MethodType ->")?;
                recur!(d)
            }

            Constant::MethodRef(MethodRef {
                class,
                name_and_type,
            }) => {
                writeln!(w, "MethodRef ->")?;
                recur!(class)?;
                recur!(name_and_type)
            }

            Constant::FieldRef(FieldRef {
                class,
                name_and_type,
            }) => {
                writeln!(w, "FieldRef ->")?;
                recur!(class)?;
                recur!(name_and_type)
            }

            Constant::InterfaceMethodRef(InterfaceMethodRef {
                class,
                name_and_type,
            }) => {
                writeln!(w, "InterfaceMethodRef ->")?;
                recur!(class)?;
                recur!(name_and_type)
            }

            Constant::NameAndTypeRef(NameAndTypeRef { name, descriptor }) => {
                writeln!(w, "NameAndTypeRef ->")?;
                recur!(name)?;
                recur!(descriptor)
            }

            Constant::InvokeDynamicRef(InvokeDynamicRef {
                bootstrap,
                name_and_type: _name_and_type,
            }) => {
                writeln!(w, "InvokeDynamicRef ->")?;
                // lookup boostrap method
                panic!("bootstrap: {:?}", bootstrap);
                // recur!(name_and_type)
            }

            Constant::MethodHandleRef(handle) => match handle {
                MethodHandle::GetField(d) => {
                    writeln!(w, "GetField ->")?;
                    recur!(d)
                }
                MethodHandle::GetStatic(d) => {
                    writeln!(w, "GetStatic ->")?;
                    recur!(d)
                }
                MethodHandle::PutField(d) => {
                    writeln!(w, "PutField ->")?;
                    recur!(d)
                }
                MethodHandle::PutStatic(d) => {
                    writeln!(w, "PutStatic ->")?;
                    recur!(d)
                }
                MethodHandle::InvokeVirtual(d) => {
                    writeln!(w, "InvokeVirtual ->")?;
                    recur!(d)
                }
                MethodHandle::InvokeDynamic(d) => {
                    writeln!(w, "InvokeDynamic ->")?;
                    recur!(d)
                }
                MethodHandle::InvokeSpecial(d) => {
                    writeln!(w, "InvokeSpecial ->")?;
                    recur!(d)
                }
                MethodHandle::NewInvokeSpecial(d) => {
                    writeln!(w, "NewInvokeSpecial ->")?;
                    recur!(d)
                }
                MethodHandle::InvokeInterface(d) => {
                    writeln!(w, "InvokeInterface ->")?;
                    recur!(d)
                }
            },

            Constant::Padding => return Ok(()),
        }?;
        writeln!(w)
    }
}
