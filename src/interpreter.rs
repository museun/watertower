#![allow(dead_code)]
type Result<T> = std::result::Result<T, Error>;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::parse::types as ty;
use crate::parse::types::attribute as attr;

#[derive(Debug)]
pub enum Error {
    Parse(crate::parse::Error),
}

impl From<crate::parse::Error> for Error {
    fn from(err: crate::parse::Error) -> Self {
        Error::Parse(err)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(err) => Some(err),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {}

trait Lookup<T: Clone> {
    fn lookup(&self, index: usize) -> Option<&T>;
}

struct Cache<T> {
    map: RefCell<HashMap<usize, Rc<T>>>,
    lookup: Box<dyn Lookup<T>>,
}

impl<T> Cache<T>
where
    T: Clone,
{
    pub fn new(lookup: Box<dyn Lookup<T>>) -> Self {
        Self {
            map: RefCell::new(HashMap::default()),
            lookup,
        }
    }

    pub fn get(&self, index: usize) -> Rc<T> {
        let mut map = self.map.borrow_mut();

        if let Some(code) = map.get(&index).map(Rc::clone) {
            return code;
        }

        if let Some(method) = self.lookup.lookup(index).map(Clone::clone).map(Rc::new) {
            map.insert(index, Rc::clone(&method));
            return method;
        }

        unimplemented!("what to do here?")
    }
}

struct CodeLookup(Rc<ty::ClassFile>);

impl Lookup<attr::Code> for CodeLookup {
    fn lookup(&self, index: usize) -> Option<&attr::Code> {
        self.0.methods.get(index).and_then(ty::Method::get_code)
    }
}

pub struct Interpreter {
    classfile: Rc<ty::ClassFile>,
    pc: usize,
    stack: Vec<Value>,
    sp: usize,

    offset: usize,
    code_cache: Cache<attr::Code>,
}

impl Interpreter {
    pub fn new<'a, R, I>(reader: I) -> Result<Self>
    where
        R: std::io::Read + 'a,
        I: Into<crate::parse::Reader<'a, R>>,
    {
        let out = std::io::stdout();
        let mut out = out.lock();
        ty::ClassFile::read(reader)
            .map(|classfile| {
                classfile.constant_pool.iter().for_each(|constant| {
                    constant
                        .dump(0, &mut out, &classfile.constant_pool)
                        .expect("write")
                });

                let classfile = Rc::new(classfile);
                Self {
                    classfile: Rc::clone(&classfile),
                    code_cache: Cache::new(Box::new(CodeLookup(Rc::clone(&classfile)))),
                    offset: 0,
                    pc: 0,
                    sp: 0,
                    stack: vec![],
                }
            })
            .map_err(Into::into)
    }

    pub fn run(self) {}

    fn push(&mut self, val: impl Into<Value>) {
        self.sp += 1;
        self.stack.push(val.into());
    }

    fn pop(&mut self) -> Value {
        self.sp -= 1;
        self.stack.pop().expect("stack must not be empty")
    }

    fn step(&mut self) {
        let code = self.code_cache.get(self.offset);
        for code in &code.code {
            self.execute(*code);
        }
        self.offset += 1;
    }

    fn execute(&mut self, op: u8) {
        use Instruction::*;
        match Instruction::lookup(op) {
            Some(inst) => match (inst.arity(), inst) {
                (d, AALOAD) => unimplemented!(),
                (d, AASTORE) => unimplemented!(),
                (d, ACONST_NULL) => unimplemented!(),
                (d, ALOAD) => unimplemented!(),
                (d, ALOAD_0) => unimplemented!(),
                (d, ALOAD_1) => unimplemented!(),
                (d, ALOAD_2) => unimplemented!(),
                (d, ALOAD_3) => unimplemented!(),
                (d, ANEWARRAY) => unimplemented!(),
                (d, ARETURN) => unimplemented!(),
                (d, ARRAYLENGTH) => unimplemented!(),
                (d, ASTORE) => unimplemented!(),
                (d, ASTORE_0) => unimplemented!(),
                (d, ASTORE_1) => unimplemented!(),
                (d, ASTORE_2) => unimplemented!(),
                (d, ASTORE_3) => unimplemented!(),
                (d, ATHROW) => unimplemented!(),
                (d, BALOAD) => unimplemented!(),
                (d, BASTORE) => unimplemented!(),
                (d, BIPUSH) => unimplemented!(),
                (d, BREAKPOINT) => unimplemented!(),
                (d, CALOAD) => unimplemented!(),
                (d, CASTORE) => unimplemented!(),
                (d, CHECKCAST) => unimplemented!(),
                (d, D2F) => unimplemented!(),
                (d, D2I) => unimplemented!(),
                (d, D2L) => unimplemented!(),
                (d, DADD) => unimplemented!(),
                (d, DALOAD) => unimplemented!(),
                (d, DASTORE) => unimplemented!(),
                (d, DCMPG) => unimplemented!(),
                (d, DCMPL) => unimplemented!(),
                (d, DCONST_0) => unimplemented!(),
                (d, DCONST_1) => unimplemented!(),
                (d, DDIV) => unimplemented!(),
                (d, DLOAD) => unimplemented!(),
                (d, DLOAD_0) => unimplemented!(),
                (d, DLOAD_1) => unimplemented!(),
                (d, DLOAD_2) => unimplemented!(),
                (d, DLOAD_3) => unimplemented!(),
                (d, DMUL) => unimplemented!(),
                (d, DNEG) => unimplemented!(),
                (d, DREM) => unimplemented!(),
                (d, DRETURN) => unimplemented!(),
                (d, DSTORE) => unimplemented!(),
                (d, DSTORE_0) => unimplemented!(),
                (d, DSTORE_1) => unimplemented!(),
                (d, DSTORE_2) => unimplemented!(),
                (d, DSTORE_3) => unimplemented!(),
                (d, DSUB) => unimplemented!(),
                (d, DUP) => unimplemented!(),
                (d, DUP_X1) => unimplemented!(),
                (d, DUP_X2) => unimplemented!(),
                (d, DUP2) => unimplemented!(),
                (d, DUP2_X1) => unimplemented!(),
                (d, DUP2_X2) => unimplemented!(),
                (d, F2D) => unimplemented!(),
                (d, F2I) => unimplemented!(),
                (d, F2L) => unimplemented!(),
                (d, FADD) => unimplemented!(),
                (d, FALOAD) => unimplemented!(),
                (d, FASTORE) => unimplemented!(),
                (d, FCMPG) => unimplemented!(),
                (d, FCMPL) => unimplemented!(),
                (d, FCONST_0) => unimplemented!(),
                (d, FCONST_1) => unimplemented!(),
                (d, FCONST_2) => unimplemented!(),
                (d, FDIV) => unimplemented!(),
                (d, FLOAD) => unimplemented!(),
                (d, FLOAD_0) => unimplemented!(),
                (d, FLOAD_1) => unimplemented!(),
                (d, FLOAD_2) => unimplemented!(),
                (d, FLOAD_3) => unimplemented!(),
                (d, FMUL) => unimplemented!(),
                (d, FNEG) => unimplemented!(),
                (d, FREM) => unimplemented!(),
                (d, FRETURN) => unimplemented!(),
                (d, FSTORE) => unimplemented!(),
                (d, FSTORE_0) => unimplemented!(),
                (d, FSTORE_1) => unimplemented!(),
                (d, FSTORE_2) => unimplemented!(),
                (d, FSTORE_3) => unimplemented!(),
                (d, FSUB) => unimplemented!(),
                (d, GETFIELD) => unimplemented!(),
                (d, GETSTATIC) => unimplemented!(),
                (d, GOTO) => unimplemented!(),
                (d, GOTO_W) => unimplemented!(),
                (d, I2B) => unimplemented!(),
                (d, I2C) => unimplemented!(),
                (d, I2D) => unimplemented!(),
                (d, I2F) => unimplemented!(),
                (d, I2L) => unimplemented!(),
                (d, I2S) => unimplemented!(),
                (d, IADD) => unimplemented!(),
                (d, IALOAD) => unimplemented!(),
                (d, IAND) => unimplemented!(),
                (d, IASTORE) => unimplemented!(),
                (d, ICONST_M1) => unimplemented!(),
                (d, ICONST_0) => unimplemented!(),
                (d, ICONST_1) => unimplemented!(),
                (d, ICONST_2) => unimplemented!(),
                (d, ICONST_3) => unimplemented!(),
                (d, ICONST_4) => unimplemented!(),
                (d, ICONST_5) => unimplemented!(),
                (d, IDIV) => unimplemented!(),
                (d, IF_ACMPEQ) => unimplemented!(),
                (d, IF_ACMPNE) => unimplemented!(),
                (d, IF_ICMPEQ) => unimplemented!(),
                (d, IF_ICMPGE) => unimplemented!(),
                (d, IF_ICMPGT) => unimplemented!(),
                (d, IF_ICMPLE) => unimplemented!(),
                (d, IF_ICMPLT) => unimplemented!(),
                (d, IF_ICMPNE) => unimplemented!(),
                (d, IFEQ) => unimplemented!(),
                (d, IFGE) => unimplemented!(),
                (d, IFGT) => unimplemented!(),
                (d, IFLE) => unimplemented!(),
                (d, IFLT) => unimplemented!(),
                (d, IFNE) => unimplemented!(),
                (d, IFNONNULL) => unimplemented!(),
                (d, IFNULL) => unimplemented!(),
                (d, IINC) => unimplemented!(),
                (d, ILOAD) => unimplemented!(),
                (d, ILOAD_0) => unimplemented!(),
                (d, ILOAD_1) => unimplemented!(),
                (d, ILOAD_2) => unimplemented!(),
                (d, ILOAD_3) => unimplemented!(),
                (d, IMPDEP1) => unimplemented!(),
                (d, IMPDEP2) => unimplemented!(),
                (d, IMUL) => unimplemented!(),
                (d, INEG) => unimplemented!(),
                (d, INSTANCEOF) => unimplemented!(),
                (d, INVOKEDYNAMIC) => unimplemented!(),
                (d, INVOKEINTERFACE) => unimplemented!(),
                (d, INVOKESPECIAL) => unimplemented!(),
                (d, INVOKESTATIC) => unimplemented!(),
                (d, INVOKEVIRTUAL) => unimplemented!(),
                (d, IOR) => unimplemented!(),
                (d, IREM) => unimplemented!(),
                (d, IRETURN) => unimplemented!(),
                (d, ISHL) => unimplemented!(),
                (d, ISHR) => unimplemented!(),
                (d, ISTORE) => unimplemented!(),
                (d, ISTORE_0) => unimplemented!(),
                (d, ISTORE_1) => unimplemented!(),
                (d, ISTORE_2) => unimplemented!(),
                (d, ISTORE_3) => unimplemented!(),
                (d, ISUB) => unimplemented!(),
                (d, IUSHR) => unimplemented!(),
                (d, IXOR) => unimplemented!(),
                (d, JSR) => unimplemented!(),
                (d, JSR_W) => unimplemented!(),
                (d, L2D) => unimplemented!(),
                (d, L2F) => unimplemented!(),
                (d, L2I) => unimplemented!(),
                (d, LADD) => unimplemented!(),
                (d, LALOAD) => unimplemented!(),
                (d, LAND) => unimplemented!(),
                (d, LASTORE) => unimplemented!(),
                (d, LCMP) => unimplemented!(),
                (d, LCONST_0) => unimplemented!(),
                (d, LCONST_1) => unimplemented!(),
                (d, LDC) => unimplemented!(),
                (d, LDC_W) => unimplemented!(),
                (d, LDC2_W) => unimplemented!(),
                (d, LDIV) => unimplemented!(),
                (d, LLOAD) => unimplemented!(),
                (d, LLOAD_0) => unimplemented!(),
                (d, LLOAD_1) => unimplemented!(),
                (d, LLOAD_2) => unimplemented!(),
                (d, LLOAD_3) => unimplemented!(),
                (d, LMUL) => unimplemented!(),
                (d, LNEG) => unimplemented!(),
                (d, LOOKUPSWITCH) => unimplemented!(),
                (d, LOR) => unimplemented!(),
                (d, LREM) => unimplemented!(),
                (d, LRETURN) => unimplemented!(),
                (d, LSHL) => unimplemented!(),
                (d, LSHR) => unimplemented!(),
                (d, LSTORE) => unimplemented!(),
                (d, LSTORE_0) => unimplemented!(),
                (d, LSTORE_1) => unimplemented!(),
                (d, LSTORE_2) => unimplemented!(),
                (d, LSTORE_3) => unimplemented!(),
                (d, LSUB) => unimplemented!(),
                (d, LUSHR) => unimplemented!(),
                (d, LXOR) => unimplemented!(),
                (d, MONITORENTER) => unimplemented!(),
                (d, MONITOREXIT) => unimplemented!(),
                (d, MULTIANEWARRAY) => unimplemented!(),
                (d, NEW) => unimplemented!(),
                (d, NEWARRAY) => unimplemented!(),
                (d, NOP) => unimplemented!(),
                (d, POP) => unimplemented!(),
                (d, POP2) => unimplemented!(),
                (d, PUTFIELD) => unimplemented!(),
                (d, PUTSTATIC) => unimplemented!(),
                (d, RET) => unimplemented!(),
                (d, RETURN) => unimplemented!(),
                (d, SALOAD) => unimplemented!(),
                (d, SASTORE) => unimplemented!(),
                (d, SIPUSH) => unimplemented!(),
                (d, SWAP) => unimplemented!(),
                (d, TABLESWITCH) => unimplemented!(),
                (d, WIDE) => unimplemented!(),
            },
            _ => unimplemented!("bad instruction: 0x{:02X}", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn something() {
        let fi = std::fs::read("./etc/hello.class").unwrap();
        let mut interpreter = Interpreter::new(&mut fi.as_slice()) //
            .unwrap_or_else(|err| panic!("{}", err));
        interpreter.step();
        interpreter.step();
    }
}

// TODO make this better
macro_rules! instruction {
    ($($opcode:expr => $arity:expr => $inst:ident => $doc:expr);* $(;)?) => {
        instruction!(@ $($opcode => $arity => $inst => $doc);*);
    };

    (@ $($opcode:expr => $arity:expr => $inst:ident => $doc:expr);*) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum Instruction {
            $(#[doc = $doc] $inst,)*
        }
        impl Instruction {
            pub fn lookup(d: u8) -> Option<Self> {
                use Instruction::*;
                match d {
                    $($opcode => Some($inst),)*
                    _ => None
                }
            }

            pub fn arity(&self) -> u8 {
                use Instruction::*;
                match self {
                    $($inst => $arity,)*
                }
            }

            pub fn description(&self) -> &'static str {
                use Instruction::*;
                match self {
                    $($inst => &$doc,)*
                }
            }

            pub fn opcode(&self) -> u8 {
                use Instruction::*;
                match self {
                    $($inst => $opcode,)*
                }
            }
        }

        impl std::fmt::Display for Instruction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use Instruction::*;
                match self {
                    $($inst => write!(f, "{}", stringify!($inst)),)*
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Arity {
    None,
    Fixed(u8),
}

instruction! {
    0x32 => 0 => AALOAD          => "load onto the stack a reference from an array";
    0x53 => 0 => AASTORE         => "store into a reference in an array";
    0x01 => 0 => ACONST_NULL     => "push a null reference onto the stack";
    0x19 => 0 => ALOAD           => "load a reference onto the stack from a local variable #index";
    0x2A => 1 => ALOAD_0         => "load a reference onto the stack from local variable 0";
    0x2B => 0 => ALOAD_1         => "load a reference onto the stack from local variable 1";
    0x2C => 0 => ALOAD_2         => "load a reference onto the stack from local variable 2";
    0x2D => 0 => ALOAD_3         => "load a reference onto the stack from local variable 3";
    0xBD => 2 => ANEWARRAY       => "create a new array of references of length count and component type identified by the class reference index (indexbyte1 << 8 + indexbyte2) in the constant pool";
    0xB0 => 0 => ARETURN         => "return a reference from a method";
    0xBE => 0 => ARRAYLENGTH     => "get the length of an array";
    0x3A => 1 => ASTORE          => "store a reference into a local variable #index";
    0x4B => 0 => ASTORE_0        => "store a reference into local variable 0";
    0x4C => 0 => ASTORE_1        => "store a reference into local variable 1";
    0x4D => 0 => ASTORE_2        => "store a reference into local variable 2";
    0x4E => 0 => ASTORE_3        => "store a reference into local variable 3";
    0xBF => 0 => ATHROW          => "throws an error or exception (notice that the rest of the stack is cleared, leaving only a reference to the Throwable)";
    0x33 => 0 => BALOAD          => "load a byte or Boolean value from an array";
    0x54 => 0 => BASTORE         => "store a byte or Boolean value into an array";
    0x10 => 1 => BIPUSH          => "push a byte onto the stack as an integer value";
    0xCA => 0 => BREAKPOINT      => "reserved for breakpoints in Java debuggers; should not appear in any class file";
    0x34 => 0 => CALOAD          => "load a char from an array";
    0x55 => 0 => CASTORE         => "store a char into an array";
    0xC0 => 2 => CHECKCAST       => "checks whether an objectref is of a certain type, the class reference of which is in the constant pool at index (indexbyte1 << 8 + indexbyte2)";
    0x90 => 0 => D2F             => "convert a double to a float";
    0x8E => 0 => D2I             => "convert a double to an int";
    0x8F => 0 => D2L             => "convert a double to a long";
    0x63 => 0 => DADD            => "add two doubles";
    0x31 => 0 => DALOAD          => "load a double from an array";
    0x52 => 0 => DASTORE         => "store a double into an array";
    0x98 => 0 => DCMPG           => "compare two doubles";
    0x97 => 0 => DCMPL           => "compare two doubles";
    0x0E => 0 => DCONST_0        => "push the constant 0.0 (a double) onto the stack";
    0x0F => 0 => DCONST_1        => "push the constant 1.0 (a double) onto the stack";
    0x6F => 0 => DDIV            => "divide two doubles";
    0x18 => 1 => DLOAD           => "load a double value from a local variable #index";
    0x26 => 0 => DLOAD_0         => "load a double from local variable 0";
    0x27 => 0 => DLOAD_1         => "load a double from local variable 1";
    0x28 => 0 => DLOAD_2         => "load a double from local variable 2";
    0x29 => 0 => DLOAD_3         => "load a double from local variable 3";
    0x6B => 0 => DMUL            => "multiply two doubles";
    0x77 => 0 => DNEG            => "negate a double";
    0x73 => 0 => DREM            => "get the remainder from a division between two doubles";
    0xAF => 0 => DRETURN         => "return a double from a method";
    0x39 => 1 => DSTORE          => "store a double value into a local variable #index";
    0x47 => 0 => DSTORE_0        => "store a double into local variable 0";
    0x48 => 0 => DSTORE_1        => "store a double into local variable 1";
    0x49 => 0 => DSTORE_2        => "store a double into local variable 2";
    0x4A => 0 => DSTORE_3        => "store a double into local variable 3";
    0x67 => 0 => DSUB            => "subtract a double from another";
    0x59 => 0 => DUP             => "duplicate the value on top of the stack";
    0x5A => 0 => DUP_X1          => "insert a copy of the top value into the stack two values from the top. value1 and value2 must not be of the type double or long.";
    0x5B => 0 => DUP_X2          => "insert a copy of the top value into the stack two (if value2 is double or long it takes up the entry of value3, too) or three values (if value2 is neither double nor long) from the top";
    0x5C => 0 => DUP2            => "duplicate top two stack words (two values, if value1 is not double nor long; a single value, if value1 is double or long)";
    0x5D => 0 => DUP2_X1         => "duplicate two words and insert beneath third word (see explanation above)";
    0x5E => 0 => DUP2_X2         => "duplicate two words and insert beneath fourth word";
    0x8D => 0 => F2D             => "convert a float to a double";
    0x8B => 0 => F2I             => "convert a float to an int";
    0x8C => 0 => F2L             => "convert a float to a long";
    0x62 => 0 => FADD            => "add two floats";
    0x30 => 0 => FALOAD          => "load a float from an array";
    0x51 => 0 => FASTORE         => "store a float in an array";
    0x96 => 0 => FCMPG           => "compare two floats";
    0x95 => 0 => FCMPL           => "compare two floats";
    0x0B => 0 => FCONST_0        => "push 0.0f on the stack";
    0x0C => 0 => FCONST_1        => "push 1.0f on the stack";
    0x0D => 0 => FCONST_2        => "push 2.0f on the stack";
    0x6E => 0 => FDIV            => "divide two floats";
    0x17 => 1 => FLOAD           => "load a float value from a local variable #index";
    0x22 => 0 => FLOAD_0         => "load a float value from local variable 0";
    0x23 => 0 => FLOAD_1         => "load a float value from local variable 1";
    0x24 => 0 => FLOAD_2         => "load a float value from local variable 2";
    0x25 => 0 => FLOAD_3         => "load a float value from local variable 3";
    0x6A => 0 => FMUL            => "multiply two floats";
    0x76 => 0 => FNEG            => "negate a float";
    0x72 => 0 => FREM            => "get the remainder from a division between two floats";
    0xAE => 0 => FRETURN         => "return a float";
    0x38 => 1 => FSTORE          => "store a float value into a local variable #index";
    0x43 => 0 => FSTORE_0        => "store a float value into local variable 0";
    0x44 => 0 => FSTORE_1        => "store a float value into local variable 1";
    0x45 => 0 => FSTORE_2        => "store a float value into local variable 2";
    0x46 => 0 => FSTORE_3        => "store a float value into local variable 3";
    0x66 => 0 => FSUB            => "subtract two floats";
    0xB4 => 2 => GETFIELD        => "get a field value of an object objectref, where the field is identified by field reference in the constant pool index (indexbyte1 << 8 + indexbyte2)";
    0xB2 => 2 => GETSTATIC       => "get a static field value of a class, where the field is identified by field reference in the constant pool index (indexbyte1 << 8 + indexbyte2)";
    0xA7 => 2 => GOTO            => "goes to another instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xC8 => 4 => GOTO_W          => "goes to another instruction at branchoffset (signed int constructed from unsigned bytes branchbyte1 << 24 + branchbyte2 << 16 + branchbyte3 << 8 + branchbyte4)";
    0x91 => 0 => I2B             => "convert an int into a byte";
    0x92 => 0 => I2C             => "convert an int into a character";
    0x87 => 0 => I2D             => "convert an int into a double";
    0x86 => 0 => I2F             => "convert an int into a float";
    0x85 => 0 => I2L             => "convert an int into a long";
    0x93 => 0 => I2S             => "convert an int into a short";
    0x60 => 0 => IADD            => "add two ints";
    0x2E => 0 => IALOAD          => "load an int from an array";
    0x7E => 0 => IAND            => "perform a bitwise AND on two integers";
    0x4F => 0 => IASTORE         => "store an int into an array";
    0x02 => 0 => ICONST_M1       => "load the int value −1 onto the stack";
    0x03 => 0 => ICONST_0        => "load the int value 0 onto the stack";
    0x04 => 0 => ICONST_1        => "load the int value 1 onto the stack";
    0x05 => 0 => ICONST_2        => "load the int value 2 onto the stack";
    0x06 => 0 => ICONST_3        => "load the int value 3 onto the stack";
    0x07 => 0 => ICONST_4        => "load the int value 4 onto the stack";
    0x08 => 0 => ICONST_5        => "load the int value 5 onto the stack";
    0x6C => 0 => IDIV            => "divide two integers";
    0xA5 => 2 => IF_ACMPEQ       => "if references are equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA6 => 2 => IF_ACMPNE       => "if references are not equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9F => 2 => IF_ICMPEQ       => "if ints are equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA2 => 2 => IF_ICMPGE       => "if value1 is greater than or equal to value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA3 => 2 => IF_ICMPGT       => "if value1 is greater than value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA4 => 2 => IF_ICMPLE       => "if value1 is less than or equal to value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA1 => 2 => IF_ICMPLT       => "if value1 is less than value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA0 => 2 => IF_ICMPNE       => "if ints are not equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x99 => 2 => IFEQ            => "if value is 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9C => 2 => IFGE            => "if value is greater than or equal to 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9D => 2 => IFGT            => "if value is greater than 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9E => 2 => IFLE            => "if value is less than or equal to 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9B => 2 => IFLT            => "if value is less than 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9A => 2 => IFNE            => "if value is not 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xC7 => 2 => IFNONNULL       => "if value is not null, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xC6 => 2 => IFNULL          => "if value is null, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x84 => 2 => IINC            => "increment local variable #index by signed byte const";
    0x15 => 1 => ILOAD           => "load an int value from a local variable #index";
    0x1A => 0 => ILOAD_0         => "load an int value from local variable 0";
    0x1B => 0 => ILOAD_1         => "load an int value from local variable 1";
    0x1C => 0 => ILOAD_2         => "load an int value from local variable 2";
    0x1D => 0 => ILOAD_3         => "load an int value from local variable 3";
    0xFE => 0 => IMPDEP1         => "reserved for implementation-dependent operations within debuggers; should not appear in any class file";
    0xFF => 0 => IMPDEP2         => "reserved for implementation-dependent operations within debuggers; should not appear in any class file";
    0x68 => 0 => IMUL            => "multiply two integers";
    0x74 => 0 => INEG            => "negate int";
    0xC1 => 2 => INSTANCEOF      => "determines if an object objectref is of a given type, identified by class reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xBA => 4 => INVOKEDYNAMIC   => "invokes a dynamic method and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB9 => 4 => INVOKEINTERFACE => "invokes an interface method on object objectref and puts the result on the stack (might be void); the interface method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB7 => 2 => INVOKESPECIAL   => "invoke instance method on object objectref and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB8 => 2 => INVOKESTATIC    => "invoke a static method and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB6 => 2 => INVOKEVIRTUAL   => "invoke virtual method on object objectref and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0x80 => 0 => IOR             => "bitwise int OR";
    0x70 => 0 => IREM            => "logical int remainder";
    0xAC => 0 => IRETURN         => "return an integer from a method";
    0x78 => 0 => ISHL            => "int shift left";
    0x7A => 0 => ISHR            => "int arithmetic shift right";
    0x36 => 1 => ISTORE          => "store int value into variable #index";
    0x3B => 0 => ISTORE_0        => "store int value into variable 0";
    0x3C => 0 => ISTORE_1        => "store int value into variable 1";
    0x3D => 0 => ISTORE_2        => "store int value into variable 2";
    0x3E => 0 => ISTORE_3        => "store int value into variable 3";
    0x64 => 0 => ISUB            => "int subtract";
    0x7C => 0 => IUSHR           => "int logical shift right";
    0x82 => 0 => IXOR            => "int xor";
    0xA8 => 2 => JSR             => "jump to subroutine at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2) and place the return address on the stack";
    0xC9 => 4 => JSR_W           => "jump to subroutine at branchoffset (signed int constructed from unsigned bytes branchbyte1 << 24 + branchbyte2 << 16 + branchbyte3 << 8 + branchbyte4) and place the return address on the stack";
    0x8A => 0 => L2D             => "convert a long to a double";
    0x89 => 0 => L2F             => "convert a long to a float";
    0x88 => 0 => L2I             => "convert a long to a int";
    0x61 => 0 => LADD            => "add two longs";
    0x2F => 0 => LALOAD          => "load a long from an array";
    0x7F => 0 => LAND            => "bitwise AND of two longs";
    0x50 => 0 => LASTORE         => "store a long to an array";
    0x94 => 0 => LCMP            => "push 0 if the two longs are the same, 1 if value1 is greater than value2, -1 otherwise";
    0x09 => 0 => LCONST_0        => "push 0L (the number zero with type long) onto the stack";
    0x0A => 0 => LCONST_1        => "push 1L (the number one with type long) onto the stack";
    0x12 => 1 => LDC             => "push a constant #index from a constant pool (String, int, float, Class, java.lang.invoke.MethodType, or java.lang.invoke.MethodHandle) onto the stack";
    0x13 => 2 => LDC_W           => "push a constant #index from a constant pool (String, int, float, Class, java.lang.invoke.MethodType, or java.lang.invoke.MethodHandle) onto the stack (wide index is constructed as indexbyte1 << 8 + indexbyte2)";
    0x14 => 2 => LDC2_W          => "push a constant #index from a constant pool (double or long) onto the stack (wide index is constructed as indexbyte1 << 8 + indexbyte2)";
    0x6D => 0 => LDIV            => "divide two longs";
    0x16 => 1 => LLOAD           => "load a long value from a local variable #index";
    0x1E => 0 => LLOAD_0         => "load a long value from a local variable 0";
    0x1F => 0 => LLOAD_1         => "load a long value from a local variable 1";
    0x20 => 0 => LLOAD_2         => "load a long value from a local variable 2";
    0x21 => 0 => LLOAD_3         => "load a long value from a local variable 3";
    0x69 => 0 => LMUL            => "multiply two longs";
    0x75 => 0 => LNEG            => "negate a long";
    0xAB => 8 => LOOKUPSWITCH    => "a target address is looked up from a table using a key and execution continues from the instruction at that address";
    0x81 => 0 => LOR             => "bitwise OR of two longs";
    0x71 => 0 => LREM            => "remainder of division of two longs";
    0xAD => 0 => LRETURN         => "return a long value";
    0x79 => 0 => LSHL            => "bitwise shift left of a long value1 by int value2 positions";
    0x7B => 0 => LSHR            => "bitwise shift right of a long value1 by int value2 positions";
    0x37 => 1 => LSTORE          => "store a long value in a local variable #index";
    0x3F => 0 => LSTORE_0        => "store a long value in a local variable 0";
    0x40 => 0 => LSTORE_1        => "store a long value in a local variable 1";
    0x41 => 0 => LSTORE_2        => "store a long value in a local variable 2";
    0x42 => 0 => LSTORE_3        => "store a long value in a local variable 3";
    0x65 => 0 => LSUB            => "subtract two longs";
    0x7D => 0 => LUSHR           => "bitwise shift right of a long value1 by int value2 positions, unsigned";
    0x83 => 0 => LXOR            => "bitwise XOR of two longs";
    0xC2 => 0 => MONITORENTER    => "enter monitor for object (\"grab the lock\" – start of synchronized() section)";
    0xC3 => 0 => MONITOREXIT     => "exit monitor for object (\"release the lock\" – end of synchronized() section)";
    0xC5 => 3 => MULTIANEWARRAY  => "create a new array of dimensions dimensions of type identified by class reference in constant pool index (indexbyte1 << 8 + indexbyte2); the sizes of each dimension is identified by count1, [count2, etc.]";
    0xBB => 2 => NEW             => "create new object of type identified by class reference in constant pool index (indexbyte1 << 8 + indexbyte2)";
    0xBC => 1 => NEWARRAY        => "create new array with count elements of primitive type identified by atype";
    0x00 => 0 => NOP             => "perform no operation";
    0x57 => 0 => POP             => "discard the top value on the stack";
    0x58 => 0 => POP2            => "discard the top two values on the stack (or one value, if it is a double or long)";
    0xB5 => 2 => PUTFIELD        => "set field to value in an object objectref, where the field is identified by a field reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB3 => 2 => PUTSTATIC       => "set static field to value in a class, where the field is identified by a field reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xA9 => 1 => RET             => "continue execution from address taken from a local variable #index (the asymmetry with jsr is intentional)";
    0xB1 => 0 => RETURN          => "return void from method";
    0x35 => 0 => SALOAD          => "load short from array";
    0x56 => 0 => SASTORE         => "store short to array";
    0x11 => 2 => SIPUSH          => "push a short onto the stack as an integer value";
    0x5F => 0 => SWAP            => "swaps two top words on the stack (note that value1 and value2 must not be double or long)";
    0xAA => 16 => TABLESWITCH    => "continue execution from an address in the table at offset index";
    0xC4 => 3 => WIDE            => "execute opcode, where opcode is either iload, fload, aload, lload, dload, istore, fstore, astore, lstore, dstore, or ret, but assume the index is 16 bit; or execute iinc, where the index is 16 bits and the constant to increment by is a signed 16 bit short ";
}

// (no name) 	cb-fd

// arity tabke
/*
0x19 1: index
0xBD 2: indexbyte1, indexbyte2
0x3A 1: index
0x10 1: byte
0xC0 2: indexbyte1, indexbyte2
0x18 1: index
0x39 1: index
0x17 1: index
0x38 1: index
0xB4 2: indexbyte1, indexbyte2
0xB2 2: indexbyte1, indexbyte2
0xA7 2: branchbyte1, branchbyte2
0xC8 4: branchbyte1, branchbyte2, branchbyte3, branchbyte4
0xA5 2: branchbyte1, branchbyte2
0xA6 2: branchbyte1, branchbyte2
0x9F 2: branchbyte1, branchbyte2
0xA2 2: branchbyte1, branchbyte2
0xA3 2: branchbyte1, branchbyte2
0xA4 2: branchbyte1, branchbyte2
0xA1 2: branchbyte1, branchbyte2
0xA0 2: branchbyte1, branchbyte2
0x99 2: branchbyte1, branchbyte2
0x9C 2: branchbyte1, branchbyte2
0x9D 2: branchbyte1, branchbyte2
0x9E 2: branchbyte1, branchbyte2
0x9B 2: branchbyte1, branchbyte2
0x9A 2: branchbyte1, branchbyte2
0xC7 2: branchbyte1, branchbyte2
0xC6 2: branchbyte1, branchbyte2
0x84 2: index, const
0x15 1: index
0xC1 2: indexbyte1, indexbyte2
0xBA 4: indexbyte1, indexbyte2, 0, 0
0xB9 4: indexbyte1, indexbyte2, count, 0
0xB7 2: indexbyte1, indexbyte2
0xB8 2: indexbyte1, indexbyte2
0xB6 2: indexbyte1, indexbyte2
0x36 1: index
0xA8 2: branchbyte1, branchbyte2
0xC9 4: branchbyte1, branchbyte2, branchbyte3, branchbyte4
0x12 1: index
0x13 2: indexbyte1, indexbyte2
0x14 2: indexbyte1, indexbyte2
0x16 1: index
0xAB 8+: [0–3 bytes padding], defaultbyte1, defaultbyte2, defaultbyte3, defaultbyte4, npairs1, npairs2, npairs3, npairs4, match-offset pairs...
0x37 1: index
0xC5 3: indexbyte1, indexbyte2, dimensions
0xBB 2: indexbyte1, indexbyte2
0xBC 1: atype
0xB5 2: indexbyte1, indexbyte2
0xB3 2: indexbyte1, indexbyte2
0xA9 1: index
0x11 2: byte1, byte2
0xAA 16+: [0–3 bytes padding], defaultbyte1, defaultbyte2, defaultbyte3, defaultbyte4, lowbyte1, lowbyte2, lowbyte3, lowbyte4, highbyte1, highbyte2, highbyte3, highbyte4, jump offsets...
// read the docs on this instruction
0xC4 3: opcode, indexbyte1, indexbyte2
    or
     5: iinc, indexbyte1, indexbyte2, countbyte1, countbyte2
*/
