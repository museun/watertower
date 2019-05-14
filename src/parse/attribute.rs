use super::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Attribute {
    Code(Code),
    SourceFile(SourceFile),
    InnerClasses(InnerClasses),
    EnclosingMethod(EnclosingMethod),
    SourceDebugExtension(SourceDebugExtension),
    ConstantValue(ConstantValue),
    Exceptions(Exceptions),
    BootstrapMethods(BootstrapMethods),
    AnnotationDefault(AnnotationDefault),
    MethodParameters(MethodParameters),
    Synthetic(Synthetic),
    Deprecated(Deprecated),
    Signature(Signature),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations),
    LineNumberTable(LineNumberTable),
    LocalVariableTable(LocalVariableTable),
    LocalVariableTypeTable(LocalVariableTypeTable),
    StackMapTable(StackMapTable),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotations),
    RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotations),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotations),
}

impl<R: Read> ReadTypeContext<R> for Attribute {
    type Output = Self;
    fn read(reader: &mut Reader<'_, R>, constants: &[Constant]) -> Result<Self::Output> {
        let index = reader.read_u16("attribute_name_index").map(ConstantIndex)?;
        let constant = index.lookup(&constants)?;

        let ty = match constant {
            Constant::Utf8(s) => s,
            _ => return Err(Error::InvalidAttributeType(constant.clone())),
        };

        let start = reader.read_u32("attribute_length")?;
        let pos = reader.pos();

        macro_rules! parse_table {
            ($($name:expr=> $ident:ident);* $(;)?) => {
                 match ty.as_str() {
                    $($name => $ident::read(reader, constants, index).map(Attribute::$ident),)*
                    _ => {
                        if cfg!(test) {
                            panic!("unknown table: {}", ty)
                        } else {
                            Err(Error::UnknownAttributeType(ty.to_string()))
                        }
                    }
                };
            };
        }

        let res = parse_table!(
            "ConstantValue"   => ConstantValue;
            "Code"            => Code;
            "StackMapTable"   => StackMapTable;
            "Exceptions"      => Exceptions;
            "LineNumberTable" => LineNumberTable;
            "SourceFile"      => SourceFile;
        );

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
}

#[derive(PartialEq, Debug, Clone)]
pub struct Code {
    pub attribute_name: ConstantIndex,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableRow>,
    pub attributes: Vec<Attribute>,
}

impl<R: Read> ReadTypeContextIndexed<R> for Code {
    type Output = Self;
    fn read(
        reader: &mut Reader<'_, R>,
        constants: &[Constant],
        index: ConstantIndex,
    ) -> Result<Self::Output> {
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

        Ok(Self {
            attribute_name: index,
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SourceFile {
    pub attribute_name: ConstantIndex,
    pub source_file: ConstantIndex,
}

impl<R: Read> ReadTypeContextIndexed<R> for SourceFile {
    type Output = Self;
    fn read(
        reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        index: ConstantIndex,
    ) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: index,
            source_file: ConstantIndex::read(reader)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct InnerClasses {
    pub attribute_name: ConstantIndex,
    pub classes: Vec<InnerClassInfo>,
}

impl<R: Read> ReadTypeContextIndexed<R> for InnerClasses {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct EnclosingMethod {
    pub attribute_name: ConstantIndex,
    pub class: ConstantIndex,
    pub method: ConstantIndex,
}

impl<R: Read> ReadTypeContextIndexed<R> for EnclosingMethod {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SourceDebugExtension {
    pub attribute_name: ConstantIndex,
    pub debug_extension: Vec<u8>,
}

impl<R: Read> ReadTypeContextIndexed<R> for SourceDebugExtension {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ConstantValue {
    pub attribute_name: ConstantIndex,
    pub constant_value: ConstantIndex,
}

impl<R: Read> ReadTypeContextIndexed<R> for ConstantValue {
    type Output = Self;
    fn read(
        reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        index: ConstantIndex,
    ) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: index,
            constant_value: ConstantIndex::read(reader)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Exceptions {
    pub attribute_name: ConstantIndex,
    pub index_table: Vec<ConstantIndex>,
}

impl<R: Read> ReadTypeContextIndexed<R> for Exceptions {
    type Output = Self;
    fn read(
        reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        index: ConstantIndex,
    ) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: index,
            index_table: reader.read_many(
                |reader| reader.read_u16("exceptions length"),
                ConstantIndex::read,
            )?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct BootstrapMethods {
    pub attribute_name: ConstantIndex,
    pub methods: Vec<BootstrapMethods>,
}

impl<R: Read> ReadTypeContextIndexed<R> for BootstrapMethods {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AnnotationDefault {
    pub attribute_name: ConstantIndex,
    pub value: ElementValue,
}

impl<R: Read> ReadTypeContextIndexed<R> for AnnotationDefault {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MethodParameters {
    pub attribute_name: ConstantIndex,
}

impl<R: Read> ReadTypeContextIndexed<R> for MethodParameters {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Synthetic {
    pub attribute_name: ConstantIndex,
}

impl<R: Read> ReadTypeContextIndexed<R> for Synthetic {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Deprecated {
    pub attribute_name: ConstantIndex,
}

impl<R: Read> ReadTypeContextIndexed<R> for Deprecated {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Signature {
    pub attribute_name: ConstantIndex,
    pub signature: ConstantIndex,
}

impl<R: Read> ReadTypeContextIndexed<R> for Signature {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeVisibleAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<R: Read> ReadTypeContextIndexed<R> for RuntimeVisibleAnnotations {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeInvisibleAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<R: Read> ReadTypeContextIndexed<R> for RuntimeInvisibleAnnotations {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LineNumberTable {
    pub attribute_name: ConstantIndex,
    pub table: Vec<(u16, u16)>,
}

impl<R: Read> ReadTypeContextIndexed<R> for LineNumberTable {
    type Output = Self;
    fn read(
        reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        index: ConstantIndex,
    ) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: index,
            table: reader.read_many(
                |reader| reader.read_u16("line_number_table length"),
                |reader| {
                    let start_pc = reader.read_u16("start_pc")?;
                    let line_no = reader.read_u16("line_number")?;
                    Ok((start_pc, line_no))
                },
            )?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariableTable {
    pub attribute_name: ConstantIndex,
    pub variables: Vec<LocalVariable>,
}

impl<R: Read> ReadTypeContextIndexed<R> for LocalVariableTable {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariableTypeTable {
    pub attribute_name: ConstantIndex,
    pub variables_types: Vec<LocalVariableType>,
}

impl<R: Read> ReadTypeContextIndexed<R> for LocalVariableTypeTable {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct StackMapTable {
    pub attribute_name: ConstantIndex,
    pub entries: Vec<StackMapFrame>,
}

impl<R: Read> ReadTypeContextIndexed<R> for StackMapTable {
    type Output = Self;
    fn read(
        reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        index: ConstantIndex,
    ) -> Result<Self::Output> {
        Ok(StackMapTable {
            attribute_name: index,
            entries: reader.read_many(
                |reader| reader.read_u16("stack_map_table length"),
                |reader| StackMapFrame::read(reader),
            )?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeVisibleTypeAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<R: Read> ReadTypeContextIndexed<R> for RuntimeVisibleTypeAnnotations {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeInvisibleTypeAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<R: Read> ReadTypeContextIndexed<R> for RuntimeInvisibleTypeAnnotations {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeVisibleParameterAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations_by_param_index: Vec<ParameterAnnotation>,
}

impl<R: Read> ReadTypeContextIndexed<R> for RuntimeVisibleParameterAnnotations {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeInvisibleParameterAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations_by_param_index: Vec<ParameterAnnotation>,
}

impl<R: Read> ReadTypeContextIndexed<R> for RuntimeInvisibleParameterAnnotations {
    type Output = Self;
    fn read(
        _reader: &mut Reader<'_, R>,
        _constants: &[Constant],
        _index: ConstantIndex,
    ) -> Result<Self::Output> {
        unimplemented!()
    }
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
pub struct ExceptionTableRow {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantIndex,
}

impl<R: Read> ReadType<R> for ExceptionTableRow {
    type Output = Self;
    fn read(reader: &mut Reader<'_, R>) -> Result<Self::Output> {
        Ok(Self {
            start_pc: reader.read_u16("start_pc")?,
            end_pc: reader.read_u16("end_c")?,
            handler_pc: reader.read_u16("handler_pc")?,
            catch_type: ConstantIndex::read(reader)?,
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

impl<R: Read> ReadType<R> for StackMapFrame {
    type Output = Self;
    fn read(reader: &mut Reader<'_, R>) -> Result<Self::Output> {
        let ty = reader.read_u8("stack_map_frame type")?;
        match ty {
            0...63 => Self::same_frame(reader, ty),
            64...127 => Self::same_locals_one_stack_item_frame(reader, ty),
            247 => Self::same_locals_one_stack_item_frame_extended(reader, ty),
            248...250 => Self::chop_frame(reader, ty),
            251 => Self::same_frame_extended(reader, ty),
            252...254 => Self::append_frame(reader, ty),
            255 => Self::full_frame(reader, ty),
            _ => Err(Error::InvalidStackFrameType(ty)),
        }
    }
}

impl StackMapFrame {
    fn same_frame<R: Read>(_: &mut Reader<'_, R>, ty: u8) -> Result<Self> {
        Ok(StackMapFrame::SameFrame { offset: ty })
    }

    fn same_frame_extended<R: Read>(reader: &mut Reader<'_, R>, _: u8) -> Result<Self> {
        Ok(StackMapFrame::SameFrameExtended {
            offset: reader.read_u16("same_frame_extended")?,
        })
    }

    fn same_locals_one_stack_item_frame<R: Read>(
        reader: &mut Reader<'_, R>,
        ty: u8,
    ) -> Result<Self> {
        Ok(StackMapFrame::SameLocalsOneStackItemFrame {
            offset: ty - 64,
            stack_item: VerificationType::read(reader)?,
        })
    }

    fn same_locals_one_stack_item_frame_extended<R: Read>(
        reader: &mut Reader<'_, R>,
        _: u8,
    ) -> Result<Self> {
        Ok(StackMapFrame::SameLocalsOneStackItemFrameExtended {
            offset: reader.read_u16("same_locals_one_stack_item_frame_extended")?,
            stack_item: VerificationType::read(reader)?,
        })
    }

    fn chop_frame<R: Read>(reader: &mut Reader<'_, R>, ty: u8) -> Result<Self> {
        Ok(StackMapFrame::ChopFrame {
            offset: reader.read_u16("chop_frame")?,
            absent_locals: (251 - ty),
        })
    }

    fn append_frame<R: Read>(reader: &mut Reader<'_, R>, _: u8) -> Result<Self> {
        let offset = reader.read_u16("append_frame")?;
        let new_locals = reader.read_many(
            |reader| reader.read_u16("num_locals"),
            VerificationType::read,
        )?;
        Ok(StackMapFrame::AppendFrame { offset, new_locals })
    }

    fn full_frame<R: Read>(reader: &mut Reader<'_, R>, _: u8) -> Result<Self> {
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

impl<R: Read> ReadType<R> for VerificationType {
    type Output = Self;
    fn read(reader: &mut Reader<'_, R>) -> Result<Self::Output> {
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
