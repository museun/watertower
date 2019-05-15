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
        let class = self
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
        stack_frame: &mut StackFrame,
        context: &mut Context,
    ) -> Result<State> {
        Ok(State::Continue)
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

        eprintln!("{:#?}", interpreter);
    }
}
