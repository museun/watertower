use super::*;

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
            return Err(Error::OutOfRange(self.0));
        }
        let constant = &pool[(&self.0 - 1) as usize];
        match *constant {
            Constant::Padding => Err(Error::IndexInsideDoubleWidthConstant(self.0)),
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

    MethodRef {
        class: ConstantIndex,
        name_and_type: ConstantIndex,
    },

    FieldRef {
        class: ConstantIndex,
        name_and_type: ConstantIndex,
    },

    InterfaceMethodRef {
        class: ConstantIndex,
        name_and_type: ConstantIndex,
    },

    NameAndTypeRef {
        name: ConstantIndex,
        descriptor: ConstantIndex,
    },

    MethodHandleRef(MethodHandle),
    MethodType(ConstantIndex),

    InvokeDynamicRef {
        bootstrap: MethodIndex,
        name_and_type: ConstantIndex,
    },

    // for padding
    Padding,
}

impl<R: Read> ReadType<'_, R> for Constant {
    type Output = Self;
    type Context = NullContext;
    fn read(reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self> {
        match reader.read_u8("tag")? {
            1 => Self::utf8(reader),
            3 => Self::integer(reader),
            4 => Self::float(reader),
            5 => Self::long(reader),
            6 => Self::double(reader),
            7 => Self::class_ref(reader),
            8 => Self::string_ref(reader),
            9 => Self::field_ref(reader),
            10 => Self::method_ref(reader),
            11 => Self::interface_method_ref(reader),
            12 => Self::name_and_type_ref(reader),
            15 => Self::method_handle_ref(reader),
            16 => Self::method_type(reader),
            18 => Self::invoke_dynamic_ref(reader),
            e => Err(Error::UnknownTag(e)),
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
            Constant::FieldRef { .. } => Some(9),
            Constant::MethodRef { .. } => Some(10),
            Constant::InterfaceMethodRef { .. } => Some(11),
            Constant::NameAndTypeRef { .. } => Some(12),
            Constant::MethodHandleRef(..) => Some(15),
            Constant::MethodType(..) => Some(16),
            Constant::InvokeDynamicRef { .. } => Some(18),
            _ => None,
        }
    }

    #[inline]
    pub(super) fn utf8<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        let len = reader.read_u16("utf-8 length")? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf, "utf-8 string")?;
        std::str::from_utf8(&buf)
            .map(|s| Constant::Utf8(s.to_string()))
            .map_err(Error::InvalidString)
    }

    #[inline]
    pub(super) fn integer<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u32("integer").map(Constant::Integer)
    }

    #[inline]
    pub(super) fn float<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_f32("float").map(Constant::Float)
    }

    #[inline]
    pub(super) fn long<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u64("long").map(Constant::Long)
    }

    #[inline]
    pub(super) fn double<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_f64("double").map(Constant::Double)
    }

    #[inline]
    pub(super) fn class_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        ConstantIndex::read(reader, &NullContext).map(Constant::ClassRef)
    }

    #[inline]
    pub(super) fn string_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        ConstantIndex::read(reader, &NullContext).map(Constant::StringRef)
    }

    #[inline]
    pub(super) fn field_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::FieldRef {
            class: ConstantIndex::read(reader, &NullContext)?,
            name_and_type: ConstantIndex::read(reader, &NullContext)?,
        })
    }

    #[inline]
    pub(super) fn method_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::MethodRef {
            class: ConstantIndex::read(reader, &NullContext)?,
            name_and_type: ConstantIndex::read(reader, &NullContext)?,
        })
    }

    #[inline]
    pub(super) fn interface_method_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::InterfaceMethodRef {
            class: ConstantIndex::read(reader, &NullContext)?,
            name_and_type: ConstantIndex::read(reader, &NullContext)?,
        })
    }

    #[inline]
    pub(super) fn name_and_type_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::NameAndTypeRef {
            name: ConstantIndex::read(reader, &NullContext)?,
            descriptor: ConstantIndex::read(reader, &NullContext)?,
        })
    }

    #[inline]
    pub(super) fn method_handle_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        let kind = reader.read_u8("method handle ref kind")?;
        let index = ConstantIndex::read(reader, &NullContext)?;
        let handle = match kind {
            1 => MethodHandle::GetField(index),
            2 => MethodHandle::GetStatic(index),
            3 => MethodHandle::PutField(index),
            4 => MethodHandle::PutStatic(index),
            5 => MethodHandle::InvokeVirtual(index),
            6 => MethodHandle::InvokeDynamic(index),
            7 => MethodHandle::InvokeSpecial(index),
            8 => MethodHandle::NewInvokeSpecial(index),
            9 => MethodHandle::InvokeInterface(index),
            e => return Err(Error::InvalidMethodHandleKind(e)),
        };

        Ok(Constant::MethodHandleRef(handle))
    }

    #[inline]
    pub(super) fn method_type<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        ConstantIndex::read(reader, &NullContext).map(Constant::MethodType)
    }

    #[inline]
    pub(super) fn invoke_dynamic_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::InvokeDynamicRef {
            bootstrap: MethodIndex::read(reader, &NullContext)?,
            name_and_type: ConstantIndex::read(reader, &NullContext)?,
        })
    }
}

impl Constant {
    pub fn dump(
        &self,
        depth: usize,
        pool: &[Constant],
        w: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        let pad = " ".repeat(depth);
        write!(w, "{}", pad)?;

        match self {
            Constant::Integer(n) => writeln!(w, "{} : Integer", n),
            Constant::Float(n) => writeln!(w, "{} : Float", n),
            Constant::Long(n) => writeln!(w, "{} : Long", n),
            Constant::Double(n) => writeln!(w, "{} : Double", n),
            Constant::Utf8(s) => writeln!(w, "'{}' : String", s),

            Constant::ClassRef(index) => {
                writeln!(w, "ClassRef ->")?;
                index.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::StringRef(index) => {
                writeln!(w, "StringRef ->")?;
                index.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::MethodRef {
                class,
                name_and_type,
            } => {
                writeln!(w, "MethodRef ->")?;
                class.lookup(pool).unwrap().dump(depth + 4, pool, w)?;
                name_and_type.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::FieldRef {
                class,
                name_and_type,
            } => {
                writeln!(w, "FieldRef ->")?;
                class.lookup(pool).unwrap().dump(depth + 4, pool, w)?;
                name_and_type.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::InterfaceMethodRef {
                class,
                name_and_type,
            } => {
                writeln!(w, "InterfaceMethodRef ->")?;
                class.lookup(pool).unwrap().dump(depth + 4, pool, w)?;
                name_and_type.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::NameAndTypeRef { name, descriptor } => {
                writeln!(w, "NameAndTypeRef ->")?;
                name.lookup(pool).unwrap().dump(depth + 4, pool, w)?;
                descriptor.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::MethodType(index) => {
                writeln!(w, "MethodType ->")?;
                index.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::MethodHandleRef(handle) => {
                writeln!(w, "MethodHandleRef ->")?;
                let pad = " ".repeat(depth + 4);
                write!(w, "{}", pad)?;
                let index = match handle {
                    MethodHandle::GetField(index) => {
                        writeln!(w, "GetField ->")?;
                        index
                    }
                    MethodHandle::GetStatic(index) => {
                        writeln!(w, "GetStatic ->")?;
                        index
                    }
                    MethodHandle::PutField(index) => {
                        writeln!(w, "PutField ->")?;
                        index
                    }
                    MethodHandle::PutStatic(index) => {
                        writeln!(w, "PutStatic ->")?;
                        index
                    }
                    MethodHandle::InvokeVirtual(index) => {
                        writeln!(w, "InvokeVirtual ->")?;
                        index
                    }
                    MethodHandle::InvokeDynamic(index) => {
                        writeln!(w, "InvokeDynamic ->")?;
                        index
                    }
                    MethodHandle::InvokeSpecial(index) => {
                        writeln!(w, "InvokeSpecial ->")?;
                        index
                    }
                    MethodHandle::NewInvokeSpecial(index) => {
                        writeln!(w, "NewInvokeSpecial ->")?;
                        index
                    }
                    MethodHandle::InvokeInterface(index) => {
                        writeln!(w, "InvokeInterface ->")?;
                        index
                    }
                };

                index.lookup(pool).unwrap().dump(depth + 8, pool, w)
            }

            Constant::InvokeDynamicRef {
                bootstrap,
                name_and_type,
            } => {
                writeln!(w, "InvokeDynamicRef ->")?;
                pool[bootstrap.0 as usize - 1].dump(depth + 4, pool, w)?;
                name_and_type.lookup(pool).unwrap().dump(depth + 4, pool, w)
            }

            Constant::Padding => Ok(()),
        }
    }
}
