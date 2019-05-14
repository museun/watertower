mod classfile;
mod error;
mod reader;

pub use error::Error;
pub use reader::Reader;

use bitflags::bitflags;
use error::Result;
use std::io::Read;

mod annotation;
mod attribute;
mod bootstrap_methods;
mod class_flags;
mod constant;
mod constant_index;
mod element_value;
mod exception_table_row;
mod field;
mod field_flags;
mod inner_class_flags;
mod inner_class_info;
mod local_variable;
mod local_variable_type;
mod method;
mod method_flags;
mod method_handle;
mod method_index;
mod parameter_annotation;
mod stack_map_frame;
mod verification_type;

use types::*;

pub mod types {
    pub use super::annotation::Annotation;
    pub use super::attribute::Attribute;
    pub use super::bootstrap_methods::BootstrapMethods;
    pub use super::class_flags::ClassFlags;
    pub use super::classfile::ClassFile;
    pub use super::constant::Constant;
    pub use super::constant_index::ConstantIndex;
    pub use super::element_value::ElementValue;
    pub use super::exception_table_row::ExceptionTableRow;
    pub use super::field::Field;
    pub use super::field_flags::FieldFlags;
    pub use super::inner_class_flags::InnerClassFlags;
    pub use super::inner_class_info::InnerClassInfo;
    pub use super::local_variable::LocalVariable;
    pub use super::local_variable_type::LocalVariableType;
    pub use super::method::Method;
    pub use super::method_flags::MethodFlags;
    pub use super::method_handle::MethodHandle;
    pub use super::method_index::MethodIndex;
    pub use super::parameter_annotation::ParameterAnnotation;
    pub use super::stack_map_frame::StackMapFrame;
    pub use super::verification_type::VerificationType;
}
