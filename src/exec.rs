#![allow(dead_code)]

#[macro_use]
pub mod error;

pub mod cache;
pub mod instructions;
pub mod interpreter;
pub mod value;

use crate::parse::types as ty;
use crate::parse::types::attribute as attr;

#[allow(unused_imports)]
use cache::*;

#[allow(unused_imports)]
use error::*;

use instructions::*;

#[doc(inline)]
pub use instructions::Instruction;

//use value::*;
