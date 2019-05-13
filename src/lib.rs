#![allow(dead_code, unused_variables)]
// TODO use https://docs.rs/smallvec/0.6.9/smallvec/

use bitflags::bitflags;
use byteorder::{ReadBytesExt, BE};
use std::io::{Read};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(String, std::io::Error),
    Expected(String, String),
    UnknownTag(u8),
    InvalidMethodHandleKind(u8),
    InvalidString(std::str::Utf8Error),
    ZeroIndex,
    OutOfRange(u16),
    IndexInsideDoubleWidthConstant(u16),
    InvalidAttributeType(Constant),
    UnknownAttributeType(String),
    InvalidStackFrameType(u8),
    InvalidVerificationType(u8),
    LengthMismatch {
        length: u32,
        actual: u32,
        ty: String,
    },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(_, err) => Some(err),
            Error::InvalidString(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(msg, err) => write!(f, "expected {}, got a read error: {}", msg, err),
            Error::Expected(left, right) => write!(f, "expected: {}, got {}", right, left),
            Error::UnknownTag(d) => write!(f, "unknown tag: 0x{:02X}", d),
            Error::InvalidMethodHandleKind(d) => {
                write!(f, "invalid method handle ref kind: 0x{:02X}", d)
            }
            Error::InvalidString(err) => write!(f, "invalid utf-8 string: {}", err),
            Error::ZeroIndex => write!(f, "invalid index: zero index"),
            Error::OutOfRange(d) => write!(f, "out of range: {}", d),
            Error::IndexInsideDoubleWidthConstant(d) => write!(
                f,
                "i
                ndex inside of a double widht constant: {}",
                d
            ),
            Error::InvalidAttributeType(d) => write!(f, "invalid attribute type: {:?}", d),
            Error::InvalidStackFrameType(d) => write!(f, "invalid stack frame type: {:#X?}", d),
            Error::InvalidVerificationType(d) => write!(f, "invalid verification type: {:#X?}", d),
            Error::UnknownAttributeType(s) => write!(f, "unknown attribute type: {}", s),
            Error::LengthMismatch { length, actual, ty } => write!(
                f,
                "length mismatch while parsing: `{}` got: {} wanted: {}",
                ty, actual, length
            ),
        }
    }
}

pub struct Reader<'a, R> {
    source: &'a mut R,
    pos: usize,
}

impl<'a, R: Read> Reader<'a, R> {
    fn new(source: &'a mut R, pos: usize) -> Self {
        Self { source, pos }
    }
}

impl<'a, R> From<&'a mut R> for Reader<'a, R>
where
    R: Read,
{
    fn from(read: &'a mut R) -> Self {
        Self::new(read, 0)
    }
}

impl<'a, R: Read> Reader<'a, R> {
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8], msg: impl std::fmt::Display) -> Result<()> {
        self.source
            .read_exact(buf)
            .map(|_| self.pos += buf.len())
            .map_err(|err| Error::Io(msg.to_string(), err))
    }

    #[inline]
    fn read_u64(&mut self, msg: impl std::fmt::Display) -> Result<u64> {
        self.source
            .read_u64::<BE>()
            .map(|n| {
                self.pos += 8;
                n
            })
            .map_err(|err| Error::Io(msg.to_string(), err))
    }

    #[inline]
    fn read_u32(&mut self, msg: impl std::fmt::Display) -> Result<u32> {
        self.source
            .read_u32::<BE>()
            .map(|n| {
                self.pos += 4;
                n
            })
            .map_err(|err| Error::Io(msg.to_string(), err))
    }

    #[inline]
    fn read_u16(&mut self, msg: impl std::fmt::Display) -> Result<u16> {
        self.source
            .read_u16::<BE>()
            .map(|n| {
                self.pos += 2;
                n
            })
            .map_err(|err| Error::Io(msg.to_string(), err))
    }

    #[inline]
    fn read_u8(&mut self, msg: impl std::fmt::Display) -> Result<u8> {
        self.source
            .read_u8()
            .map(|n| {
                self.pos += 1;
                n
            })
            .map_err(|err| Error::Io(msg.to_string(), err))
    }

    #[inline]
    fn read_f32(&mut self, msg: impl std::fmt::Display) -> Result<f32> {
        self.source
            .read_f32::<BE>()
            .map(|n| {
                self.pos += 4;
                n
            })
            .map_err(|err| Error::Io(msg.to_string(), err))
    }

    #[inline]
    fn read_f64(&mut self, msg: impl std::fmt::Display) -> Result<f64> {
        self.source
            .read_f64::<BE>()
            .map(|n| {
                self.pos += 8;
                n
            })
            .map_err(|err| Error::Io(msg.to_string(), err))
    }

    fn read_many<Length, Step, Index, Element>(
        &mut self,
        len: Length,
        step: Step,
    ) -> Result<Vec<Element>>
    where
        Length: Fn(&mut Self) -> Result<Index>,
        Step: Fn(&mut Self) -> Result<Element>,
        Index: Into<usize>,
    {
        let mut vec = Vec::with_capacity(len(self)?.into());
        for i in 0..vec.capacity() {
            vec.push(step(self)?);
        }
        Ok(vec)
    }

    // fn read_struct<T>(&mut self) -> Result<T> {
    //     let size = std::mem::size_of::<T>();
    //     unsafe {
    //         let mut out = std::mem::zeroed();
    //         self.read_exact(std::slice::from_raw_parts_mut(
    //             &mut out as *mut _ as *mut u8,
    //             size,
    //         ))?;
    //         Ok(out)
    //     }
    // }
}

macro_rules! expect {
    ($left:expr, $right:expr) => {
        if $left != $right {
            return Err(Error::Expected(
                format!("{:#X?}", $left),
                format!("{:#X?}", $right),
            ));
        }
    };
}

pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Constant>,
    pub flags: ClassFlags,
    pub this_class: ConstantIndex,
    pub super_class: ConstantIndex,
    pub interfaces: Vec<ConstantIndex>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl std::fmt::Debug for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassFile")
            .field("minor_version", &self.minor_version)
            .field("major_version", &self.major_version)
            .field("constant_pool_len", &self.constant_pool.len())
            .field("flags", &self.flags)
            .field("this_class", &self.this_class)
            .field("super_class", &self.super_class)
            .field("interfaces", &self.interfaces)
            .field("fields", &self.fields)
            .field("methods", &self.methods)
            .field("attributes", &self.attributes)
            .finish()
    }
}

impl ClassFile {
    pub fn read<'a, R, I>(reader: I) -> Result<Self>
    where
        R: Read + 'a,
        I: Into<Reader<'a, R>>,
    {
        let mut reader = reader.into();

        expect!(reader.read_u32("magic")?, 0xCAFE_BABE_u32);
        let minor_version = reader.read_u16("minor_version")?;
        let major_version = reader.read_u16("major_version")?;

        let constant_pool = reader.read_many(
            |reader| reader.read_u16("constant_pool_count").map(|d| d - 1),
            Constant::read,
        )?;

        let flags = reader
            .read_u16("flags")
            .map(ClassFlags::from_bits)?
            .expect("valid flags");

        let this_class = reader.read_u16("this_class").map(ConstantIndex)?;
        let super_class = reader.read_u16("super_class").map(ConstantIndex)?;

        let interfaces = reader.read_many(
            |reader| reader.read_u16("interfaces_count"), //
            ConstantIndex::read,
        )?;

        let fields = reader.read_many(
            |reader| reader.read_u16("fields_count"), //
            |reader| Field::read(reader, constant_pool.as_slice()),
        )?;

        let methods = reader.read_many(
            |reader| reader.read_u16("methods_count"), //
            |reader| Method::read(reader, constant_pool.as_slice()),
        )?;

        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes_count"), //
            |reader| Attribute::read(reader, constant_pool.as_slice()),
        )?;

        Ok(Self {
            minor_version,
            major_version,
            constant_pool,
            flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ConstantIndex(pub u16);

impl ConstantIndex {
    fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u16("constant index").map(Self)
    }

    // TODO impl this as Index on &'a [T] where T: Constant
    fn lookup(self, pool: &[Constant]) -> Result<&Constant> {
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct MethodIndex(pub u16);

impl MethodIndex {
    fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u16("method index").map(Self)
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

impl Constant {
    fn dump(
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

impl Constant {
    fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
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

    #[inline]
    fn get_tag(self) -> Option<u8> {
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
    fn utf8<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        let len = reader.read_u16("utf-8 length")? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf, "utf-8 string")?;
        std::str::from_utf8(&buf)
            .map(|s| Constant::Utf8(s.to_string()))
            .map_err(Error::InvalidString)
    }

    #[inline]
    fn integer<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u32("integer").map(Constant::Integer)
    }

    #[inline]
    fn float<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_f32("float").map(Constant::Float)
    }

    #[inline]
    fn long<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u64("long").map(Constant::Long)
    }

    #[inline]
    fn double<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_f64("double").map(Constant::Double)
    }

    #[inline]
    fn class_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        ConstantIndex::read(reader).map(Constant::ClassRef)
    }

    #[inline]
    fn string_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        ConstantIndex::read(reader).map(Constant::StringRef)
    }

    #[inline]
    fn field_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::FieldRef {
            class: ConstantIndex::read(reader)?,
            name_and_type: ConstantIndex::read(reader)?,
        })
    }

    #[inline]
    fn method_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::MethodRef {
            class: ConstantIndex::read(reader)?,
            name_and_type: ConstantIndex::read(reader)?,
        })
    }

    #[inline]
    fn interface_method_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::InterfaceMethodRef {
            class: ConstantIndex::read(reader)?,
            name_and_type: ConstantIndex::read(reader)?,
        })
    }

    #[inline]
    fn name_and_type_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::NameAndTypeRef {
            name: ConstantIndex::read(reader)?,
            descriptor: ConstantIndex::read(reader)?,
        })
    }

    #[inline]
    fn method_handle_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        let kind = reader.read_u8("method handle ref kind")?;
        let index = ConstantIndex::read(reader)?;
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
    fn method_type<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        ConstantIndex::read(reader).map(Constant::MethodType)
    }

    #[inline]
    fn invoke_dynamic_ref<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        Ok(Constant::InvokeDynamicRef {
            bootstrap: MethodIndex::read(reader)?,
            name_and_type: ConstantIndex::read(reader)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ExceptionTableRow {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantIndex,
}

impl ExceptionTableRow {
    fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
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

#[derive(PartialEq, Debug, Clone)]
pub enum StackMapFrame {
    SameFrame {
        offset: u8,
    },
    SameFrameExtended {
        offset: u16,
    },
    SameLocalsOneStackItemFrame {
        offset: u8,
        stack_item: VerificationType,
    },
    SameLocalsOneStackItemFrameExtended {
        offset: u16,
        stack_item: VerificationType,
    },
    ChopFrame {
        offset: u16,
        absent_locals: u8,
    },
    AppendFrame {
        offset: u16,
        new_locals: Vec<VerificationType>,
    },
    FullFrame {
        offset: u16,
        locals: Vec<VerificationType>,
        stack_items: Vec<VerificationType>,
    },
}

impl StackMapFrame {
    fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        let ty = reader.read_u8("stack_map_frame type")?;
        match ty {
            0...63 => Ok(StackMapFrame::SameFrame { offset: ty }),
            64...127 => Ok(StackMapFrame::SameLocalsOneStackItemFrame {
                offset: ty - 64,
                stack_item: VerificationType::read(reader)?,
            }),
            247 => Ok(StackMapFrame::SameLocalsOneStackItemFrameExtended {
                offset: reader.read_u16("same_locals_one_stack_item_frame_extended")?,
                stack_item: VerificationType::read(reader)?,
            }),
            248...250 => Ok(StackMapFrame::ChopFrame {
                offset: reader.read_u16("chop_frame")?,
                absent_locals: (251 - ty),
            }),
            251 => Ok(StackMapFrame::SameFrameExtended {
                offset: reader.read_u16("same_frame_extended")?,
            }),
            252...254 => {
                let offset = reader.read_u16("append_frame")?;
                let new_locals = reader.read_many(
                    |reader| reader.read_u16("num_locals"),
                    VerificationType::read,
                )?;
                Ok(StackMapFrame::AppendFrame { offset, new_locals })
            }
            255 => {
                let offset = reader.read_u16("full_frame")?;
                let locals = reader.read_many(
                    |reader| reader.read_u16("num_locals"),
                    VerificationType::read,
                )?;
                let stack_items = reader.read_many(
                    |reader| reader.read_u16("num_stack_items"),
                    VerificationType::read,
                )?;
                Ok(StackMapFrame::FullFrame {
                    offset,
                    locals,
                    stack_items,
                })
            }
            _ => Err(Error::InvalidStackFrameType(ty)),
        }
    }
}

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
    fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
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

#[derive(PartialEq, Debug, Clone)]
pub struct InnerClassInfo {
    inner_class: ConstantIndex,
    outer_class: ConstantIndex,
    inner_class_name: ConstantIndex,
    flags: InnerClassFlags,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariable {
    start_pc: u16,
    length: u16,
    name: ConstantIndex,
    descriptor: ConstantIndex,
    index: u16,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariableType {
    start_pc: u16,
    length: u16,
    name: ConstantIndex,
    signature: ConstantIndex,
    index: u16,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Annotation {
    type_index: ConstantIndex,
    indices_with_values: Vec<(ConstantIndex, ElementValue)>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ElementValue {
    Byte(ConstantIndex),
    Char(ConstantIndex),
    Double(ConstantIndex),
    Float(ConstantIndex),
    Integer(ConstantIndex),
    Long(ConstantIndex),
    Short(ConstantIndex),
    Boolean(ConstantIndex),
    String(ConstantIndex),
    Enum {
        ty: ConstantIndex,
        val: ConstantIndex,
    },
    Class(ConstantIndex),
    Anotation(Annotation),
    Array(Vec<Self>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParameterAnnotation(Vec<Annotation>);

#[derive(PartialEq, Debug, Clone)]
pub struct BootstrapMethods {
    method: ConstantIndex,
    arguments: Vec<ConstantIndex>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Attribute {
    Code {
        attribute_name: ConstantIndex,
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>, // TODO instructions
        exception_table: Vec<ExceptionTableRow>,
        attributes: Vec<Attribute>,
    },

    SourceFile {
        attribute_name: ConstantIndex,
        source_file: ConstantIndex,
    },

    InnerClasses {
        attribute_name: ConstantIndex,
        classes: Vec<InnerClassInfo>,
    },

    EnclosingMethod {
        attribute_name: ConstantIndex,
        class: ConstantIndex,
        method: ConstantIndex,
    },

    SourceDebugExtension {
        attribute_name: ConstantIndex,
        debug_extension: Vec<u8>, // TODO extension
    },

    ConstantValue {
        attribute_name: ConstantIndex,
        constant_value: ConstantIndex,
    },

    Exceptions {
        attribute_name: ConstantIndex,
        index_table: Vec<ConstantIndex>,
    },

    BootstrapMethods {
        attribute_name: ConstantIndex,
        methods: Vec<BootstrapMethods>,
    },

    AnnotationDefault {
        attribute_name: ConstantIndex,
        value: ElementValue,
    },

    MethodParameters {
        attribute_name: ConstantIndex,
    },

    Synthetic {
        attribute_name: ConstantIndex,
    },

    Deprecated {
        attribute_name: ConstantIndex,
    },

    Signature {
        attribute_name: ConstantIndex,
        signature: ConstantIndex,
    },

    RuntimeVisibleAnnotations {
        attribute_name: ConstantIndex,
        annotations: Vec<Annotation>,
    },

    RuntimeInvisibleAnnotations {
        attribute_name: ConstantIndex,
        annotations: Vec<Annotation>,
    },

    LineNumberTable {
        attribute_name: ConstantIndex,
        table: Vec<(u16, u16)>, // TODO spans
    },

    LocalVariableTable {
        attribute_name: ConstantIndex,
        variables: Vec<LocalVariable>,
    },

    LocalVariableTypeTable {
        attribute_name: ConstantIndex,
        variables_types: Vec<LocalVariableType>,
    },

    StackMapTable {
        attribute_name: ConstantIndex,
        entries: Vec<StackMapFrame>,
    },

    RuntimeVisibleTypeAnnotations {
        attribute_name: ConstantIndex,
        annotations: Vec<Annotation>,
    },

    RuntimeInvisibleTypeAnnotations {
        attribute_name: ConstantIndex,
        annotations: Vec<Annotation>,
    },

    RuntimeVisibleParameterAnnotations {
        attribute_name: ConstantIndex,
        annotations_by_param_index: Vec<ParameterAnnotation>,
    },

    RuntimeInvisibleParameterAnnotations {
        attribute_name: ConstantIndex,
        annotations_by_param_index: Vec<ParameterAnnotation>,
    },
}

impl Attribute {
    fn read<R: Read>(reader: &mut Reader<'_, R>, constants: &[Constant]) -> Result<Self> {
        let index = reader.read_u16("attribute_name_index").map(ConstantIndex)?;
        let constant = index.lookup(&constants)?;

        let ty = match constant {
            Constant::Utf8(s) => s,
            _ => return Err(Error::InvalidAttributeType(constant.clone())),
        };

        let start = reader.read_u32("attribute_length")?;
        let pos = reader.pos();
        let res = match ty.as_str() {
            "ConstantValue" => Self::constant_value(index, reader),
            "Code" => Self::code(index, reader, constants),
            "StackMapTable" => Self::stack_map_table(index, reader),
            "Exceptions" => Self::exceptions(index, reader),
            "LineNumberTable" => Self::line_number_table(index, reader),
            "SourceFile" => Self::source_file(index, reader),
            _ => Err(Error::UnknownAttributeType(ty.to_string())),
        };
        let end = (reader.pos() - pos) as u32;
        if start == end {
            res
        } else {
            res.and(Err(Error::LengthMismatch {
                length: start,
                actual: end,
                ty: ty.to_string(),
            }))
        }
    }

    fn constant_value<R: Read>(
        attribute_name: ConstantIndex,
        reader: &mut Reader<'_, R>,
    ) -> Result<Self> {
        Ok(Attribute::ConstantValue {
            attribute_name,
            constant_value: ConstantIndex::read(reader)?,
        })
    }

    fn code<R: Read>(
        attribute_name: ConstantIndex,
        reader: &mut Reader<'_, R>,
        constants: &[Constant],
    ) -> Result<Self> {
        let max_stack = reader.read_u16("max_stack")?;
        let max_locals = reader.read_u16("max_locals")?;

        let code = reader.read_many(
            |reader| reader.read_u32("code length").map(|d| d as usize),
            |reader| reader.read_u8("code"),
        )?;

        let exception_table = reader.read_many(
            |reader| reader.read_u16("code length").map(|d| d as usize),
            ExceptionTableRow::read,
        )?;

        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes length"),
            |reader| Attribute::read(reader, constants),
        )?;

        Ok(Attribute::Code {
            attribute_name,
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }

    fn stack_map_table<R: Read>(
        attribute_name: ConstantIndex,
        reader: &mut Reader<'_, R>,
    ) -> Result<Self> {
        let entries = reader.read_many(
            |reader| reader.read_u16("stack_map_table length"),
            |reader| StackMapFrame::read(reader),
        )?;
        Ok(Attribute::StackMapTable {
            attribute_name,
            entries,
        })
    }

    fn exceptions<R: Read>(
        attribute_name: ConstantIndex,
        reader: &mut Reader<'_, R>,
    ) -> Result<Self> {
        let index_table = reader.read_many(
            |reader| reader.read_u16("exceptions length"),
            ConstantIndex::read,
        )?;
        Ok(Attribute::Exceptions {
            attribute_name,
            index_table,
        })
    }

    fn line_number_table<R: Read>(
        attribute_name: ConstantIndex,
        reader: &mut Reader<'_, R>,
    ) -> Result<Self> {
        let table = reader.read_many(
            |reader| reader.read_u16("line_number_table length"),
            |reader| {
                let start_pc = reader.read_u16("start_pc")?;
                let line_no = reader.read_u16("line_number")?;
                Ok((start_pc, line_no))
            },
        )?;
        Ok(Attribute::LineNumberTable {
            attribute_name,
            table,
        })
    }

    fn source_file<R: Read>(
        attribute_name: ConstantIndex,
        reader: &mut Reader<'_, R>,
    ) -> Result<Self> {
        Ok(Attribute::SourceFile {
            attribute_name,
            source_file: ConstantIndex::read(reader)?,
        })
    }
}

bitflags! {
    pub struct ClassFlags: u16 {
        const PUBLIC     = 0x0001;
        const FINAL      = 0x0010;
        const SUPER      = 0x0020;
        const ABSTRACT   = 0x0040;
        const INTERFACE  = 0x0200;
        const SYNTHETIC  = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM       = 0x4000;
    }
}

bitflags! {
    pub struct InnerClassFlags: u16 {
        const PUBLIC     = 0x0001;
        const PRIVATE    = 0x0002;
        const PROTECTED  = 0x0004;
        const STATIC     = 0x0008;
        const FINAL      = 0x0010;
        const ABSTRACT   = 0x0040;
        const INTERFACE  = 0x0200;
        const SYNTHETIC  = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM       = 0x4000;
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Field {
    pub flags: FieldFlags,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

impl Field {
    fn read<R: Read>(reader: &mut Reader<'_, R>, constants: &[Constant]) -> Result<Self> {
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

#[derive(PartialEq, Debug, Clone)]
pub struct Method {
    pub flags: MethodFlags,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

impl Method {
    fn read<R: Read>(reader: &mut Reader<'_, R>, constants: &[Constant]) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::*;

    struct LogThis {
        file: std::fs::File,
        pos: usize,
    }

    impl LogThis {
        pub fn new(name: impl AsRef<std::path::Path>) -> Self {
            Self {
                file: std::fs::File::create(name).unwrap(),
                pos: 0,
            }
        }
    }
    impl std::io::Write for LogThis {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            for ch in buf {
                if self.pos % 16 == 0 {
                    if self.pos > 0 {
                        self.file.write_all(&[b'\n'])?;
                    }
                    self.file.write_fmt(format_args!("{:0>4X} ", self.pos))?;
                }
                self.file.write_fmt(format_args!("{:02x} ", ch))?;
                self.pos += 1;
            }
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.file.flush()
        }
    }

    #[test]
    fn read() {
        let data = std::fs::read("./etc/hello.class").unwrap();
        let logthis = LogThis::new("read_dump.txt");
        let mut reader = tee::Reader::new(data.as_slice(), logthis, true);
        let class_file = dbg!(ClassFile::read(&mut reader)).unwrap();

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        for el in &class_file.constant_pool {
            el.dump(0, &class_file.constant_pool, &mut stdout).unwrap();
        }
    }
}

pub mod interpreter;
