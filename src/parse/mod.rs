mod class;
mod error;
mod reader;

pub use error::Error;
pub use reader::Reader;

use bitflags::bitflags;
use error::Result;
use reader::{NullContext, ReadContext, ReadType};
use std::io::Read;

pub mod attribute;

mod constant;
mod field;
mod method;

use types::*;

pub mod types {
    pub use super::attribute::{self, Attribute};
    pub use super::class::{ClassFile, ClassFlags, InnerClassFlags, InnerClassInfo};
    pub use super::constant::{Constant, ConstantIndex};
    pub use super::field::{Field, FieldFlags};
    pub use super::method::{Method, MethodFlags, MethodHandle, MethodIndex};
}
