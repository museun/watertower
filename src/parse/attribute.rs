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
