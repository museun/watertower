#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::rc::Rc;

use super::*;

#[derive(Debug, Copy, Clone)]
enum LocalVariable {
    None,
    Null,
    Integer(i64),
}

#[derive(Debug, Copy, Clone)]
enum StackValue {
    None,
    Null,
    Integer(i64),
}

macro_rules! stack_value_integer {
    ($($ty:ty)*) => {
        $(
            impl From<$ty> for StackValue {
                fn from(d: $ty) -> Self {
                    StackValue::Integer(d as i64)
                }
            }
        )*
    };
}

stack_value_integer! {
    u8 u16 u32 u64 usize
    i8 i16 i32 i64 isize
}

#[derive(Debug, Clone)]
struct StackFrame {
    local_variables: Vec<LocalVariable>,
    stack: Vec<StackValue>,
}

impl StackFrame {
    fn create(vars: impl Into<usize>, size: impl Into<usize>) -> Self {
        StackFrame {
            local_variables: vec![LocalVariable::None; vars.into()],
            stack: Vec::with_capacity(size.into()),
        }
    }

    fn for_method(method: &ty::Method, mut args: Vec<LocalVariable>) -> Self {
        let code = method.get_code().unwrap();
        let mut stackframe = Self::create(code.max_locals, code.max_stack);
        for (i, arg) in args.drain(..).enumerate() {
            stackframe.local_variables[i] = arg
        }
        stackframe
    }

    // fn trace() {}

    fn get_variable(&self, index: usize) -> Option<&LocalVariable> {
        self.local_variables.get(index)
    }

    fn get_variable_mut(&mut self, index: usize) -> Option<&mut LocalVariable> {
        self.local_variables.get_mut(index)
    }

    fn set_variable(&mut self, index: usize, var: impl Into<LocalVariable>) {
        if let Some(local_variables) = self.get_variable_mut(index) {
            *local_variables = var.into();
            return;
        }
        // TODO make a linked list for this
        self.local_variables.insert(index, var.into());
    }

    fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    fn push(&mut self, value: impl Into<StackValue>) {
        self.stack.push(value.into())
    }
}

#[derive(Debug, Clone)]
struct Context {
    return_value: Option<StackValue>,
    class: Rc<ty::ClassFile>,
}

struct InstructionIter<'a> {
    code: Option<&'a [u8]>,
    pos: usize,
}

impl ty::Method {
    fn instructions(&self) -> InstructionIter<'_> {
        InstructionIter::from_method(self)
    }
}

impl<'a> Iterator for InstructionIter<'a> {
    type Item = Instruction;
    fn next(&mut self) -> Option<Self::Item> {
        let op = self.code?.get(self.pos).cloned()?;
        self.pos += 1;
        Instruction::lookup(op)
    }
}

impl<'a> InstructionIter<'a> {
    fn peek(&self, offset: usize) -> Option<&'a [u8]> {
        self.code?.get(self.pos..self.pos + offset)
    }

    fn advance(&mut self, offset: usize) {
        self.pos = offset;
    }

    fn goto(&mut self, abs: usize) {
        self.pos = abs;
    }

    fn from_method(method: &'a ty::Method) -> Self {
        match method.get_code() {
            Some(s) => Self {
                code: Some(s.code.as_slice()),
                pos: 0,
            },
            None => Self { code: None, pos: 0 },
        }
    }
}

#[derive(Debug, Default)]
pub struct Interpreter {
    main_class: String,
    class_index_map: HashMap<String, HashMap<usize, String>>,
    classes: HashMap<String, Rc<ty::ClassFile>>,
    // class_path
}

impl Interpreter {
    pub fn load_class(&mut self, class: Rc<ty::ClassFile>) {
        let map = Self::build_class_map(&class);
        let name = class.get_class_name();
        self.class_index_map.insert(name.to_string(), map);
        self.classes.insert(name.to_string(), Rc::clone(&class));
    }

    pub fn load_class_from_reader<'a, R, Reader>(&mut self, reader: Reader) -> Result<()>
    where
        R: std::io::Read + 'a,
        Reader: Into<crate::parse::Reader<'a, R>>,
    {
        ty::ClassFile::read(reader)
            .map_err(Into::into)
            .map(Rc::new)
            .map(|class| self.load_class(class))
    }

    pub fn run(mut self) -> Result<()> {
        // TODO make this work properly
        self.main_class = "hello".into();

        let class = &self
            .classes
            .get(&self.main_class)
            .ok_or_else(|| Error::MissingMainClass)
            .map(Rc::clone)?;

        let method = class
            .methods
            .iter()
            .find(|method| method.name() == "main") // TODO check arity (and args (and type))
            .ok_or_else(|| Error::MissingEntryPoint)?;

        if let Some(val) = self.run_method(method, Rc::clone(&class), vec![])? {
            eprintln!(">> {:?}", val)
        }
        Ok(())
    }

    fn build_class_map(class: &ty::ClassFile) -> HashMap<usize, String> {
        class
            .constant_pool
            .iter()
            .filter_map(|k| match k {
                ty::Constant::MethodRef(ty::constant::MethodRef { class, .. }) => Some(class),
                _ => None,
            })
            .filter_map(|index| match index.lookup(&class.constant_pool) {
                Ok(&ty::Constant::ClassRef(class)) => Some((index, class)),
                _ => None,
            })
            .filter_map(|(index, name)| match name.lookup(&class.constant_pool) {
                Ok(ty::Constant::Utf8(s)) => Some((index.0, s)),
                _ => None,
            })
            .map(|(index, name)| (usize::from(index), name.to_string()))
            .collect()
    }

    fn run_method(
        &mut self,
        method: &ty::Method,
        class: Rc<ty::ClassFile>,
        args: Vec<LocalVariable>,
    ) -> Result<Option<StackValue>> {
        let mut stack_frame = StackFrame::for_method(method, args);
        let mut return_value: Option<StackValue> = None;
        let mut pc = 0;
        let mut instructions = method.instructions();

        let mut context = Context {
            return_value: None,
            class: Rc::clone(&class),
        };

        while let Some(instruction) = instructions.next() {
            match self.execute(&instruction, &mut stack_frame, &mut context)? {
                State::Continue => {}
                State::GotoAbsolute(offset) => {}
                State::GotoRelative(offset) => {}
                State::Return(val) => {}
            }
        }

        unimplemented!()
    }

    fn execute(
        &mut self,
        instruction: &Instruction,
        mut stack_frame: &mut StackFrame,
        context: &mut Context,
    ) -> Result<State> {
        match instruction {
            Instruction::NOP(..) => return Ok(State::Continue),
            Instruction::ACONST_NULL(..) => stack_frame.push(StackValue::Null),
            //
            Instruction::ICONST_M1(..) => stack_frame.push(-1),
            Instruction::ICONST_0(..) => stack_frame.push(0),
            Instruction::ICONST_1(..) => stack_frame.push(1),
            Instruction::ICONST_2(..) => stack_frame.push(2),
            Instruction::ICONST_3(..) => stack_frame.push(3),
            Instruction::ICONST_4(..) => stack_frame.push(4),
            Instruction::ICONST_5(..) => stack_frame.push(5),
            //
            Instruction::BIPUSH(BIPUSH(d)) => stack_frame.push(*d),
            Instruction::SIPUSH(SIPUSH(d, ..)) => stack_frame.push(*d),
            //
            Instruction::ILOAD(ILOAD(offset)) => {
                Self::exec_iload(&mut stack_frame, usize::from(*offset))?
            }
            Instruction::ILOAD_0(..) => Self::exec_iload(&mut stack_frame, 0)?,
            Instruction::ILOAD_1(..) => Self::exec_iload(&mut stack_frame, 1)?,
            Instruction::ILOAD_2(..) => Self::exec_iload(&mut stack_frame, 2)?,
            Instruction::ILOAD_3(..) => Self::exec_iload(&mut stack_frame, 3)?,
            //
            Instruction::ISTORE(ISTORE(offset)) => {
                Self::exec_istore(&mut stack_frame, usize::from(*offset))?
            }
            Instruction::ISTORE_0(..) => Self::exec_istore(&mut stack_frame, 0)?,
            Instruction::ISTORE_1(..) => Self::exec_istore(&mut stack_frame, 1)?,
            Instruction::ISTORE_2(..) => Self::exec_istore(&mut stack_frame, 2)?,
            Instruction::ISTORE_3(..) => Self::exec_istore(&mut stack_frame, 3)?,
            //
            Instruction::IADD(..) => match (stack_frame.pop(), stack_frame.pop()) {
                (Some(StackValue::Integer(lhs)), Some(StackValue::Integer(rhs))) => {
                    stack_frame.push(StackValue::Integer(lhs + rhs)) // TODO check for overflow
                }
                (Some(..), Some(..)) => return Err(Error::StackType("integer")),
                (None, None) | (Some(..), None) => return Err(Error::EmptyStack),
                _ => generic_error!("invalid state for IADD"), // TODO this should be a panic
            },
            //
            Instruction::IINC(IINC(offset, _value)) => {
                let offset = usize::from(*offset);
                match stack_frame.get_variable_mut(offset) {
                    Some(LocalVariable::Integer(val)) => *val = *val + 1,
                    Some(..) => return Err(Error::VariableType("integer", offset)),
                    None => return Err(Error::VariableOutOfScope),
                }
            }
            //
            Instruction::GOTO(GOTO(offset, _)) => {
                return Ok(State::GotoRelative(usize::from(*offset)))
            }
            Instruction::RETURN(..) => return Ok(State::Return(StackValue::None)),
            Instruction::IRETURN(..) => {
                return match stack_frame.pop() {
                    Some(StackValue::Integer(ret)) => Ok(State::Return(StackValue::Integer(ret))),
                    Some(..) => Err(Error::StackType("integer")),
                    None => Err(Error::EmptyStack),
                }
            }
            //
            Instruction::GETSTATIC(GETSTATIC(a, b)) => {}
            Instruction::INVOKEVIRTUAL(INVOKEVIRTUAL(a, b)) => {}
            //
            Instruction::LDC(LDC(offset)) => {}
            e => eprintln!("unhandled instruction: {}", e),
        }

        Ok(State::Continue)
    }

    fn exec_iload(stack_frame: &mut StackFrame, offset: usize) -> Result<()> {
        let val = match stack_frame.get_variable(offset) {
            Some(LocalVariable::Integer(val)) => *val,
            Some(LocalVariable::None) => {
                generic_error!("local variable at index {} is not defined", offset);
            }
            Some(..) => generic_error!("local variable at index {} is not an integer", offset),
            None => generic_error!("local variable at index {} is out of range", offset),
        };
        stack_frame.push(StackValue::Integer(val));
        Ok(())
    }

    fn exec_istore(stack_frame: &mut StackFrame, offset: usize) -> Result<()> {
        match stack_frame.pop() {
            Some(StackValue::Integer(val)) => {
                stack_frame.set_variable(offset, LocalVariable::Integer(val));
                Ok(())
            }
            _ => Err(Error::GenericError(format!(
                "stack value at index {} is not an integer",
                offset
            ))),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum State {
    Continue,
    GotoAbsolute(usize),
    GotoRelative(usize),
    Return(StackValue),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn something() {
        let fi = std::fs::read("./etc/hello.class").unwrap();

        let mut interpreter = Interpreter::default();
        interpreter
            .load_class_from_reader(&mut fi.as_slice())
            .unwrap();

        for method in &interpreter.classes["hello"].methods {
            for d in &method.get_code().unwrap().code {
                let inst = Instruction::lookup(*d).unwrap();
                eprintln!("{:02X} -> {}", d, inst);
                eprintln!("  {}", wrap_line(inst.description(), 30));
            }
        }

        // interpreter.run().unwrap();
    }

    enum Line<'a> {
        Single(&'a str),
        Many(Vec<&'a str>),
    }

    impl<'a> std::fmt::Display for Line<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Line::Single(s) => write!(f, "{}", s),
                Line::Many(list) => {
                    for el in list {
                        writeln!(f, "{}", el)?;
                    }
                    Ok(())
                }
            }
        }
    }

    fn wrap_line(mut s: &str, max: usize) -> Line<'_> {
        if s.len() <= max {
            return Line::Single(s);
        }
        let mut parts = vec![];

        let mut s = &mut s;
        for i in 0..(s.len() % max) - 1 {
            let t = &s[i * s.len()..];
            parts.push(&t[..std::cmp::min(s.len(), max)]);
            *s = t;
        }

        Line::Many(parts)
    }
}
