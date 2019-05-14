mod class;
mod error;
mod reader;

pub use error::Error;
pub use reader::Reader;

use bitflags::bitflags;
use error::Result;
use reader::{ReadType, ReadTypeContext, ReadTypeContextIndexed};
use std::io::Read;

mod annotation;
mod attribute;
mod constant;
mod field;
mod method;
mod variable;

use types::*;

pub mod types {
    pub use super::annotation::{Annotation, ParameterAnnotation};
    pub use super::attribute::{
        AnnotationDefault, Attribute, BootstrapMethods, Code, ConstantValue, Deprecated,
        ElementValue, EnclosingMethod, ExceptionTableRow, Exceptions, InnerClasses,
        LineNumberTable, LocalVariableTable, LocalVariableTypeTable, MethodParameters,
        RuntimeInvisibleAnnotations, RuntimeInvisibleParameterAnnotations,
        RuntimeInvisibleTypeAnnotations, RuntimeVisibleAnnotations,
        RuntimeVisibleParameterAnnotations, RuntimeVisibleTypeAnnotations, Signature,
        SourceDebugExtension, SourceFile, StackMapFrame, StackMapTable, Synthetic,
        VerificationType,
    };
    pub use super::class::{ClassFile, ClassFlags, InnerClassFlags, InnerClassInfo};
    pub use super::constant::{Constant, ConstantIndex};
    pub use super::field::{Field, FieldFlags};
    pub use super::method::{Method, MethodFlags, MethodHandle, MethodIndex};
    pub use super::variable::{LocalVariable, LocalVariableType};
}
