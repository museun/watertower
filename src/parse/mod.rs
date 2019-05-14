mod class;
mod error;
mod reader;

pub use error::Error;
pub use reader::Reader;

use bitflags::bitflags;
use error::Result;
use reader::{ReadType, ReadTypeContext, ReadTypeContextIndexed};
use std::io::Read;

mod attribute;
mod constant;
mod field;
mod method;

use types::*;

pub mod types {
    pub use super::attribute::{
        Annotation, AnnotationDefault, Attribute, BootstrapMethods, Code, ConstantValue,
        Deprecated, ElementValue, EnclosingMethod, ExceptionTableRow, Exceptions, InnerClasses,
        LineNumberTable, LocalVariable, LocalVariableTable, LocalVariableType,
        LocalVariableTypeTable, MethodParameters, ParameterAnnotation, RuntimeInvisibleAnnotations,
        RuntimeInvisibleParameterAnnotations, RuntimeInvisibleTypeAnnotations,
        RuntimeVisibleAnnotations, RuntimeVisibleParameterAnnotations,
        RuntimeVisibleTypeAnnotations, Signature, SourceDebugExtension, SourceFile, StackMapFrame,
        StackMapTable, Synthetic, VerificationType,
    };
    pub use super::class::{ClassFile, ClassFlags, InnerClassFlags, InnerClassInfo};
    pub use super::constant::{Constant, ConstantIndex};
    pub use super::field::{Field, FieldFlags};
    pub use super::method::{Method, MethodFlags, MethodHandle, MethodIndex};
}
