use super::*;

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
}

impl Attribute {
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

#[derive(PartialEq, Debug, Clone)]
pub struct BootstrapMethods {
    pub method: ConstantIndex,
    pub arguments: Vec<ConstantIndex>,
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
