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
pub mod constant;

mod field;
mod method;

use constant::{Extract, Lookup};
use types::*;

pub mod types {
    #[doc(inline)]
    pub use super::attribute::{self, Attribute};
    pub use super::class::{ClassFile, ClassFlags, InnerClassFlags, InnerClassInfo};
    #[doc(inline)]
    pub use super::constant::{self, Constant, ConstantIndex};
    pub use super::field::{Field, FieldFlags};
    pub use super::method::{Method, MethodFlags, MethodHandle, MethodIndex};
}
