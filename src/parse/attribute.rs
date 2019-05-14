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

impl<'a, R: Read> ReadType<'a, R> for Attribute {
    type Output = Self;
    type Context = ReadContext<'a>;

    fn read(reader: &mut Reader<'_, R>, context: &'a Self::Context) -> Result<Self::Output> {
        let index = reader.read_u16("attribute_name_index").map(ConstantIndex)?;
        let constant = index.lookup(&context.constants)?;

        let ty = match constant {
            Constant::Utf8(s) => s,
            _ => return Err(Error::InvalidAttributeType(constant.clone())),
        };

        let start = reader.read_u32("attribute_length")?;
        let pos = reader.pos();

        macro_rules! parse_table {
            ($($name:expr=> $ident:ident);* $(;)?) => {{
                let context = ReadIndexContext{
                    constants: &context.constants,
                    index,
                };
                match ty.as_str() {
                    $($name => $ident::read(reader, &context).map(Attribute::$ident),)*
                    _ => Err(Error::UnknownAttributeType(ty.to_string())),
                }
            }};
        }

        // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7
        let res = parse_table!(
            "ConstantValue"                        => ConstantValue;
            "Code"                                 => Code;
            "StackMapTable"                        => StackMapTable;
            "Exceptions"                           => Exceptions;
            "BootstrapMethods"                     => BootstrapMethods;

            "InnerClasses"                         => InnerClasses;
            "EnclosingMethod"                      => EnclosingMethod;
            "Synthetic"                            => Synthetic;
            "Signature"                            => Signature;
            "RuntimeVisibleAnnotations"            => RuntimeVisibleAnnotations;
            "RuntimeInvisibleAnnotations"          => RuntimeInvisibleAnnotations;
            "RuntimeVisibleParameterAnnotations"   => RuntimeVisibleParameterAnnotations;
            "RuntimeInvisibleParameterAnnotations" => RuntimeInvisibleParameterAnnotations;
            "RuntimeVisibleTypeAnnotations"        => RuntimeVisibleTypeAnnotations;
            "RuntimeInvisibleTypeAnnotations"      => RuntimeInvisibleTypeAnnotations;
            "AnnotationDefault"                    => AnnotationDefault;
            "MethodParameters"                     => MethodParameters;

            "SourceFile"                           => SourceFile;
            "SourceDebugExtension"                 => SourceDebugExtension;
            "LineNumberTable"                      => LineNumberTable;
            "LocalVariableTable"                   => LocalVariableTable;
            "LocalVariableTypeTable"               => LocalVariableTypeTable;
            "Deprecated"                           => Deprecated;
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

pub struct ReadIndexContext<'a> {
    constants: &'a [Constant],
    index: ConstantIndex,
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

impl<'a, R: Read> ReadType<'a, R> for Code {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        let max_stack = reader.read_u16("max_stack")?;
        let max_locals = reader.read_u16("max_locals")?;

        let code = reader.read_many(
            |reader| reader.read_u32("code length").map(|d| d as usize),
            |reader| reader.read_u8("code"),
        )?;

        let exception_table = reader.read_many(
            |reader| reader.read_u16("code length").map(|d| d as usize),
            |reader| ExceptionTableRow::read(reader, &context),
        )?;

        let ctx = ReadContext {
            constants: &context.constants,
        };
        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes length"),
            |reader| Attribute::read(reader, &ctx),
        )?;

        Ok(Self {
            attribute_name: context.index,
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

impl<'a, R: Read> ReadType<'a, R> for SourceFile {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: context.index,
            source_file: ConstantIndex::read(reader, &NullContext)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct InnerClasses {
    pub attribute_name: ConstantIndex,
    pub classes: Vec<InnerClassInfo>,
}

impl<'a, R: Read> ReadType<'a, R> for InnerClasses {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct EnclosingMethod {
    pub attribute_name: ConstantIndex,
    pub class: ConstantIndex,
    pub method: ConstantIndex,
}

impl<'a, R: Read> ReadType<'a, R> for EnclosingMethod {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SourceDebugExtension {
    pub attribute_name: ConstantIndex,
    pub debug_extension: Vec<u8>,
}

impl<'a, R: Read> ReadType<'a, R> for SourceDebugExtension {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ConstantValue {
    pub attribute_name: ConstantIndex,
    pub constant_value: ConstantIndex,
}

impl<'a, R: Read> ReadType<'a, R> for ConstantValue {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: context.index,
            constant_value: ConstantIndex::read(reader, &NullContext)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Exceptions {
    pub attribute_name: ConstantIndex,
    pub index_table: Vec<ConstantIndex>,
}

impl<'a, R: Read> ReadType<'a, R> for Exceptions {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: context.index,
            index_table: reader.read_many(
                |reader| reader.read_u16("exceptions length"),
                |reader| ConstantIndex::read(reader, &NullContext),
            )?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct BootstrapMethods {
    pub attribute_name: ConstantIndex,
    pub methods: Vec<BootstrapMethods>,
}

impl<'a, R: Read> ReadType<'a, R> for BootstrapMethods {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AnnotationDefault {
    pub attribute_name: ConstantIndex,
    pub value: ElementValue,
}

impl<'a, R: Read> ReadType<'a, R> for AnnotationDefault {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MethodParameters {
    pub attribute_name: ConstantIndex,
}

impl<'a, R: Read> ReadType<'a, R> for MethodParameters {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Synthetic {
    pub attribute_name: ConstantIndex,
}

impl<'a, R: Read> ReadType<'a, R> for Synthetic {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Deprecated {
    pub attribute_name: ConstantIndex,
}

impl<'a, R: Read> ReadType<'a, R> for Deprecated {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Signature {
    pub attribute_name: ConstantIndex,
    pub signature: ConstantIndex,
}

impl<'a, R: Read> ReadType<'a, R> for Signature {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeVisibleAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<'a, R: Read> ReadType<'a, R> for RuntimeVisibleAnnotations {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeInvisibleAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<'a, R: Read> ReadType<'a, R> for RuntimeInvisibleAnnotations {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LineNumberTable {
    pub attribute_name: ConstantIndex,
    pub table: Vec<(u16, u16)>,
}

impl<'a, R: Read> ReadType<'a, R> for LineNumberTable {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            attribute_name: context.index,
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

impl<'a, R: Read> ReadType<'a, R> for LocalVariableTable {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantIndex,
    pub descriptor: ConstantIndex,
    pub index: u16,
}

impl<'a, R: Read> ReadType<'a, R> for LocalVariable {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariableTypeTable {
    pub attribute_name: ConstantIndex,
    pub variables_types: Vec<LocalVariableType>,
}

impl<'a, R: Read> ReadType<'a, R> for LocalVariableTypeTable {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantIndex,
    pub signature: ConstantIndex,
    pub index: u16,
}

impl<'a, R: Read> ReadType<'a, R> for LocalVariableType {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct StackMapTable {
    pub attribute_name: ConstantIndex,
    pub entries: Vec<StackMapFrame>,
}

impl<'a, R: Read> ReadType<'a, R> for StackMapTable {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(StackMapTable {
            attribute_name: context.index,
            entries: reader.read_many(
                |reader| reader.read_u16("stack_map_table length"),
                |reader| StackMapFrame::read(reader, context),
            )?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeVisibleTypeAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<'a, R: Read> ReadType<'a, R> for RuntimeVisibleTypeAnnotations {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeInvisibleTypeAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations: Vec<Annotation>,
}

impl<'a, R: Read> ReadType<'a, R> for RuntimeInvisibleTypeAnnotations {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Annotation {
    pub type_index: ConstantIndex,
    pub indices_with_values: Vec<(ConstantIndex, ElementValue)>,
}

impl<'a, R: Read> ReadType<'a, R> for Annotation {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeVisibleParameterAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations_by_param_index: Vec<ParameterAnnotation>,
}

impl<'a, R: Read> ReadType<'a, R> for RuntimeVisibleParameterAnnotations {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RuntimeInvisibleParameterAnnotations {
    pub attribute_name: ConstantIndex,
    pub annotations_by_param_index: Vec<ParameterAnnotation>,
}

impl<'a, R: Read> ReadType<'a, R> for RuntimeInvisibleParameterAnnotations {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParameterAnnotation(pub Vec<Annotation>);

impl<'a, R: Read> ReadType<'a, R> for ParameterAnnotation {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(_reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
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

impl<'a, R: Read> ReadType<'a, R> for ExceptionTableRow {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            start_pc: reader.read_u16("start_pc")?,
            end_pc: reader.read_u16("end_c")?,
            handler_pc: reader.read_u16("handler_pc")?,
            catch_type: ConstantIndex::read(reader, &NullContext)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SameFrame {
    pub offset: u8,
}

impl<R: Read> ReadType<'_, R> for SameFrame {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(_reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self { offset: context.ty })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SameFrameExtended {
    pub offset: u16,
}

impl<R: Read> ReadType<'_, R> for SameFrameExtended {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            offset: reader.read_u16("same_frame_extended")?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SameLocalsOneStackItemFrame {
    pub offset: u8,
    pub stack_item: VerificationType,
}

impl<R: Read> ReadType<'_, R> for SameLocalsOneStackItemFrame {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            offset: context.ty - 64,
            stack_item: VerificationType::read(reader, context)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SameLocalsOneStackItemFrameExtended {
    pub offset: u16,
    pub stack_item: VerificationType,
}

impl<R: Read> ReadType<'_, R> for SameLocalsOneStackItemFrameExtended {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            offset: reader.read_u16("same_locals_one_stack_item_frame_extended")?,
            stack_item: VerificationType::read(reader, context)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ChopFrame {
    pub offset: u16,
    pub absent_locals: u8,
}

impl<R: Read> ReadType<'_, R> for ChopFrame {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            offset: reader.read_u16("chop_frame")?,
            absent_locals: (251 - context.ty),
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AppendFrame {
    pub offset: u16,
    pub new_locals: Vec<VerificationType>,
}

impl<R: Read> ReadType<'_, R> for AppendFrame {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            offset: reader.read_u16("append_frame")?,
            new_locals: reader.read_many(
                |reader| reader.read_u16("num_locals"),
                |reader| VerificationType::read(reader, context),
            )?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FullFrame {
    pub offset: u16,
    pub locals: Vec<VerificationType>,
    pub stack_items: Vec<VerificationType>,
}

impl<R: Read> ReadType<'_, R> for FullFrame {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(reader: &mut Reader<'_, R>, context: &Self::Context) -> Result<Self::Output> {
        Ok(Self {
            offset: reader.read_u16("full_frame")?,
            locals: reader.read_many(
                |reader| reader.read_u16("num_locals"),
                |reader| VerificationType::read(reader, context),
            )?,
            stack_items: reader.read_many(
                |reader| reader.read_u16("num_stack_items"),
                |reader| VerificationType::read(reader, context),
            )?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum StackMapFrame {
    SameFrame(SameFrame),
    SameFrameExtended(SameFrameExtended),
    SameLocalsOneStackItemFrame(SameLocalsOneStackItemFrame),
    SameLocalsOneStackItemFrameExtended(SameLocalsOneStackItemFrameExtended),
    ChopFrame(ChopFrame),
    AppendFrame(AppendFrame),
    FullFrame(FullFrame),
}

impl<'a, R: Read> ReadType<'a, R> for StackMapFrame {
    type Output = Self;
    type Context = ReadIndexContext<'a>;
    fn read(reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        let ty = reader.read_u8("stack_map_frame type")?;
        let ctx = ReadTypeContext { ty };

        match ty {
            0...63 => {
                //
                SameFrame::read(reader, &ctx).map(StackMapFrame::SameFrame)
            }
            64...127 => {
                //
                SameLocalsOneStackItemFrame::read(reader, &ctx)
                    .map(StackMapFrame::SameLocalsOneStackItemFrame)
            }
            247 => {
                //
                SameLocalsOneStackItemFrameExtended::read(reader, &ctx)
                    .map(StackMapFrame::SameLocalsOneStackItemFrameExtended)
            }
            248...250 => {
                //
                ChopFrame::read(reader, &ctx).map(StackMapFrame::ChopFrame)
            }
            251 => {
                //
                SameFrameExtended::read(reader, &ctx).map(StackMapFrame::SameFrameExtended)
            }
            252...254 => {
                //
                AppendFrame::read(reader, &ctx).map(StackMapFrame::AppendFrame)
            }
            255 => {
                //
                FullFrame::read(reader, &ctx).map(StackMapFrame::FullFrame)
            }
            _ => Err(Error::InvalidStackFrameType(ty)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ReadTypeContext {
    ty: u8,
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

impl<R: Read> ReadType<'_, R> for VerificationType {
    type Output = Self;
    type Context = ReadTypeContext;
    fn read(reader: &mut Reader<'_, R>, _context: &Self::Context) -> Result<Self::Output> {
        use VerificationType::*;
        match reader.read_u8("verification_type")? {
            0 => Ok(Top),
            1 => Ok(Integer),
            2 => Ok(Float),
            3 => Ok(Long),
            4 => Ok(Double),
            5 => Ok(Null),
            6 => Ok(UninitializedThis),
            7 => ConstantIndex::read(reader, &NullContext).map(Object),
            8 => reader.read_u16("uninitialized").map(Uninitialized),
            e => Err(Error::InvalidVerificationType(e)),
        }
    }
}
