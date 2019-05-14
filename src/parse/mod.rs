mod class;
mod error;
mod reader;

pub use error::Error;
pub use reader::Reader;

use bitflags::bitflags;
use error::Result;
use reader::{ReadType, ReadTypeContext};
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
        Attribute, BootstrapMethods, ElementValue, ExceptionTableRow, StackMapFrame,
        VerificationType,
    };
    pub use super::class::{ClassFile, ClassFlags, InnerClassFlags, InnerClassInfo};
    pub use super::constant::{Constant, ConstantIndex};
    pub use super::field::{Field, FieldFlags};
    pub use super::method::{Method, MethodFlags, MethodHandle, MethodIndex};
    pub use super::variable::{LocalVariable, LocalVariableType};
}
